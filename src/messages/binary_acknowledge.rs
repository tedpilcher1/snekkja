use crate::messages::utils::get_bits;

#[derive(Debug)]
pub struct BinaryAcknowledge {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub mmsi1: u32,
    pub mmsiseq1: u8,
    pub mmsi2: Option<u32>,
    pub mmsiseq2: Option<u8>,
    pub mmsi3: Option<u32>,
    pub mmsiseq3: Option<u8>,
    pub mmsi4: Option<u32>,
    pub mmsiseq4: Option<u8>,
}

impl From<&[u8]> for BinaryAcknowledge {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let mmsi1 = get_bits::<u32, 40, 30>(bytes);
        let mmsiseq1 = get_bits::<u8, 70, 2>(bytes);

        // Each additional destination occupies 32 bits, present only if the
        // payload byte count covers the relevant bit range
        let payload_bits = bytes.len().saturating_sub(7) * 8;

        let (mmsi2, mmsiseq2) = if payload_bits > 103 {
            (
                Some(get_bits::<u32, 72, 30>(bytes)),
                Some(get_bits::<u8, 102, 2>(bytes)),
            )
        } else {
            (None, None)
        };

        let (mmsi3, mmsiseq3) = if payload_bits > 135 {
            (
                Some(get_bits::<u32, 104, 30>(bytes)),
                Some(get_bits::<u8, 134, 2>(bytes)),
            )
        } else {
            (None, None)
        };

        let (mmsi4, mmsiseq4) = if payload_bits > 167 {
            (
                Some(get_bits::<u32, 136, 30>(bytes)),
                Some(get_bits::<u8, 166, 2>(bytes)),
            )
        } else {
            (None, None)
        };

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            mmsi1,
            mmsiseq1,
            mmsi2,
            mmsiseq2,
            mmsi3,
            mmsiseq3,
            mmsi4,
            mmsiseq4,
        }
    }
}
