static DECODE: [u8; 256] = {
    let mut t = [0xFFu8; 256];
    let mut i = 48usize;
    while i <= 87 {
        t[i] = (i as u8) - 48;
        i += 1;
    }
    let mut i = 96usize;
    while i <= 119 {
        t[i] = (i as u8) - 56;
        i += 1;
    }
    t
};

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
        &self.buf[..self.len]
    }

    #[inline(always)]
    pub fn unarmor(&mut self, bytes: &[u8], fill_bits: usize) {
        #[inline(always)]
        fn decode(byte: u8) -> u8 {
            let v = DECODE[byte as usize];
            debug_assert!(v != 0xFF, "Armored byte out of range: {byte}");
            v
        }

        let bit_count = bytes.len() * 6;
        self.len = bit_count.div_ceil(8);

        let iter = bytes.chunks_exact(4);
        let rem = iter.remainder();

        for (inc, outc) in iter.zip(self.buf.chunks_exact_mut(3)) {
            let c0 = decode(inc[0]);
            let c1 = decode(inc[1]);
            let c2 = decode(inc[2]);
            let c3 = decode(inc[3]);
            outc[0] = (c0 << 2) | (c1 >> 4);
            outc[1] = (c1 << 4) | (c2 >> 2);
            outc[2] = (c2 << 6) | c3;
        }

        let bj = (bytes.len() / 4) * 3;
        match rem {
            [c0] => {
                self.buf[bj] = decode(*c0) << 2;
            }
            [c0, c1] => {
                let (c0, c1) = (decode(*c0), decode(*c1));
                self.buf[bj] = (c0 << 2) | (c1 >> 4);
                self.buf[bj + 1] = c1 << 4;
            }
            [c0, c1, c2] => {
                let (c0, c1, c2) = (decode(*c0), decode(*c1), decode(*c2));
                self.buf[bj] = (c0 << 2) | (c1 >> 4);
                self.buf[bj + 1] = (c1 << 4) | (c2 >> 2);
                self.buf[bj + 2] = c2 << 6;
            }
            _ => {}
        }

        if fill_bits != 0 {
            let bits_in_final_byte = match bit_count % 8 {
                0 => 8,
                n => n,
            };
            let final_idx = self.len - 1;
            let shift = (8 - bits_in_final_byte) + fill_bits.min(bits_in_final_byte);
            self.buf[final_idx] &= if shift >= 8 { 0x00 } else { 0xFFu8 << shift };
            if fill_bits > bits_in_final_byte {
                self.buf[final_idx - 1] &= 0xFFu8 << (fill_bits - bits_in_final_byte);
            }
        }
    }
}
