use crate::messages::utils::{AisStr, decode_text_dynamic, get_bit, get_bits_u8, get_bits_u32};

#[derive(Debug)]
pub struct AddressedSafetyMessage {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub seqno: u8,
    pub dest_mmsi: u32,
    pub retransmit: bool,
    pub text: AisStr<156>,
}

impl From<&[u8]> for AddressedSafetyMessage {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits_u8::<0, 6>(bytes);
        let repeat_indicator = get_bits_u8::<6, 2>(bytes);
        let mmsi = get_bits_u32::<8, 30>(bytes);
        let seqno = get_bits_u8::<38, 2>(bytes);
        let dest_mmsi = get_bits_u32::<40, 30>(bytes);
        let retransmit = get_bit::<70>(bytes);

        let payload_bits = bytes.len().saturating_sub(7) * 8;
        let max_chars = (payload_bits.saturating_sub(72) / 6).min(156);
        let text = decode_text_dynamic::<156>(bytes, 72, max_chars);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            seqno,
            dest_mmsi,
            retransmit,
            text,
        }
    }
}
