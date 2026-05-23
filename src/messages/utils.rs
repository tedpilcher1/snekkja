use std::{fmt, ops::RangeInclusive};

#[derive(Clone, Copy)]
pub struct AisStr<const N: usize> {
    buf: [u8; N],
    len: u8,
}

impl<const N: usize> AisStr<N> {
    pub fn as_str(&self) -> &str {
        // Safety: buf contains only printable ASCII from the 6-bit AIS character set
        unsafe { std::str::from_utf8_unchecked(&self.buf[..self.len as usize]) }
    }
}

impl<const N: usize> fmt::Debug for AisStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub fn decode_text_fixed<const N: usize>(
    bytes: &[u8],
    start: usize,
    num_chars: usize,
) -> AisStr<N> {
    let mut buf = [0u8; N];
    let mut trimmed_len = 0usize;

    for (i, slot) in buf.iter_mut().enumerate().take(num_chars) {
        let val = get_bits_dyn(bytes, start + i * 6, 6) as u8;
        let ch = if val < 32 { val + 64 } else { val };
        *slot = ch;
        if ch != b'@' && ch != b' ' {
            trimmed_len = i + 1;
        }
    }

    AisStr {
        buf,
        len: trimmed_len as u8,
    }
}

#[inline(always)]
fn get_bits<const START: usize, const LEN: usize>(bytes: &[u8]) -> u64 {
    // Safety: as_slice() guarantees 7 zero bytes past the payload, so any
    // byte_start < self.len has a full 8 bytes available to read.
    let val =
        unsafe { (bytes.as_ptr().add(const { START / 8 }) as *const u64).read_unaligned() }.to_be();

    (val >> const { 64 - START % 8 - LEN }) & const { (1u64 << LEN) - 1 }
}

/// Reads up to `max_chars` six-bit AIS characters starting at `start_bit`, stopping early at the
/// first zero value (which encodes '@', forbidden in name extensions by the spec)
///
/// Fill bits past the payload are already zeroed by the unarmorer, so this terminates cleanly
/// without needing the original fill-bits count.
pub fn decode_text_dynamic<const N: usize>(
    bytes: &[u8],
    start_bit: usize,
    max_chars: usize,
) -> AisStr<N> {
    let mut buf = [0u8; N];
    let mut len = 0usize;

    for (i, slot) in buf.iter_mut().enumerate().take(max_chars.min(N)) {
        let val = get_bits_dyn(bytes, start_bit + i * 6, 6) as u8;
        if val == 0 {
            break;
        }
        *slot = if val < 32 { val + 64 } else { val };
        len = i + 1;
    }

    AisStr {
        buf,
        len: len as u8,
    }
}

#[inline(always)]
fn get_bits_dyn(bytes: &[u8], start: usize, len: usize) -> u64 {
    let shift = 64 - start % 8 - len;
    let mask = (1u64 << len) - 1;

    // Safety: as_slice() guarantees 7 zero bytes past the payload, so any
    // byte_start < self.len has a full 8 bytes available to read.
    let val = unsafe { (bytes.as_ptr().add(start / 8) as *const u64).read_unaligned() }.to_be();

    (val >> shift) & mask
}

#[inline(always)]
pub fn get_bit<const BIT: usize>(bytes: &[u8]) -> bool {
    get_bits::<BIT, 1>(bytes) != 0
}

#[inline(always)]
pub fn get_bits_u8<const START: usize, const LEN: usize>(bytes: &[u8]) -> u8 {
    get_bits::<START, LEN>(bytes) as u8
}

#[inline(always)]
pub fn get_bits_u16<const START: usize, const LEN: usize>(bytes: &[u8]) -> u16 {
    get_bits::<START, LEN>(bytes) as u16
}

#[inline(always)]
pub fn get_bits_u32<const START: usize, const LEN: usize>(bytes: &[u8]) -> u32 {
    get_bits::<START, LEN>(bytes) as u32
}

#[inline(always)]
pub fn get_bits_i32<const START: usize, const LEN: usize>(bytes: &[u8]) -> i32 {
    let shift = 32 - LEN;
    (get_bits::<START, LEN>(bytes) as i32) << shift >> shift
}

#[inline(always)]
pub fn get_bit_dyn(bytes: &[u8], i: usize) -> bool {
    get_bits_dyn(bytes, i, 1) != 0
}

#[inline(always)]
pub fn get_bits_u8_dyn(bytes: &[u8], range: RangeInclusive<usize>) -> u8 {
    get_bits_dyn(bytes, *range.start(), range.end() - range.start() + 1) as u8
}

#[inline(always)]
pub fn get_bits_u16_dyn(bytes: &[u8], range: RangeInclusive<usize>) -> u16 {
    get_bits_dyn(bytes, *range.start(), range.end() - range.start() + 1) as u16
}
