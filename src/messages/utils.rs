use std::ops::RangeInclusive;

#[inline(always)]
fn get_bits(bytes: &[u8], start: usize, len: usize) -> u64 {
    let byte_start = start / 8;
    let bit_offset = start % 8;
    let bytes_needed = (bit_offset + len + 7) / 8;

    let mut val = 0u64;
    for i in 0..bytes_needed {
        val = (val << 8) | bytes[byte_start + i] as u64;
    }

    let shift = bytes_needed * 8 - bit_offset - len;
    (val >> shift) & ((1u64 << len) - 1)
}

#[inline(always)]
pub fn get_bit(bytes: &[u8], i: usize) -> bool {
    get_bits(bytes, i, 1) != 0
}

#[inline(always)]
pub fn get_bits_u8(bytes: &[u8], range: RangeInclusive<usize>) -> u8 {
    get_bits(
        bytes.into(),
        *range.start(),
        range.end() - range.start() + 1,
    ) as u8
}

#[inline(always)]
pub fn get_bits_u16(bytes: &[u8], range: RangeInclusive<usize>) -> u16 {
    get_bits(bytes, *range.start(), range.end() - range.start() + 1) as u16
}

#[inline(always)]
pub fn get_bits_u32(bytes: &[u8], range: RangeInclusive<usize>) -> u32 {
    get_bits(bytes, *range.start(), range.end() - range.start() + 1) as u32
}
