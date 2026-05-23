use crate::messages::{
    fields::ship_type::ShipType,
    utils::{AisStr, decode_text_fixed, get_bits_u8, get_bits_u16, get_bits_u32},
};

#[derive(Debug)]
pub enum ClassBStaticData {
    PartA(ClassBStaticDataPartA),
    PartB(ClassBStaticDataPartB),
}

#[derive(Debug)]
pub struct ClassBStaticDataPartA {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub shipname: AisStr<20>,
}

#[derive(Debug)]
pub struct ClassBStaticDataPartB {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub shiptype: ShipType,
    pub vendorid: AisStr<3>,
    pub model: u8,
    pub serial: u32,
    pub callsign: AisStr<7>,
    pub dimensions: Dimensions,
}

#[derive(Debug)]
pub enum Dimensions {
    Vessel {
        to_bow: Option<u16>,
        to_stern: Option<u16>,
        to_port: Option<u8>,
        to_starboard: Option<u8>,
    },
    Mothership {
        mmsi: u32,
    },
}

impl From<&[u8]> for ClassBStaticData {
    fn from(bytes: &[u8]) -> Self {
        let message_type = get_bits_u8::<0, 6>(bytes);
        let repeat_indicator = get_bits_u8::<6, 2>(bytes);
        let mmsi = get_bits_u32::<8, 30>(bytes);
        let partno = get_bits_u8::<38, 2>(bytes);

        match partno {
            0 => Self::PartA(ClassBStaticDataPartA {
                message_type,
                repeat_indicator,
                mmsi,
                shipname: decode_text_fixed::<20>(bytes, 40, 20),
            }),
            _ => {
                let shiptype = ShipType::from(get_bits_u8::<40, 8>(bytes));
                let vendorid = decode_text_fixed::<3>(bytes, 48, 3);
                let model = get_bits_u8::<66, 4>(bytes);
                let serial = get_bits_u32::<70, 20>(bytes);
                let callsign = decode_text_fixed::<7>(bytes, 90, 7);
                let dimensions = if is_auxiliary_mmsi(mmsi) {
                    Dimensions::Mothership {
                        mmsi: get_bits_u32::<132, 30>(bytes),
                    }
                } else {
                    Dimensions::Vessel {
                        to_bow: match get_bits_u16::<132, 9>(bytes) {
                            0 => None,
                            v => Some(v),
                        },
                        to_stern: match get_bits_u16::<141, 9>(bytes) {
                            0 => None,
                            v => Some(v),
                        },
                        to_port: match get_bits_u8::<150, 6>(bytes) {
                            0 => None,
                            v => Some(v),
                        },
                        to_starboard: match get_bits_u8::<156, 6>(bytes) {
                            0 => None,
                            v => Some(v),
                        },
                    }
                };

                Self::PartB(ClassBStaticDataPartB {
                    message_type,
                    repeat_indicator,
                    mmsi,
                    shiptype,
                    vendorid,
                    model,
                    serial,
                    callsign,
                    dimensions,
                })
            }
        }
    }
}

#[inline(always)]
fn is_auxiliary_mmsi(mmsi: u32) -> bool {
    (980_000_000..=989_999_999).contains(&mmsi)
}
