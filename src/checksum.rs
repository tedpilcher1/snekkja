use crate::TAIL_EXTRACT;

#[inline]
#[target_feature(enable = "neon")]
pub unsafe fn valid_checksum(sentence: &[u8], expected_checksum: u8) -> bool {
    unsafe {
        use std::arch::aarch64::{
            vdupq_n_u8, veorq_u8, vget_high_u8, vget_lane_u64, vget_low_u8, vld1q_u8, vqtbx1q_u8,
            vreinterpret_u64_u8,
        };

        let mut vacc = vdupq_n_u8(0);
        let mut i = 0usize;

        while i + 16 <= sentence.len() {
            vacc = veorq_u8(vacc, vld1q_u8(sentence.as_ptr().add(i)));
            i += 16;
        }

        let tail_len = sentence.len() - i;
        if tail_len > 0 {
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

        // Extract the two 8-byte halves as u64 and fold into one byte
        let lo = vget_lane_u64(vreinterpret_u64_u8(vget_low_u8(vacc)), 0);
        let hi = vget_lane_u64(vreinterpret_u64_u8(vget_high_u8(vacc)), 0);
        let mut acc = lo ^ hi;
        acc ^= acc >> 32;
        acc ^= acc >> 16;
        acc ^= acc >> 8;

        acc as u8 == expected_checksum
    }
}
