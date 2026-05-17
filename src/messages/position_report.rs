use crate::messages::{
    fields::{
        maneuver_indicator::ManeuverIndicator,
        navigation_status::NavigationStatus,
        position_accuracy::PositionAccuracy,
        primitives::{parse_cog, parse_latitude, parse_longitude, parse_sog, parse_true_heading},
        radio_status::RadioStatus,
        rate_of_turn::RateOfTurn,
    },
    utils::{get_bit, get_bits_u8, get_bits_u16, get_bits_u32},
};

#[derive(Debug)]
pub struct PositionReport {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub navigation_status: Option<NavigationStatus>,
    pub rate_of_turn: Option<RateOfTurn>,
    pub speed_over_ground: Option<f32>,
    pub position_accuracy: PositionAccuracy,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
    pub course_over_ground: Option<f32>,
    pub true_heading: Option<u16>,
    pub timestamp: u8,
    pub maneuver_indicator: Option<ManeuverIndicator>,
    pub raim: bool,
    pub radio_status: RadioStatus,
}

impl From<&[u8]> for PositionReport {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits_u8::<0, 6>(bytes);
        let repeat_indicator = get_bits_u8::<6, 2>(bytes);
        let mmsi = get_bits_u32::<8, 30>(bytes);
        let navigation_status = NavigationStatus::parse(get_bits_u8::<38, 4>(bytes));
        let rate_of_turn = RateOfTurn::parse(get_bits_u8::<42, 8>(bytes) as i8);
        let speed_over_ground = parse_sog(get_bits_u16::<50, 10>(bytes));
        let position_accuracy = PositionAccuracy::from(get_bits_u8::<60, 1>(bytes));
        let longitude = parse_longitude(get_bits_u32::<61, 28>(bytes) as i32);
        let latitude = parse_latitude(get_bits_u32::<89, 27>(bytes) as i32);
        let course_over_ground = parse_cog(get_bits_u16::<116, 12>(bytes));
        let true_heading = parse_true_heading(get_bits_u16::<128, 9>(bytes));
        let timestamp = get_bits_u8::<137, 6>(bytes);
        let maneuver_indicator = ManeuverIndicator::parse(get_bits_u8::<143, 2>(bytes));
        let raim = get_bit::<148>(bytes);
        let radio_status = RadioStatus::parse(bytes, 149, message_type);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            navigation_status,
            rate_of_turn,
            speed_over_ground,
            position_accuracy,
            longitude,
            latitude,
            course_over_ground,
            true_heading,
            timestamp,
            maneuver_indicator,
            raim,
            radio_status,
        }
    }
}
