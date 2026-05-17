use crate::messages::{position_report::PositionReport, unarmor::Unarmored, utils::get_bits_u8};

mod fields;
mod position_report;
mod unarmor;
mod utils;

type AisMessageType = u8;

#[derive(Debug)]
pub enum AisMessage {
    PositionReport(PositionReport),
}

impl AisMessage {
    // returns option temporarily
    #[inline(always)]
    pub fn parse(bytes: &[u8], fill_bits: usize) -> (AisMessageType, Option<Self>) {
        let unarmored = Unarmored::unarmor(bytes, fill_bits);
        let message_type = AisMessageType::from(get_bits_u8(&unarmored.0, 0..=5));

        let message = match message_type {
            1..=3 => Some(AisMessage::PositionReport(PositionReport::from(unarmored))),
            _ => None,
        };

        (message_type, message)
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::AisMessage;

    #[test]
    fn can_parse_ais_message() {
        let bytes = b"177KQJ5000G?tO`K>RA1wUbN0TKH";
        let (ais_message_type, _ais_message) = AisMessage::parse(bytes, 0);

        assert_eq!(ais_message_type, 1)
    }

    #[test]
    fn can_parse_ais_message_2() {
        let bytes = b"H3`tNKA0ua`EA@v0BLu8t000000";
        let (ais_message_type, ais_message) = AisMessage::parse(bytes, 2);

        println!("{:?}", ais_message);

        assert_eq!(ais_message_type, 24)
    }
}
