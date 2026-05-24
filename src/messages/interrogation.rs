use crate::messages::utils::get_bits;

#[derive(Debug)]
pub struct Interrogation {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub mmsi1: u32,
    pub type1_1: u8,
    pub offset1_1: u16,
    pub type1_2: Option<u8>,
    pub offset1_2: Option<u16>,
    pub mmsi2: Option<u32>,
    pub type2_1: Option<u8>,
    pub offset2_1: Option<u16>,
}

impl From<&[u8]> for Interrogation {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let mmsi1 = get_bits::<u32, 40, 30>(bytes);
        let type1_1 = get_bits::<u8, 70, 6>(bytes);
        let offset1_1 = get_bits::<u16, 76, 12>(bytes);

        let payload_bits = bytes.len().saturating_sub(7) * 8;

        // Present at 110 bits (also accept 112-bit padded form)
        let (type1_2, offset1_2) = if payload_bits >= 110 {
            (
                Some(get_bits::<u8, 90, 6>(bytes)),
                Some(get_bits::<u16, 96, 12>(bytes)),
            )
        } else {
            (None, None)
        };

        let (mmsi2, type2_1, offset2_1) = if payload_bits >= 160 {
            (
                Some(get_bits::<u32, 110, 30>(bytes)),
                Some(get_bits::<u8, 140, 6>(bytes)),
                Some(get_bits::<u16, 146, 12>(bytes)),
            )
        } else {
            (None, None, None)
        };

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            mmsi1,
            type1_1,
            offset1_1,
            type1_2,
            offset1_2,
            mmsi2,
            type2_1,
            offset2_1,
        }
    }
}
