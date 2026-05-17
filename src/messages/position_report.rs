use crate::messages::{
    fields::{
        maneuver_indicator::ManeuverIndicator,
        navigation_status::NavigationStatus,
        position_accuracy::PositionAccuracy,
        primitives::{parse_cog, parse_latitude, parse_longitude, parse_sog, parse_true_heading},
        radio_status::RadioStatus,
        rate_of_turn::RateOfTurn,
    },
    unarmor::Unarmored,
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

impl From<Unarmored> for PositionReport {
    fn from(unarmored: Unarmored) -> Self {
        let Unarmored(bytes) = unarmored;

        let message_type = get_bits_u8(&bytes, 0..=5);
        let repeat_indicator = get_bits_u8(&bytes, 6..=7);
        let mmsi = get_bits_u32(&bytes, 8..=37);
        let navigation_status = NavigationStatus::parse(get_bits_u8(&bytes, 38..=41));
        let rate_of_turn = RateOfTurn::parse(get_bits_u8(&bytes, 42..=49) as i8);
        let speed_over_ground = parse_sog(get_bits_u16(&bytes, 50..=59));
        let position_accuracy = PositionAccuracy::from(get_bits_u8(&bytes, 60..=60));
        let longitude = parse_longitude(get_bits_u32(&bytes, 61..=88) as i32);
        let latitude = parse_latitude(get_bits_u32(&bytes, 89..=115) as i32);
        let course_over_ground = parse_cog(get_bits_u16(&bytes, 116..=127));
        let true_heading = parse_true_heading(get_bits_u16(&bytes, 128..=136));
        let timestamp = get_bits_u8(&bytes, 137..=142);
        let maneuver_indicator = ManeuverIndicator::parse(get_bits_u8(&bytes, 143..=144));
        let raim = get_bit(&bytes, 148);
        let radio_status = RadioStatus::parse(&bytes, 149, message_type);

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
