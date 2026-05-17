use std::ops::RangeInclusive;

#[inline(always)]
fn get_bits<const START: usize, const LEN: usize>(bytes: &[u8]) -> u64 {
    let byte_start = const { START / 8 };
    let bytes_needed = const { (START % 8 + LEN).div_ceil(8) };
    let shift = const { (START % 8 + LEN).div_ceil(8) * 8 - START % 8 - LEN };
    let mask: u64 = const { (1u64 << LEN) - 1 };

    let mut val = 0u64;
    for i in 0..bytes_needed {
        val = (val << 8) | bytes[byte_start + i] as u64;
    }
    (val >> shift) & mask
}

#[inline(always)]
fn get_bits_dyn(bytes: &[u8], start: usize, len: usize) -> u64 {
    let byte_start = start / 8;
    let bit_offset = start % 8;
    let bytes_needed = (bit_offset + len).div_ceil(8);

    let mut val = 0u64;
    for i in 0..bytes_needed {
        val = (val << 8) | bytes[byte_start + i] as u64;
    }

    let shift = bytes_needed * 8 - bit_offset - len;
    (val >> shift) & ((1u64 << len) - 1)
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
