use crate::messages::{
    fields::{
        epfd_fix_type::EpfdFixType,
        position_accuracy::PositionAccuracy,
        primitives::{parse_cog, parse_latitude, parse_longitude, parse_sog, parse_true_heading},
        ship_type::ShipType,
    },
    utils::{AisStr, decode_text_fixed, get_bit, get_bits},
};

#[derive(Debug)]
pub struct ExtendedClassBPositionReport {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub speed_over_ground: Option<f32>,
    pub position_accuracy: PositionAccuracy,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
    pub course_over_ground: Option<f32>,
    pub true_heading: Option<u16>,
    pub timestamp: u8,
    pub regional: u8,
    pub shipname: AisStr<20>,
    pub ship_type: ShipType,
    pub to_bow: Option<u16>,
    pub to_stern: Option<u16>,
    pub to_port: Option<u8>,
    pub to_starboard: Option<u8>,
    pub epfd: EpfdFixType,
    pub raim: bool,
    pub dte: bool,
    pub assigned: bool,
}

impl From<&[u8]> for ExtendedClassBPositionReport {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let speed_over_ground = parse_sog(get_bits::<u16, 46, 10>(bytes));
        let position_accuracy = PositionAccuracy::from(get_bits::<u8, 56, 1>(bytes));
        let longitude = parse_longitude(get_bits::<i32, 57, 28>(bytes));
        let latitude = parse_latitude(get_bits::<i32, 85, 27>(bytes));
        let course_over_ground = parse_cog(get_bits::<u16, 112, 12>(bytes));
        let true_heading = parse_true_heading(get_bits::<u16, 124, 9>(bytes));
        let timestamp = get_bits::<u8, 133, 6>(bytes);
        let regional = get_bits::<u8, 139, 4>(bytes);
        let shipname = decode_text_fixed::<20, 143>(bytes);
        let ship_type = ShipType::from(get_bits::<u8, 263, 8>(bytes));
        let to_bow = match get_bits::<u16, 271, 9>(bytes) {
            0 => None,
            v => Some(v),
        };
        let to_stern = match get_bits::<u16, 280, 9>(bytes) {
            0 => None,
            v => Some(v),
        };
        let to_port = match get_bits::<u8, 289, 6>(bytes) {
            0 => None,
            v => Some(v),
        };
        let to_starboard = match get_bits::<u8, 295, 6>(bytes) {
            0 => None,
            v => Some(v),
        };
        let epfd = EpfdFixType::from(get_bits::<u8, 301, 4>(bytes));
        let raim = get_bit::<305>(bytes);
        let dte = get_bit::<306>(bytes);
        let assigned = get_bit::<307>(bytes);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            speed_over_ground,
            position_accuracy,
            longitude,
            latitude,
            course_over_ground,
            true_heading,
            timestamp,
            regional,
            shipname,
            ship_type,
            to_bow,
            to_stern,
            to_port,
            to_starboard,
            epfd,
            raim,
            dte,
            assigned,
        }
    }
}
