use std::{fmt, ops::RangeInclusive};

/// AIS 6-bit value to ascii
///
/// 0–31 map to '@'–'_' (add 64)
/// 32–63 stay as-is
#[rustfmt::skip]
const AIS_CHAR: [u8; 64] = [
    b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'Z', b'[', b'\\',b']', b'^', b'_',
    b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'',
    b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
];

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

#[cfg(target_arch = "aarch64")]
pub fn decode_text_fixed<const N: usize, const START: usize>(bytes: &[u8]) -> AisStr<N> {
    let (buf, len) = unsafe { neon::decode_text::<N, START>(bytes) };
    AisStr {
        buf,
        len: len as u8,
    }
}

#[cfg(not(target_arch = "aarch64"))]
pub fn decode_text_fixed<const N: usize, const START: usize>(bytes: &[u8]) -> AisStr<N> {
    let mut buf = [0u8; N];
    let mut trimmed_len = 0usize;
    let mut byte_pos = START / 8;
    let mut bit_off = START % 8;
    for i in 0..N {
        let b0 = unsafe { *bytes.as_ptr().add(byte_pos) };
        let b1 = unsafe { *bytes.as_ptr().add(byte_pos + 1) };
        let word = ((b0 as u16) << 8) | b1 as u16;
        let val = ((word >> (10 - bit_off)) & 0x3F) as usize;
        let ch = AIS_CHAR[val];
        buf[i] = ch;
        if ch != b'@' && ch != b' ' {
            trimmed_len = i + 1;
        }
        bit_off += 6;
        if bit_off >= 8 {
            bit_off -= 8;
            byte_pos += 1;
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

    #[allow(clippy::needless_range_loop)]
    for i in 0..max_chars.min(N) {
        let val = extract_bits_dyn(bytes, start_bit + i * 6, 6) as usize;
        if val == 0 {
            break;
        }
        buf[i] = AIS_CHAR[val];
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

#[cfg(target_arch = "aarch64")]
mod neon {
    use std::arch::aarch64::*;

    // Inverse of decode_pack in unarmor.rs: scatter 12 packed bytes → 16 6-bit values
    // Each group of 3 bytes yields 4 values
    //   val[4k+0] = byte[3k]     >> 2
    //   val[4k+1] = ((byte[3k]   << 4)  | (byte[3k+1] >> 4)) & 0x3F
    //   val[4k+2] = ((byte[3k+1] << 2)  | (byte[3k+2] >> 6)) & 0x3F
    //   val[4k+3] = byte[3k+2]   & 0x3F
    const HI_IDX: [u8; 16] = [0, 0, 1, 2, 3, 3, 4, 5, 6, 6, 7, 8, 9, 9, 10, 11];
    const LO_IDX: [u8; 16] = [0, 1, 2, 2, 3, 4, 5, 5, 6, 7, 8, 8, 9, 10, 11, 11];
    const HI_SH: [i8; 16] = [-2, 4, 2, 0, -2, 4, 2, 0, -2, 4, 2, 0, -2, 4, 2, 0];
    const LO_SH: [i8; 16] = [
        -8, -4, -6, -8, -8, -4, -6, -8, -8, -4, -6, -8, -8, -4, -6, -8,
    ];

    #[inline]
    #[target_feature(enable = "neon")]
    pub(super) unsafe fn decode_text<const N: usize, const START: usize>(
        bytes: &[u8],
    ) -> ([u8; N], usize) {
        unsafe {
            let hi_idx = vld1q_u8(HI_IDX.as_ptr());
            let lo_idx = vld1q_u8(LO_IDX.as_ptr());
            let hi_sh = vld1q_s8(HI_SH.as_ptr());
            let lo_sh = vld1q_s8(LO_SH.as_ptr());
            let mask63 = vdupq_n_u8(0x3F);
            let thresh = vdupq_n_u8(32);
            let add64 = vdupq_n_u8(64);

            let mut buf = [0u8; N];
            let base = bytes.as_ptr().add(START / 8);

            let mut i = 0_usize;
            while i + 16 <= N {
                // 16 chars where each is 6 bits = 96 bits
                // becuase 96 is divisible by 8, sub-byte alignement is same on each iteration,
                // hence can simply advance pointer by 12 bytes per pass
                let p = base.add(i * 6 / 8);

                // shift the byte stream left so first 6 bit group lands at the MSB of byte 0
                let data: uint8x16_t = match const { START % 8 } {
                    0 => vld1q_u8(p),
                    1 => vorrq_u8(
                        vshlq_n_u8::<1>(vld1q_u8(p)),
                        vshrq_n_u8::<7>(vld1q_u8(p.add(1))),
                    ),
                    2 => vorrq_u8(
                        vshlq_n_u8::<2>(vld1q_u8(p)),
                        vshrq_n_u8::<6>(vld1q_u8(p.add(1))),
                    ),
                    3 => vorrq_u8(
                        vshlq_n_u8::<3>(vld1q_u8(p)),
                        vshrq_n_u8::<5>(vld1q_u8(p.add(1))),
                    ),
                    4 => vorrq_u8(
                        vshlq_n_u8::<4>(vld1q_u8(p)),
                        vshrq_n_u8::<4>(vld1q_u8(p.add(1))),
                    ),
                    5 => vorrq_u8(
                        vshlq_n_u8::<5>(vld1q_u8(p)),
                        vshrq_n_u8::<3>(vld1q_u8(p.add(1))),
                    ),
                    6 => vorrq_u8(
                        vshlq_n_u8::<6>(vld1q_u8(p)),
                        vshrq_n_u8::<2>(vld1q_u8(p.add(1))),
                    ),
                    7 => vorrq_u8(
                        vshlq_n_u8::<7>(vld1q_u8(p)),
                        vshrq_n_u8::<1>(vld1q_u8(p.add(1))),
                    ),
                    _ => unreachable!(),
                };

                // scatter the 12 packed bytes into 16 separate 6 bit values
                let hi = vshlq_u8(vqtbl1q_u8(data, hi_idx), hi_sh);
                let lo = vshlq_u8(vqtbl1q_u8(data, lo_idx), lo_sh);
                let vals = vandq_u8(vorrq_u8(hi, lo), mask63);

                // ais 6 bit to ascii
                let adj = vandq_u8(vcltq_u8(vals, thresh), add64);
                let ch = vaddq_u8(vals, adj);

                vst1q_u8(buf.as_mut_ptr().add(i), ch);
                i += 16;
            }

            // Scalar tail for N % 16 remaining chars (at most 15)
            // After a multiple of 16 chars, i * 6 is divisible by 8, so bit_off = START % 8
            let mut byte_pos = START / 8 + i * 6 / 8;
            let mut bit_off = (START + i * 6) % 8;
            while i < N {
                // Safety: as_slice() guarantees 7 zero bytes past payload, so byte_pos + 1 is valid
                let b0 = *bytes.as_ptr().add(byte_pos);
                let b1 = *bytes.as_ptr().add(byte_pos + 1);
                let word = ((b0 as u16) << 8) | b1 as u16;
                let val = ((word >> (10 - bit_off)) & 0x3F) as usize;
                buf[i] = super::AIS_CHAR[val];
                bit_off += 6;
                if bit_off >= 8 {
                    bit_off -= 8;
                    byte_pos += 1;
                }
                i += 1;
            }

            // backward scan for trailing '@' / ' ' padding
            let mut trimmed_len = N;
            while trimmed_len > 0 && matches!(buf[trimmed_len - 1], b'@' | b' ') {
                trimmed_len -= 1;
            }

            (buf, trimmed_len)
        }
    }
}
