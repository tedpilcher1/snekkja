#[inline]
#[target_feature(enable = "neon")]
pub unsafe fn valid_checksum(sentence: &[u8], expected_checksum: u8) -> bool {
    unsafe {
        use std::arch::aarch64::{
            vdupq_n_u8, veorq_u8, vget_high_u8, vget_lane_u64, vget_low_u8, vld1q_u8, vqtbx1q_u8,
            vreinterpret_u64_u8,
        };

        // TAIL_EXTRACT[k][j]: selects byte j of the k-byte tail from the last 16 bytes,
        // index 255 is out-of-range for vqtbx1q_u8 so those lanes return the fallback (0).
        static TAIL_EXTRACT: [[u8; 16]; 16] = {
            let mut t = [[255u8; 16]; 16];
            let mut k = 1usize;
            while k < 16 {
                let mut j = 0usize;
                while j < k {
                    t[k][j] = (16 - k + j) as u8;
                    j += 1;
                }
                k += 1;
            }
            t
        };

        let mut vacc = vdupq_n_u8(0);
        let mut i = 0usize;

        while i + 16 <= sentence.len() {
            vacc = veorq_u8(vacc, vld1q_u8(sentence.as_ptr().add(i)));
            i += 16;
        }

        let tail_len = sentence.len() - i;
        if tail_len > 0 {
            // For i > 0 (true for all real AIS sentences >= 16 bytes), overlap-load the
            // last 16 bytes and shuffle the tail into positions 0..tail_len with 0-padding.
            // XOR with 0 is identity, so the padding bytes don't affect the result.
            let chunk = if i > 0 {
                let last16 = vld1q_u8(sentence.as_ptr().add(sentence.len() - 16));
                let extract = vld1q_u8(TAIL_EXTRACT[tail_len].as_ptr());
                vqtbx1q_u8(vdupq_n_u8(0), last16, extract)
            } else {
                let mut tmp = [0u8; 16];
                core::ptr::copy_nonoverlapping(sentence.as_ptr(), tmp.as_mut_ptr(), tail_len);
                vld1q_u8(tmp.as_ptr())
            };
            vacc = veorq_u8(vacc, chunk);
        }

        // Extract the two 8-byte halves as u64 and fold into one byte via scalar shifts.
        let lo = vget_lane_u64(vreinterpret_u64_u8(vget_low_u8(vacc)), 0);
        let hi = vget_lane_u64(vreinterpret_u64_u8(vget_high_u8(vacc)), 0);
        let mut acc = lo ^ hi;
        acc ^= acc >> 32;
        acc ^= acc >> 16;
        acc ^= acc >> 8;

        acc as u8 == expected_checksum
    }
}
