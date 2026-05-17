static TABLE: [ShipType; 256] = {
    let mut t = [ShipType::Unknown; 256];
    t[0] = ShipType::NotAvailable;
    let mut i = 1usize;
    while i <= 19 {
        t[i] = ShipType::Reserved;
        i += 1;
    }
    let mut i = 20usize;
    while i <= 29 {
        t[i] = ShipType::WingInGround;
        i += 1;
    }
    t[30] = ShipType::Fishing;
    t[31] = ShipType::Towing;
    t[32] = ShipType::TowingLarge;
    t[33] = ShipType::DredgingOrUnderwaterOps;
    t[34] = ShipType::DivingOps;
    t[35] = ShipType::MilitaryOps;
    t[36] = ShipType::Sailing;
    t[37] = ShipType::PleasureCraft;
    t[38] = ShipType::Reserved;
    t[39] = ShipType::Reserved;
    let mut i = 40usize;
    while i <= 49 {
        t[i] = ShipType::HighSpeedCraft;
        i += 1;
    }
    t[50] = ShipType::PilotVessel;
    t[51] = ShipType::SearchAndRescue;
    t[52] = ShipType::Tug;
    t[53] = ShipType::PortTender;
    t[54] = ShipType::AntiPollution;
    t[55] = ShipType::LawEnforcement;
    t[56] = ShipType::SpareLocal;
    t[57] = ShipType::SpareLocal;
    t[58] = ShipType::MedicalTransport;
    t[59] = ShipType::Noncombatant;
    let mut i = 60usize;
    while i <= 69 {
        t[i] = ShipType::Passenger;
        i += 1;
    }
    let mut i = 70usize;
    while i <= 79 {
        t[i] = ShipType::Cargo;
        i += 1;
    }
    let mut i = 80usize;
    while i <= 89 {
        t[i] = ShipType::Tanker;
        i += 1;
    }
    let mut i = 90usize;
    while i <= 99 {
        t[i] = ShipType::Other;
        i += 1;
    }
    t
};

#[derive(Debug, Clone, Copy)]
pub enum ShipType {
    NotAvailable,
    WingInGround,
    Fishing,
    Towing,
    TowingLarge,
    DredgingOrUnderwaterOps,
    DivingOps,
    MilitaryOps,
    Sailing,
    PleasureCraft,
    HighSpeedCraft,
    PilotVessel,
    SearchAndRescue,
    Tug,
    PortTender,
    AntiPollution,
    LawEnforcement,
    SpareLocal,
    MedicalTransport,
    Noncombatant,
    Passenger,
    Cargo,
    Tanker,
    Other,
    Reserved,
    Unknown,
}

impl From<u8> for ShipType {
    #[inline(always)]
    fn from(code: u8) -> Self {
        // Safety: table is size 256
        unsafe { *TABLE.get_unchecked(code as usize) }
    }
}
