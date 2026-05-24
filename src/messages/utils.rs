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
        let val = extract_bits_dyn(bytes, start + i * 6, 6) as u8;
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

pub trait FromBits {
    fn from_bits<const LEN: usize>(val: u64) -> Self;
    fn from_bits_dyn(val: u64, len: usize) -> Self;
}

impl FromBits for u8 {
    fn from_bits<const LEN: usize>(val: u64) -> Self {
        val as u8
    }
    fn from_bits_dyn(val: u64, _len: usize) -> Self {
        val as u8
    }
}
impl FromBits for u16 {
    fn from_bits<const LEN: usize>(val: u64) -> Self {
        val as u16
    }
    fn from_bits_dyn(val: u64, _len: usize) -> Self {
        val as u16
    }
}
impl FromBits for u32 {
    fn from_bits<const LEN: usize>(val: u64) -> Self {
        val as u32
    }
    fn from_bits_dyn(val: u64, _len: usize) -> Self {
        val as u32
    }
}
impl FromBits for i32 {
    fn from_bits<const LEN: usize>(val: u64) -> Self {
        let shift = 32 - LEN;
        (val as i32) << shift >> shift
    }
    fn from_bits_dyn(val: u64, len: usize) -> Self {
        let shift = 32 - len;
        (val as i32) << shift >> shift
    }
}

#[inline(always)]
pub fn get_bits<T: FromBits, const START: usize, const LEN: usize>(bytes: &[u8]) -> T {
    // Safety: as_slice() guarantees 7 zero bytes past the payload, so any
    // byte_start < self.len has a full 8 bytes available to read
    let val =
        unsafe { (bytes.as_ptr().add(const { START / 8 }) as *const u64).read_unaligned() }.to_be();

    let extracted = (val >> const { 64 - START % 8 - LEN }) & const { (1u64 << LEN) - 1 };
    T::from_bits::<LEN>(extracted)
}

#[inline(always)]
pub fn get_bits_dyn<T: FromBits>(bytes: &[u8], range: RangeInclusive<usize>) -> T {
    let start = *range.start();
    let len = range.end() - start + 1;
    let extracted = extract_bits_dyn(bytes, start, len);
    T::from_bits_dyn(extracted, len)
}

/// Reads up to `max_chars` six-bit AIS characters starting at `start_bit`, stopping early at the
/// first zero value (which encodes '@', forbidden by spec)
///
/// Fill bits past the payload are already zeroed by the unarmorer, so this terminates cleanly
/// without needing the original fill-bits count
pub fn decode_text_dynamic<const N: usize>(
    bytes: &[u8],
    start_bit: usize,
    max_chars: usize,
) -> AisStr<N> {
    let mut buf = [0u8; N];
    let mut len = 0usize;

    for (i, slot) in buf.iter_mut().enumerate().take(max_chars.min(N)) {
        let val = extract_bits_dyn(bytes, start_bit + i * 6, 6) as u8;
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
fn extract_bits_dyn(bytes: &[u8], start: usize, len: usize) -> u64 {
    let shift = 64 - start % 8 - len;
    let mask = (1u64 << len) - 1;

    // Safety: as_slice() guarantees 7 zero bytes past the payload, so any
    // byte_start < self.len has a full 8 bytes available to read.
    let val = unsafe { (bytes.as_ptr().add(start / 8) as *const u64).read_unaligned() }.to_be();

    (val >> shift) & mask
}

#[inline(always)]
pub fn get_bit<const BIT: usize>(bytes: &[u8]) -> bool {
    get_bits::<u8, BIT, 1>(bytes) != 0
}

#[inline(always)]
pub fn get_bit_dyn(bytes: &[u8], i: usize) -> bool {
    extract_bits_dyn(bytes, i, 1) != 0
}
