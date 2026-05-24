use crate::messages::{
    fields::primitives::{parse_lat_i1, parse_lon_i1},
    utils::{get_bit, get_bits},
};

#[derive(Debug)]
pub enum ChannelManagementTarget {
    Geographic {
        ne_lon: Option<f32>,
        ne_lat: Option<f32>,
        sw_lon: Option<f32>,
        sw_lat: Option<f32>,
    },
    Addressed {
        dest1: u32,
        dest2: u32,
    },
}

#[derive(Debug)]
pub struct ChannelManagement {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub channel_a: u16,
    pub channel_b: u16,
    pub txrx: u8,
    pub power: bool,
    pub target: ChannelManagementTarget,
    pub band_a: bool,
    pub band_b: bool,
    pub zonesize: u8,
}

impl From<&[u8]> for ChannelManagement {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits::<u8, 0, 6>(bytes);
        let repeat_indicator = get_bits::<u8, 6, 2>(bytes);
        let mmsi = get_bits::<u32, 8, 30>(bytes);
        let channel_a = get_bits::<u16, 40, 12>(bytes);
        let channel_b = get_bits::<u16, 52, 12>(bytes);
        let txrx = get_bits::<u8, 64, 4>(bytes);
        let power = get_bit::<68>(bytes);
        let addressed = get_bit::<139>(bytes);
        let band_a = get_bit::<140>(bytes);
        let band_b = get_bit::<141>(bytes);
        let zonesize = get_bits::<u8, 142, 3>(bytes);

        let target = if addressed {
            ChannelManagementTarget::Addressed {
                dest1: get_bits::<u32, 69, 30>(bytes),
                dest2: get_bits::<u32, 104, 30>(bytes),
            }
        } else {
            ChannelManagementTarget::Geographic {
                ne_lon: parse_lon_i1(get_bits::<i32, 69, 18>(bytes)),
                ne_lat: parse_lat_i1(get_bits::<i32, 87, 17>(bytes)),
                sw_lon: parse_lon_i1(get_bits::<i32, 104, 18>(bytes)),
                sw_lat: parse_lat_i1(get_bits::<i32, 122, 17>(bytes)),
            }
        };

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            channel_a,
            channel_b,
            txrx,
            power,
            target,
            band_a,
            band_b,
            zonesize,
        }
    }
}
