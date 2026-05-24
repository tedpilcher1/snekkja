use crate::messages::{
    addressed_safety_message::AddressedSafetyMessage,
    aid_to_navigation_report::AidToNavigationReport,
    assignment_mode_command::AssignmentModeCommand, base_station_report::BaseStationReport,
    binary_acknowledge::BinaryAcknowledge, channel_management::ChannelManagement,
    class_b_position_report::ClassBPositionReport, class_b_static_data::ClassBStaticData,
    data_link_management::DataLinkManagement, group_assignment_command::GroupAssignmentCommand,
    interrogation::Interrogation, position_report::PositionReport,
    sar_aircraft_position_report::SarAircraftPositionReport, static_voyage_data::StaticVoyageData,
    utc_date_inquiry::UtcDateInquiry, utils::get_bits,
};

pub mod addressed_safety_message;
pub mod aid_to_navigation_report;
pub mod assignment_mode_command;
pub mod base_station_report;
pub mod binary_acknowledge;
pub mod channel_management;
pub mod class_b_position_report;
pub mod class_b_static_data;
pub mod data_link_management;
mod fields;
pub mod group_assignment_command;
pub mod interrogation;
pub mod position_report;
pub mod sar_aircraft_position_report;
pub mod static_voyage_data;
mod unarmor;
pub mod utc_date_inquiry;
mod utils;

pub(crate) use unarmor::Unarmored;

type AisMessageType = u8;

#[derive(Debug)]
pub enum AisMessage {
    PositionReport(PositionReport),
    BaseStationReport(BaseStationReport),
    StaticVoyageData(StaticVoyageData),
    ClassBPositionReport(ClassBPositionReport),
    ClassBStaticData(ClassBStaticData),
    AidToNavigationReport(AidToNavigationReport),
    BinaryAcknowledge(BinaryAcknowledge),
    SarAircraftPositionReport(SarAircraftPositionReport),
    UtcDateInquiry(UtcDateInquiry),
    AddressedSafetyMessage(AddressedSafetyMessage),
    Interrogation(Interrogation),
    DataLinkManagement(DataLinkManagement),
    AssignmentModeCommand(AssignmentModeCommand),
    ChannelManagement(ChannelManagement),
    GroupAssignmentCommand(GroupAssignmentCommand),
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

        let message_type = AisMessageType::from(get_bits::<u8, 0, 6>(bytes));

        let message = match message_type {
            1..=3 => Some(AisMessage::PositionReport(PositionReport::from(bytes))),
            4 | 11 => Some(AisMessage::BaseStationReport(BaseStationReport::from(
                bytes,
            ))),
            5 => Some(AisMessage::StaticVoyageData(StaticVoyageData::from(bytes))),
            7 | 13 => Some(AisMessage::BinaryAcknowledge(BinaryAcknowledge::from(
                bytes,
            ))),
            9 => Some(AisMessage::SarAircraftPositionReport(
                SarAircraftPositionReport::from(bytes),
            )),
            10 => Some(AisMessage::UtcDateInquiry(UtcDateInquiry::from(bytes))),
            12 => Some(AisMessage::AddressedSafetyMessage(
                AddressedSafetyMessage::from(bytes),
            )),
            15 => Some(AisMessage::Interrogation(Interrogation::from(bytes))),
            16 => Some(AisMessage::AssignmentModeCommand(
                AssignmentModeCommand::from(bytes),
            )),
            18 => Some(AisMessage::ClassBPositionReport(
                ClassBPositionReport::from(bytes),
            )),
            20 => Some(AisMessage::DataLinkManagement(DataLinkManagement::from(
                bytes,
            ))),
            21 => Some(AisMessage::AidToNavigationReport(
                AidToNavigationReport::from(bytes),
            )),
            22 => Some(AisMessage::ChannelManagement(ChannelManagement::from(
                bytes,
            ))),
            23 => Some(AisMessage::GroupAssignmentCommand(
                GroupAssignmentCommand::from(bytes),
            )),
            24 => Some(AisMessage::ClassBStaticData(ClassBStaticData::from(bytes))),
            _ => None,
        };

        (message_type, message)
    }
}
