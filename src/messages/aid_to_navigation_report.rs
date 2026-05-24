pub use crate::messages::fields::navaid_type::NavaidType;
use crate::messages::{
    fields::{
        epfd_fix_type::EpfdFixType,
        position_accuracy::PositionAccuracy,
        primitives::{parse_latitude, parse_longitude},
    },
    utils::{AisStr, decode_text_dynamic, decode_text_fixed, get_bit, get_bits},
};

#[derive(Debug)]
pub struct AidToNavigationReport {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub aid_type: NavaidType,
    pub name: AisStr<20>,
    pub position_accuracy: PositionAccuracy,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
    pub to_bow: Option<u16>,
    pub to_stern: Option<u16>,
    pub to_port: Option<u8>,
    pub to_starboard: Option<u8>,
    pub epfd: EpfdFixType,
    pub second: u8,
    pub off_position: bool,
    pub regional: u8,
    pub raim: bool,
    pub virtual_aid: bool,
    pub assigned: bool,
    pub name_extension: Option<AisStr<14>>,
}

impl From<&[u8]> for AidToNavigationReport {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let aid_type = NavaidType::from(get_bits::<u8, 38, 5>(bytes));
        let name = decode_text_fixed::<20, 43>(bytes);
        let position_accuracy = PositionAccuracy::from(get_bits::<u8, 163, 1>(bytes));
        let longitude = parse_longitude(get_bits::<i32, 164, 28>(bytes));
        let latitude = parse_latitude(get_bits::<i32, 192, 27>(bytes));
        let to_bow = match get_bits::<u16, 219, 9>(bytes) {
            0 => None,
            v => Some(v),
        };
        let to_stern = match get_bits::<u16, 228, 9>(bytes) {
            0 => None,
            v => Some(v),
        };
        let to_port = match get_bits::<u8, 237, 6>(bytes) {
            0 => None,
            v => Some(v),
        };
        let to_starboard = match get_bits::<u8, 243, 6>(bytes) {
            0 => None,
            v => Some(v),
        };
        let epfd = EpfdFixType::from(get_bits::<u8, 249, 4>(bytes));
        let second = get_bits::<u8, 253, 6>(bytes);
        let off_position = get_bit::<259>(bytes);
        let regional = get_bits::<u8, 260, 8>(bytes);
        let raim = get_bit::<268>(bytes);
        let virtual_aid = get_bit::<269>(bytes);
        let assigned = get_bit::<270>(bytes);

        // bytes.len() includes 7 zero-padding bytes added by the unarmorer
        // subtract them to get the actual payload byte count
        let name_extension = {
            let payload_bits = bytes.len().saturating_sub(7) * 8;
            if payload_bits > 272 {
                let max_chars = ((payload_bits - 272) / 6).min(14);
                if max_chars > 0 {
                    let ext = decode_text_dynamic::<14>(bytes, 272, max_chars);
                    if ext.as_str().is_empty() {
                        None
                    } else {
                        Some(ext)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            aid_type,
            name,
            position_accuracy,
            longitude,
            latitude,
            to_bow,
            to_stern,
            to_port,
            to_starboard,
            epfd,
            second,
            off_position,
            regional,
            raim,
            virtual_aid,
            assigned,
            name_extension,
        }
    }
}
