use crate::messages::{
    base_station_report::BaseStationReport, position_report::PositionReport,
    static_voyage_data::StaticVoyageData, utils::get_bits_u8,
};

pub mod base_station_report;
mod fields;
pub mod position_report;
pub mod static_voyage_data;
mod unarmor;
mod utils;

pub(crate) use unarmor::Unarmored;

type AisMessageType = u8;

#[derive(Debug)]
pub enum AisMessage {
    PositionReport(PositionReport),
    BaseStationReport(BaseStationReport),
    StaticVoyageData(StaticVoyageData),
}

impl AisMessage {
    #[inline(always)]
    pub(crate) fn parse(
        unarmored_buf: &mut Unarmored,
        bytes: &[u8],
        fill_bits: usize,
    ) -> (AisMessageType, Option<Self>) {
        unsafe {
            unarmored_buf.unarmor(bytes, fill_bits);
        }

        let bytes = unarmored_buf.as_slice();

        let message_type = AisMessageType::from(get_bits_u8::<0, 6>(bytes));

        let message = match message_type {
            1..=3 => Some(AisMessage::PositionReport(PositionReport::from(bytes))),
            4 => Some(AisMessage::BaseStationReport(BaseStationReport::from(
                bytes,
            ))),
            5 => Some(AisMessage::StaticVoyageData(StaticVoyageData::from(bytes))),
            _ => None,
        };

        (message_type, message)
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::{AisMessage, Unarmored};

    #[test]
    fn can_parse_ais_message_type_1() {
        let mut unarmored_buf = Unarmored::new();
        let bytes = b"177KQJ5000G?tO`K>RA1wUbN0TKH";

        let (ais_message_type, _ais_message) = AisMessage::parse(&mut unarmored_buf, bytes, 0);

        assert_eq!(ais_message_type, 1)
    }
}
