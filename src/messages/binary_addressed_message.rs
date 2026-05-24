use crate::messages::utils::{get_bit, get_bits, get_bits_dyn};

#[derive(Debug)]
pub struct BinaryAddressedMessage {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub seqno: u8,
    pub dest_mmsi: u32,
    pub retransmit: bool,
    pub dac: u16,
    pub fid: u8,
    pub data_bits: u16,
    pub data: [u8; 115],
}

impl From<&[u8]> for BinaryAddressedMessage {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let seqno = get_bits::<u8, 38, 2>(bytes);
        let dest_mmsi = get_bits::<u32, 40, 30>(bytes);
        let retransmit = get_bit::<70>(bytes);
        let dac = get_bits::<u16, 72, 10>(bytes);
        let fid = get_bits::<u8, 82, 6>(bytes);

        let payload_bits = bytes.len().saturating_sub(7) * 8;
        let data_bit_count = payload_bits.saturating_sub(88).min(920);
        let data = extract_data(bytes, 88, data_bit_count);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            seqno,
            dest_mmsi,
            retransmit,
            dac,
            fid,
            data_bits: data_bit_count as u16,
            data,
        }
    }
}

fn extract_data(bytes: &[u8], start: usize, data_bits: usize) -> [u8; 115] {
    let mut result = [0u8; 115];
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
