use crate::messages::{
    fields::{
        navigation_status::NavigationStatus,
        position_accuracy::PositionAccuracy,
        primitives::{parse_lat_i1, parse_lon_i1},
    },
    utils::{get_bit, get_bits},
};

#[derive(Debug)]
pub struct LongRangePositionReport {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub position_accuracy: PositionAccuracy,
    pub raim: bool,
    pub nav_status: Option<NavigationStatus>,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
    pub speed_over_ground: Option<u8>,
    pub course_over_ground: Option<u16>,
    pub gnss: bool,
}

impl From<&[u8]> for LongRangePositionReport {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let position_accuracy = PositionAccuracy::from(get_bits::<u8, 38, 1>(bytes));
        let raim = get_bit::<39>(bytes);
        let nav_status = NavigationStatus::parse(get_bits::<u8, 40, 4>(bytes));
        let longitude = parse_lon_i1(get_bits::<i32, 44, 18>(bytes));
        let latitude = parse_lat_i1(get_bits::<i32, 62, 17>(bytes));
        let speed_over_ground = match get_bits::<u8, 79, 6>(bytes) {
            63 => None,
            v => Some(v),
        };
        let course_over_ground = match get_bits::<u16, 85, 9>(bytes) {
            511 => None,
            v => Some(v),
        };
        let gnss = get_bit::<94>(bytes);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            position_accuracy,
            raim,
            nav_status,
            longitude,
            latitude,
            speed_over_ground,
            course_over_ground,
            gnss,
        }
    }
}
