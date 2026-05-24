use crate::messages::utils::{get_bit, get_bits, get_bits_dyn};

#[derive(Debug)]
pub struct SingleSlotBinaryMessage {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub addressed: bool,
    pub structured: bool,
    pub dest_mmsi: Option<u32>,
    pub app_id: Option<u16>,
    pub data_bits: u8,
    pub data: [u8; 16],
}

impl From<&[u8]> for SingleSlotBinaryMessage {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let addressed = get_bit::<38>(bytes);
        let structured = get_bit::<39>(bytes);

        let payload_bits = bytes.len().saturating_sub(7) * 8;

        let dest_mmsi = if addressed {
            Some(get_bits::<u32, 40, 30>(bytes))
        } else {
            None
        };

        let app_id_start = if addressed { 70 } else { 40 };
        let app_id = if structured {
            Some(get_bits_dyn::<u16>(bytes, app_id_start..=app_id_start + 15))
        } else {
            None
        };

        let data_start = app_id_start + if structured { 16 } else { 0 };
        let data_bit_count = payload_bits.saturating_sub(data_start).min(128);
        let data = extract_data(bytes, data_start, data_bit_count);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            addressed,
            structured,
            dest_mmsi,
            app_id,
            data_bits: data_bit_count as u8,
            data,
        }
    }
}

fn extract_data(bytes: &[u8], start: usize, data_bits: usize) -> [u8; 16] {
    let mut result = [0u8; 16];
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
