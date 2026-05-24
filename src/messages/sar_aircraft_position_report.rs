use crate::messages::{
    fields::{
        position_accuracy::PositionAccuracy,
        primitives::{parse_cog, parse_latitude, parse_longitude},
        radio_status::RadioStatus,
    },
    utils::{get_bit, get_bits},
};

#[derive(Debug)]
pub struct SarAircraftPositionReport {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub altitude: Option<u16>,
    pub speed_over_ground: Option<u16>,
    pub position_accuracy: PositionAccuracy,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
    pub course_over_ground: Option<f32>,
    pub timestamp: u8,
    pub regional: u8,
    pub dte: bool,
    pub assigned: bool,
    pub raim: bool,
    pub radio: RadioStatus,
}

impl From<&[u8]> for SarAircraftPositionReport {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let altitude = parse_altitude(get_bits::<u16, 38, 12>(bytes));
        let speed_over_ground = parse_sar_sog(get_bits::<u16, 50, 10>(bytes));
        let position_accuracy = PositionAccuracy::from(get_bits::<u8, 60, 1>(bytes));
        let longitude = parse_longitude(get_bits::<i32, 61, 28>(bytes));
        let latitude = parse_latitude(get_bits::<i32, 89, 27>(bytes));
        let course_over_ground = parse_cog(get_bits::<u16, 116, 12>(bytes));
        let timestamp = get_bits::<u8, 128, 6>(bytes);
        let regional = get_bits::<u8, 134, 8>(bytes);
        let dte = get_bit::<142>(bytes);
        let assigned = get_bit::<146>(bytes);
        let raim = get_bit::<147>(bytes);
        let radio = RadioStatus::parse_with_selector(bytes, 148);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            altitude,
            speed_over_ground,
            position_accuracy,
            longitude,
            latitude,
            course_over_ground,
            timestamp,
            regional,
            dte,
            assigned,
            raim,
            radio,
        }
    }
}

#[inline(always)]
fn parse_altitude(data: u16) -> Option<u16> {
    match data {
        4095 => None,
        _ => Some(data),
    }
}

#[inline(always)]
fn parse_sar_sog(data: u16) -> Option<u16> {
    match data {
        1023 => None,
        _ => Some(data),
    }
}
