static TABLE: [Option<NavigationStatus>; 256] = {
    let mut t = [Some(NavigationStatus::Unknown); 256];
    t[0_usize] = Some(NavigationStatus::UnderWayUsingEngine);
    t[1_usize] = Some(NavigationStatus::AtAnchor);
    t[2_usize] = Some(NavigationStatus::NotUnderCommand);
    t[3_usize] = Some(NavigationStatus::RestrictedManouverability);
    t[4_usize] = Some(NavigationStatus::ConstrainedByDraught);
    t[5_usize] = Some(NavigationStatus::Moored);
    t[6_usize] = Some(NavigationStatus::Aground);
    t[7_usize] = Some(NavigationStatus::EngagedInFishing);
    t[8_usize] = Some(NavigationStatus::UnderWaySailing);
    t[9_usize] = Some(NavigationStatus::ReservedForFutureAmendmentOfNavigationalStatusForHSC);
    t[10_usize] = Some(NavigationStatus::ReservedForFutureAmendmentOfNavigationalStatusForWIG);
    t[11_usize] = Some(NavigationStatus::PowerDrivenVesselTowingAstern);
    t[12_usize] = Some(NavigationStatus::PowerDrivenVesselPushingAheadOrTowingAlongside);
    t[13_usize] = Some(NavigationStatus::ReservedForFutureUse);
    t[14_usize] = Some(NavigationStatus::AisSartIsActive);
    t[15_usize] = None;
    t
};

#[derive(Debug, Clone, Copy)]
pub enum NavigationStatus {
    UnderWayUsingEngine,
    AtAnchor,
    NotUnderCommand,
    RestrictedManouverability,
    ConstrainedByDraught,
    Moored,
    Aground,
    EngagedInFishing,
    UnderWaySailing,
    ReservedForFutureAmendmentOfNavigationalStatusForHSC,
    ReservedForFutureAmendmentOfNavigationalStatusForWIG,
    PowerDrivenVesselTowingAstern,
    PowerDrivenVesselPushingAheadOrTowingAlongside,
    ReservedForFutureUse,
    AisSartIsActive,
    Unknown,
}

impl NavigationStatus {
    #[inline(always)]
    pub fn parse(nibble: u8) -> Option<Self> {
        // Safety: Table has size of 256
        unsafe { *TABLE.get_unchecked(nibble as usize) }
    }
}
