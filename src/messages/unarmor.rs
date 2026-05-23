use std::arch::aarch64::{
    int8x16_t, uint8x16_t, vandq_u8, vcgtq_u8, vdupq_n_u8, vget_high_u8, vget_lane_u32,
    vget_low_u8, vld1q_s8, vld1q_u8, vorrq_u8, vqtbl1q_u8, vqtbx1q_u8, vreinterpret_u32_u8,
    vshlq_u8, vst1_u8, vsubq_u8,
};

use crate::TAIL_EXTRACT;

const HI_IDX: [u8; 16] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14, 0, 0, 0, 0];
const LO_IDX: [u8; 16] = [1, 2, 3, 5, 6, 7, 9, 10, 11, 13, 14, 15, 0, 0, 0, 0];
const HI_SH: [i8; 16] = [2, 4, 6, 2, 4, 6, 2, 4, 6, 2, 4, 6, 0, 0, 0, 0];
const LO_SH: [i8; 16] = [-4, -2, 0, -4, -2, 0, -4, -2, 0, -4, -2, 0, 0, 0, 0, 0];

#[derive(Debug)]
pub struct Unarmored {
    buf: [u8; 256],
    len: usize,
}

impl Default for Unarmored {
    fn default() -> Self {
        Self::new()
    }
}

impl Unarmored {
    pub fn new() -> Self {
        Self {
            buf: [0; 256],
            len: 0,
        }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        // Safety: self.len is always set to bit_count.div_ceil(8) where
        // bit_count = bytes.len() * 6. AIS payloads fit well within the
        // 256-byte buf, so self.len <= buf.len() is always satisfied
        unsafe { self.buf.get_unchecked(..self.len) }
    }

    #[inline]
    #[target_feature(enable = "neon")]
    pub unsafe fn unarmor(&mut self, input: &[u8], fill_bits: usize) {
        unsafe {
            self.len = (input.len() * 6).div_ceil(8);

            let mut i = 0_usize;
            let mut o = 0_usize;

            // For each group of 4 6-bit values, we produce 3 packed bytes:
            //   out[0] = (v[a]<<2) | (v[b]>>4)
            //   out[1] = (v[b]<<4) | (v[c]>>2)
            //   out[2] = (v[c]<<6) | v[d]
            // Done in 4 groups (16 -> 12) per pass
            let hi_idx = vld1q_u8(HI_IDX.as_ptr());
            let lo_idx = vld1q_u8(LO_IDX.as_ptr());
            let hi_sh = vld1q_s8(HI_SH.as_ptr());
            let lo_sh = vld1q_s8(LO_SH.as_ptr());

            let v48 = vdupq_n_u8(48);
            let v40 = vdupq_n_u8(40);
            let v8 = vdupq_n_u8(8);

            while i + 16 <= input.len() {
                let chunk = vld1q_u8(input.as_ptr().add(i));
                let packed = decode_pack(chunk, v48, v40, v8, hi_idx, lo_idx, hi_sh, lo_sh);
                write12(self.buf.as_mut_ptr().add(o), packed);
                i += 16;
                o += 12;
            }

            let tail_len = input.len() - i;
            if tail_len > 0 {
                // Load the last 16 bytes of input (valid when i > 0, i.e. input.len() >= 16),
                // then use vqtbx1q_u8 to extract the tail chars into positions 0..tail_len
                // and fill positions tail_len..16 with b'0' (decodes to 0) in one instruction.
                // This avoids any copy and keeps the shuffle/shift constants in registers.
                // For the rare case where input.len() < 16 (no NEON loop ran), fall back to
                // a tmp buffer — all real AIS messages are >= 27 chars so this branch is dead.
                let chunk = if i > 0 {
                    let last16 = vld1q_u8(input.as_ptr().add(input.len() - 16));
                    let extract = vld1q_u8(TAIL_EXTRACT[tail_len].as_ptr());
                    vqtbx1q_u8(v48, last16, extract)
                } else {
                    let mut tmp = [b'0'; 16];
                    core::ptr::copy_nonoverlapping(input.as_ptr(), tmp.as_mut_ptr(), tail_len);
                    vld1q_u8(tmp.as_ptr())
                };
                let packed = decode_pack(chunk, v48, v40, v8, hi_idx, lo_idx, hi_sh, lo_sh);
                // Writing 12 bytes is safe as any bytes past self.len are never exposed by as_slice()
                write12(self.buf.as_mut_ptr().add(o), packed);
            }

            if fill_bits != 0 {
                let bit_count = input.len() * 6;
                let bits_in_final_byte = match bit_count % 8 {
                    0 => 8,
                    n => n,
                };

                let final_idx = self.len - 1;
                let shift = (8 - bits_in_final_byte) + fill_bits.min(bits_in_final_byte);

                *self.buf.get_unchecked_mut(final_idx) &=
                    if shift >= 8 { 0x00 } else { 0xFFu8 << shift };

                if fill_bits > bits_in_final_byte {
                    *self.buf.get_unchecked_mut(final_idx - 1) &=
                        0xFFu8 << (fill_bits - bits_in_final_byte);
                }
            }
        }
    }
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn decode_pack(
    chunk: uint8x16_t,
    v48: uint8x16_t,
    v40: uint8x16_t,
    v8: uint8x16_t,
    hi_idx: uint8x16_t,
    lo_idx: uint8x16_t,
    hi_sh: int8x16_t,
    lo_sh: int8x16_t,
) -> uint8x16_t {
    unsafe {
        let vals = vsubq_u8(chunk, v48);
        let adj = vandq_u8(vcgtq_u8(vals, v40), v8);
        let vals = vsubq_u8(vals, adj);
        let hi = vshlq_u8(vqtbl1q_u8(vals, hi_idx), hi_sh);
        let lo = vshlq_u8(vqtbl1q_u8(vals, lo_idx), lo_sh);
        vorrq_u8(hi, lo)
    }
}

#[inline(always)]
unsafe fn write12(out: *mut u8, packed: uint8x16_t) {
    unsafe {
        vst1_u8(out, vget_low_u8(packed));
        let hi4 = vget_lane_u32(vreinterpret_u32_u8(vget_high_u8(packed)), 0);
        (out.add(8) as *mut u32).write_unaligned(hi4);
    }
}
