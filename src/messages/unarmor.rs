pub struct Unarmored(pub Box<[u8]>);

impl Unarmored {
    #[inline(always)]
    pub fn unarmor(bytes: &[u8], fill_bits: usize) -> Self {
        // Each ASCII character encodes 6 bits of AIS data.
        // Calculate how many output bytes we need to hold all those bits.
        // e.g. 3 chars = 18 bits = 3 bytes; 4 chars = 24 bits = 3 bytes; 5 chars = 30 bits = 4 bytes
        let bit_count = bytes.len() * 6;
        let byte_count = bit_count.div_ceil(8);
        let mut output = vec![0u8; byte_count];

        // Walk each armored ASCII byte, decode it to 6 bits, and pack it into
        // the output buffer MSB-first (i.e. the first character's bits land at
        // the most-significant end of byte 0).
        for (i, byte) in bytes.iter().enumerate() {
            // Decode ASCII → 6-bit value per the AIS armoring table:
            //   '0'–'W' (48–87)  → subtract 48  → 0–39
            //   '`'–'w' (96–119) → subtract 56  → 40–63
            let six_bits: u8 = match byte {
                48..=87 => byte - 48,
                96..=119 => byte - 56,
                _ => panic!("Armored byte out of range: {byte}"),
            };

            // We're writing 6 bits starting at bit offset `i * 6` in the output stream.
            // Work out which byte that lands in, and how far into that byte.
            //
            // Example: character index 3 → bit offset 18
            //   offset_byte = 18 / 8 = 2   (third output byte)
            //   offset_bit  = 18 % 8 = 2   (start 2 bits into that byte)
            let bit_offset = i * 6;
            let offset_byte = bit_offset / 8;
            let offset_bit = bit_offset % 8; // how many bits are already occupied in this byte

            // Pack 6 bits MSB-first into the output byte stream.
            // Bit stream position `offset_bit` maps to bit (7 - offset_bit) of the current byte,
            // so we left-shift to align the value with the MSB end of the remaining space.
            if offset_bit <= 2 {
                // All 6 bits fit in the current byte.
                output[offset_byte] |= six_bits << (2 - offset_bit);
            } else {
                // Top (8 - offset_bit) bits fill the rest of the current byte.
                output[offset_byte] |= six_bits >> (offset_bit - 2);
                // Remaining (offset_bit - 2) bits go into the MSB end of the next byte.
                output[offset_byte + 1] |= six_bits << (10 - offset_bit);
            }
        }

        // Zero out the fill bits at the tail of the output.
        // Fill bits are padding introduced to reach a 6-bit boundary; they carry no data.
        if fill_bits != 0 {
            // How many real (non-padding) bits are in the final output byte?
            // If bit_count is a multiple of 8, the whole final byte is real data.
            let bits_in_final_byte = match bit_count % 8 {
                0 => 8,
                n => n,
            };
            let final_idx = byte_count - 1;

            // Build a mask that preserves only the real bits in the final byte.
            // `shift` is how far left we need to push 0xFF to knock out the fill bits.
            //
            // e.g. bits_in_final_byte = 6, fill_bits = 2:
            //   unused = 8 - 6 = 2 low bits are structurally empty
            //   fill occupies 2 of the 6 real bits
            //   shift = 2 + 2 = 4 → mask = 0xFF << 4 = 0b11110000
            let shift = (8 - bits_in_final_byte) + fill_bits.min(bits_in_final_byte);
            output[final_idx] &= if shift >= 8 { 0x00 } else { 0xFFu8 << shift };

            // Rare case: fill_bits is larger than the bits available in the final byte,
            // meaning the padding overflows back into the second-to-last byte too.
            if fill_bits > bits_in_final_byte {
                let overflow = fill_bits - bits_in_final_byte;
                output[final_idx - 1] &= 0xFFu8 << overflow;
            }
        }

        Self(output.into_boxed_slice())
    }
}
