use crate::messages::utils::{AisStr, decode_text_dynamic, get_bits};

#[derive(Debug)]
pub struct SafetyBroadcastMessage {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub text: AisStr<161>,
}

impl From<&[u8]> for SafetyBroadcastMessage {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);

        let payload_bits = bytes.len().saturating_sub(7) * 8;
        let max_chars = (payload_bits.saturating_sub(40) / 6).min(161);
        let text = decode_text_dynamic::<161>(bytes, 40, max_chars);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            text,
        }
    }
}
