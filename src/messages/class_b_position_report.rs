use crate::messages::{
    fields::{
        position_accuracy::PositionAccuracy,
        primitives::{parse_cog, parse_latitude, parse_longitude, parse_sog, parse_true_heading},
        radio_status::RadioStatus,
    },
    utils::{get_bit, get_bits},
};

#[derive(Debug)]
pub struct ClassBPositionReport {
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
    pub cs: bool,
    pub display: bool,
    pub dsc: bool,
    pub band: bool,
    pub msg22: bool,
    pub assigned: bool,
    pub raim: bool,
    pub radio: RadioStatus,
}

impl From<&[u8]> for ClassBPositionReport {
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
        let regional = get_bits::<u8, 139, 2>(bytes);
        let cs = get_bit::<141>(bytes);
        let display = get_bit::<142>(bytes);
        let dsc = get_bit::<143>(bytes);
        let band = get_bit::<144>(bytes);
        let msg22 = get_bit::<145>(bytes);
        let assigned = get_bit::<146>(bytes);
        let raim = get_bit::<147>(bytes);
        let radio = RadioStatus::parse_with_selector(bytes, 148);

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
            cs,
            display,
            dsc,
            band,
            msg22,
            assigned,
            raim,
            radio,
        }
    }
}
