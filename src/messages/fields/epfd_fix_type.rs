static TABLE: [EpfdFixType; 16] = [
    EpfdFixType::Undefined,
    EpfdFixType::Gps,
    EpfdFixType::Glonass,
    EpfdFixType::CombinedGpsGlonass,
    EpfdFixType::LoranC,
    EpfdFixType::Chayka,
    EpfdFixType::IntegratedNavigationSystem,
    EpfdFixType::Surveyed,
    EpfdFixType::Galileo,
    EpfdFixType::Reserved,
    EpfdFixType::Reserved,
    EpfdFixType::Reserved,
    EpfdFixType::Reserved,
    EpfdFixType::Reserved,
    EpfdFixType::Reserved,
    EpfdFixType::InternalGnss,
];

#[derive(Debug, Clone, Copy)]
pub enum EpfdFixType {
    Undefined,
    Gps,
    Glonass,
    CombinedGpsGlonass,
    LoranC,
    Chayka,
    IntegratedNavigationSystem,
    Surveyed,
    Galileo,
    Reserved,
    InternalGnss,
}

impl From<u8> for EpfdFixType {
    #[inline(always)]
    fn from(nibble: u8) -> Self {
        // Safety: 4-bit field, nibble is always 0-15
        unsafe { *TABLE.get_unchecked(nibble as usize) }
    }
}
