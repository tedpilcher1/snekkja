use crate::messages::utils::get_bits;

#[derive(Debug)]
pub struct UtcDateInquiry {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub dest_mmsi: u32,
}

impl From<&[u8]> for UtcDateInquiry {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let dest_mmsi = get_bits::<u32, 40, 30>(bytes);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            dest_mmsi,
        }
    }
}
