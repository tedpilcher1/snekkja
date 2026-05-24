use crate::messages::{
    fields::{
        primitives::{parse_lat_i1, parse_lon_i1},
        ship_type::ShipType,
    },
    utils::get_bits,
};

#[derive(Debug)]
pub struct GroupAssignmentCommand {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub ne_lon: Option<f32>,
    pub ne_lat: Option<f32>,
    pub sw_lon: Option<f32>,
    pub sw_lat: Option<f32>,
    pub station_type: u8,
    pub ship_type: ShipType,
    pub txrx: u8,
    pub interval: u8,
    pub quiet: u8,
}

impl From<&[u8]> for GroupAssignmentCommand {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let ne_lon = parse_lon_i1(get_bits::<i32, 40, 18>(bytes));
        let ne_lat = parse_lat_i1(get_bits::<i32, 58, 17>(bytes));
        let sw_lon = parse_lon_i1(get_bits::<i32, 75, 18>(bytes));
        let sw_lat = parse_lat_i1(get_bits::<i32, 93, 17>(bytes));
        let station_type = get_bits::<u8, 110, 4>(bytes);
        let ship_type = ShipType::from(get_bits::<u8, 114, 8>(bytes));
        let txrx = get_bits::<u8, 144, 2>(bytes);
        let interval = get_bits::<u8, 146, 4>(bytes);
        let quiet = get_bits::<u8, 150, 4>(bytes);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            ne_lon,
            ne_lat,
            sw_lon,
            sw_lat,
            station_type,
            ship_type,
            txrx,
            interval,
            quiet,
        }
    }
}
