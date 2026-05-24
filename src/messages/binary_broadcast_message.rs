use crate::messages::utils::{get_bits, get_bits_dyn};

#[derive(Debug)]
pub struct BinaryBroadcastMessage {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub dac: u16,
    pub fid: u8,
    pub data_bits: u16,
    pub data: [u8; 119],
}

impl From<&[u8]> for BinaryBroadcastMessage {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let dac = get_bits::<u16, 40, 10>(bytes);
        let fid = get_bits::<u8, 50, 6>(bytes);

        let payload_bits = bytes.len().saturating_sub(7) * 8;
        let data_bit_count = payload_bits.saturating_sub(56).min(952);
        let data = extract_data(bytes, 56, data_bit_count);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            dac,
            fid,
            data_bits: data_bit_count as u16,
            data,
        }
    }
}

fn extract_data(bytes: &[u8], start: usize, data_bits: usize) -> [u8; 119] {
    let mut result = [0u8; 119];
    let full_bytes = data_bits / 8;
    let remaining = data_bits % 8;

    for (i, slot) in result.iter_mut().enumerate().take(full_bytes) {
        *slot = get_bits_dyn(bytes, (start + i * 8)..=(start + i * 8 + 7));
    }

    if remaining > 0 {
        let val: u8 = get_bits_dyn(
            bytes,
            (start + full_bytes * 8)..=(start + full_bytes * 8 + remaining - 1),
        );
        result[full_bytes] = val << (8 - remaining);
    }

    result
}
