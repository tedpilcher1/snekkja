use crate::messages::utils::get_bits;

#[derive(Debug)]
pub struct DataLinkManagement {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub offset1: u16,
    pub number1: u8,
    pub timeout1: u8,
    pub increment1: u16,
    pub offset2: Option<u16>,
    pub number2: Option<u8>,
    pub timeout2: Option<u8>,
    pub increment2: Option<u16>,
    pub offset3: Option<u16>,
    pub number3: Option<u8>,
    pub timeout3: Option<u8>,
    pub increment3: Option<u16>,
    pub offset4: Option<u16>,
    pub number4: Option<u8>,
    pub timeout4: Option<u8>,
    pub increment4: Option<u16>,
}

impl From<&[u8]> for DataLinkManagement {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let offset1 = get_bits::<u16, 40, 12>(bytes);
        let number1 = get_bits::<u8, 52, 4>(bytes);
        let timeout1 = get_bits::<u8, 56, 3>(bytes);
        let increment1 = get_bits::<u16, 59, 11>(bytes);

        let payload_bits = bytes.len().saturating_sub(7) * 8;

        let (offset2, number2, timeout2, increment2) = if payload_bits >= 100 {
            (
                Some(get_bits::<u16, 70, 12>(bytes)),
                Some(get_bits::<u8, 82, 4>(bytes)),
                Some(get_bits::<u8, 86, 3>(bytes)),
                Some(get_bits::<u16, 89, 11>(bytes)),
            )
        } else {
            (None, None, None, None)
        };

        let (offset3, number3, timeout3, increment3) = if payload_bits >= 130 {
            (
                Some(get_bits::<u16, 100, 12>(bytes)),
                Some(get_bits::<u8, 112, 4>(bytes)),
                Some(get_bits::<u8, 116, 3>(bytes)),
                Some(get_bits::<u16, 119, 11>(bytes)),
            )
        } else {
            (None, None, None, None)
        };

        let (offset4, number4, timeout4, increment4) = if payload_bits >= 160 {
            (
                Some(get_bits::<u16, 130, 12>(bytes)),
                Some(get_bits::<u8, 142, 4>(bytes)),
                Some(get_bits::<u8, 146, 3>(bytes)),
                Some(get_bits::<u16, 149, 11>(bytes)),
            )
        } else {
            (None, None, None, None)
        };

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            offset1,
            number1,
            timeout1,
            increment1,
            offset2,
            number2,
            timeout2,
            increment2,
            offset3,
            number3,
            timeout3,
            increment3,
            offset4,
            number4,
            timeout4,
            increment4,
        }
    }
}
