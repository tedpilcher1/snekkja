use crate::messages::utils::{get_bit, get_bits_u8, get_bits_u16};

#[derive(Debug)]
pub enum RadioStatus {
    Sotdma(SotdmaMessage),
    Itdma(ItdmaMessage),
}

#[derive(Debug)]
pub struct SotdmaMessage {
    pub sync_state: SyncState,
    pub slot_timeout: u8,
    pub sub_message: SubMessage,
}

#[derive(Debug)]
pub struct ItdmaMessage {
    pub sync_state: SyncState,
    pub slot_increment: u16,
    pub num_slots: u8,
    pub keep: bool,
}

#[derive(Debug)]
pub enum SyncState {
    UtcDirect,
    UtcIndirect,
    BaseStation,
    NumberOfReceivedStations,
    Unknown(u8),
}

#[derive(Debug)]
pub enum SubMessage {
    SlotOffset(u16),
    UtcHourAndMinute(u8, u8),
    SlotNumber(u16),
    ReceivedStations(u16),
}

impl RadioStatus {
    #[inline(always)]
    pub fn parse(bytes: &[u8], start: usize, message_type: u8) -> Self {
        match message_type {
            3 => Self::Itdma(ItdmaMessage::parse(bytes, start)),
            _ => Self::Sotdma(SotdmaMessage::parse(bytes, start)),
        }
    }
}

impl SyncState {
    #[inline(always)]
    fn parse(raw: u8) -> Self {
        match raw {
            0 => Self::UtcDirect,
            1 => Self::UtcIndirect,
            2 => Self::BaseStation,
            3 => Self::NumberOfReceivedStations,
            _ => Self::Unknown(raw),
        }
    }
}

impl SotdmaMessage {
    #[inline(always)]
    fn parse(bytes: &[u8], start: usize) -> Self {
        let sync_state = SyncState::parse(get_bits_u8(bytes, start..=start + 1));
        let slot_timeout = get_bits_u8(bytes, start + 2..=start + 4);
        let sub_message = SubMessage::parse(bytes, start + 5, slot_timeout);
        Self {
            sync_state,
            slot_timeout,
            sub_message,
        }
    }
}

impl ItdmaMessage {
    #[inline(always)]
    fn parse(bytes: &[u8], start: usize) -> Self {
        let sync_state = SyncState::parse(get_bits_u8(bytes, start..=start + 1));
        let slot_increment = get_bits_u16(bytes, start + 2..=start + 14);
        let num_slots = get_bits_u8(bytes, start + 15..=start + 17);
        let keep = get_bit(bytes, start + 18);
        Self {
            sync_state,
            slot_increment,
            num_slots,
            keep,
        }
    }
}

impl SubMessage {
    #[inline(always)]
    fn parse(bytes: &[u8], start: usize, slot_timeout: u8) -> Self {
        match slot_timeout {
            0 => Self::SlotOffset(get_bits_u16(bytes, start..=start + 13)),
            1 => {
                let hour = get_bits_u8(bytes, start..=start + 4);
                let minute = get_bits_u8(bytes, start + 6..=start + 11);
                Self::UtcHourAndMinute(hour, minute)
            }
            2 | 4 | 6 => Self::SlotNumber(get_bits_u16(bytes, start..=start + 13)),
            _ => Self::ReceivedStations(get_bits_u16(bytes, start..=start + 13)),
        }
    }
}
