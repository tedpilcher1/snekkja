use crate::messages::{
    fields::{
        epfd_fix_type::EpfdFixType,
        position_accuracy::PositionAccuracy,
        primitives::{parse_latitude, parse_longitude},
        radio_status::RadioStatus,
    },
    utils::{get_bit, get_bits_i32, get_bits_u8, get_bits_u16, get_bits_u32},
};

#[derive(Debug)]
pub struct BaseStationReport {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub position_accuracy: PositionAccuracy,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
    pub epfd: EpfdFixType,
    pub raim: bool,
    pub radio_status: RadioStatus,
}

impl From<&[u8]> for BaseStationReport {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits_u8::<0, 6>(bytes);
        let repeat_indicator = get_bits_u8::<6, 2>(bytes);
        let mmsi = get_bits_u32::<8, 30>(bytes);
        let year = match get_bits_u16::<38, 14>(bytes) {
            0 => None,
            y => Some(y),
        };
        let month = match get_bits_u8::<52, 4>(bytes) {
            0 => None,
            m => Some(m),
        };
        let day = match get_bits_u8::<56, 5>(bytes) {
            0 => None,
            d => Some(d),
        };
        let hour = match get_bits_u8::<61, 5>(bytes) {
            24 => None,
            h => Some(h),
        };
        let minute = match get_bits_u8::<66, 6>(bytes) {
            60 => None,
            m => Some(m),
        };
        let second = match get_bits_u8::<72, 6>(bytes) {
            60 => None,
            s => Some(s),
        };
        let position_accuracy = PositionAccuracy::from(get_bits_u8::<78, 1>(bytes));
        let longitude = parse_longitude(get_bits_i32::<79, 28>(bytes));
        let latitude = parse_latitude(get_bits_i32::<107, 27>(bytes));
        let epfd = EpfdFixType::from(get_bits_u8::<134, 4>(bytes));
        let raim = get_bit::<148>(bytes);
        let radio_status = RadioStatus::parse(bytes, 149, message_type);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            year,
            month,
            day,
            hour,
            minute,
            second,
            position_accuracy,
            longitude,
            latitude,
            epfd,
            raim,
            radio_status,
        }
    }
}
