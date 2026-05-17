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

        let chunks = bytes.len() / 4;
        let rem = bytes.len() % 4;

        for k in 0..chunks {
            let i = k * 4;
            let j = k * 3;
            let c0 = decode(bytes[i]);
            let c1 = decode(bytes[i + 1]);
            let c2 = decode(bytes[i + 2]);
            let c3 = decode(bytes[i + 3]);
            self.buf[j] = (c0 << 2) | (c1 >> 4);
            self.buf[j + 1] = (c1 << 4) | (c2 >> 2);
            self.buf[j + 2] = (c2 << 6) | c3;
        }

        let bi = chunks * 4;
        let bj = chunks * 3;
        match rem {
            1 => {
                self.buf[bj] = decode(bytes[bi]) << 2;
            }
            2 => {
                let c0 = decode(bytes[bi]);
                let c1 = decode(bytes[bi + 1]);
                self.buf[bj] = (c0 << 2) | (c1 >> 4);
                self.buf[bj + 1] = c1 << 4;
            }
            3 => {
                let c0 = decode(bytes[bi]);
                let c1 = decode(bytes[bi + 1]);
                let c2 = decode(bytes[bi + 2]);
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
