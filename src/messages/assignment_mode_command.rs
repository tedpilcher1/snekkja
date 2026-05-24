use crate::messages::utils::{get_bits_u8, get_bits_u16, get_bits_u32};

#[derive(Debug)]
pub struct AssignmentModeCommand {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub mmsi1: u32,
    pub offset1: u16,
    pub increment1: u16,
    pub mmsi2: Option<u32>,
    pub offset2: Option<u16>,
    pub increment2: Option<u16>,
}

impl From<&[u8]> for AssignmentModeCommand {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits_u8::<0, 6>(bytes);
        let repeat_indicator = get_bits_u8::<6, 2>(bytes);
        let mmsi = get_bits_u32::<8, 30>(bytes);
        let mmsi1 = get_bits_u32::<40, 30>(bytes);
        let offset1 = get_bits_u16::<70, 12>(bytes);
        let increment1 = get_bits_u16::<82, 10>(bytes);

        let payload_bits = bytes.len().saturating_sub(7) * 8;

        let (mmsi2, offset2, increment2) = if payload_bits >= 144 {
            (
                Some(get_bits_u32::<92, 30>(bytes)),
                Some(get_bits_u16::<122, 12>(bytes)),
                Some(get_bits_u16::<134, 10>(bytes)),
            )
        } else {
            (None, None, None)
        };

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            mmsi1,
            offset1,
            increment1,
            mmsi2,
            offset2,
            increment2,
        }
    }
}
