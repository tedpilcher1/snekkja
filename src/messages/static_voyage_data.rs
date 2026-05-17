use crate::messages::{
    fields::{epfd_fix_type::EpfdFixType, primitives::parse_draught, ship_type::ShipType},
    utils::{AisStr, decode_text_fixed, get_bit, get_bits_u8, get_bits_u16, get_bits_u32},
};

#[derive(Debug)]
pub struct StaticVoyageData {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub ais_version: u8,
    pub imo: u32,
    pub callsign: AisStr<7>,
    pub shipname: AisStr<20>,
    pub shiptype: ShipType,
    pub to_bow: Option<u16>,
    pub to_stern: Option<u16>,
    pub to_port: Option<u8>,
    pub to_starboard: Option<u8>,
    pub epfd: EpfdFixType,
    pub eta_month: Option<u8>,
    pub eta_day: Option<u8>,
    pub eta_hour: Option<u8>,
    pub eta_minute: Option<u8>,
    pub draught: Option<f32>,
    pub destination: AisStr<20>,
    pub dte: bool,
}

impl From<&[u8]> for StaticVoyageData {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits_u8::<0, 6>(bytes);
        let repeat_indicator = get_bits_u8::<6, 2>(bytes);
        let mmsi = get_bits_u32::<8, 30>(bytes);
        let ais_version = get_bits_u8::<38, 2>(bytes);
        let imo = get_bits_u32::<40, 30>(bytes);
        let callsign = decode_text_fixed::<7>(bytes, 70, 7);
        let shipname = decode_text_fixed::<20>(bytes, 112, 20);
        let shiptype = ShipType::from(get_bits_u8::<232, 8>(bytes));
        let to_bow = match get_bits_u16::<240, 9>(bytes) {
            0 => None,
            v => Some(v),
        };
        let to_stern = match get_bits_u16::<249, 9>(bytes) {
            0 => None,
            v => Some(v),
        };
        let to_port = match get_bits_u8::<258, 6>(bytes) {
            0 => None,
            v => Some(v),
        };
        let to_starboard = match get_bits_u8::<264, 6>(bytes) {
            0 => None,
            v => Some(v),
        };
        let epfd = EpfdFixType::from(get_bits_u8::<270, 4>(bytes));
        let eta_month = match get_bits_u8::<274, 4>(bytes) {
            0 => None,
            m => Some(m),
        };
        let eta_day = match get_bits_u8::<278, 5>(bytes) {
            0 => None,
            d => Some(d),
        };
        let eta_hour = match get_bits_u8::<283, 5>(bytes) {
            24 => None,
            h => Some(h),
        };
        let eta_minute = match get_bits_u8::<288, 6>(bytes) {
            60 => None,
            m => Some(m),
        };
        let draught = parse_draught(get_bits_u8::<294, 8>(bytes));
        let destination = decode_text_fixed::<20>(bytes, 302, 20);
        // bit 422; reads 0 (=ready) on truncated 420/422-bit messages, matching the spec default
        let dte = get_bit::<422>(bytes);

        Self {
            message_type,
            repeat_indicator,
            mmsi,
            ais_version,
            imo,
            callsign,
            shipname,
            shiptype,
            to_bow,
            to_stern,
            to_port,
            to_starboard,
            epfd,
            eta_month,
            eta_day,
            eta_hour,
            eta_minute,
            draught,
            destination,
            dte,
        }
    }
}
