static TABLE: [Option<NavigationStatus>; 256] = {
    let mut t = [Some(NavigationStatus::Unknown); 256];
    t[0 as usize] = Some(NavigationStatus::UnderWayUsingEngine);
    t[1 as usize] = Some(NavigationStatus::AtAnchor);
    t[2 as usize] = Some(NavigationStatus::NotUnderCommand);
    t[3 as usize] = Some(NavigationStatus::RestrictedManouverability);
    t[4 as usize] = Some(NavigationStatus::ConstrainedByDraught);
    t[5 as usize] = Some(NavigationStatus::Moored);
    t[6 as usize] = Some(NavigationStatus::Aground);
    t[7 as usize] = Some(NavigationStatus::EngagedInFishing);
    t[8 as usize] = Some(NavigationStatus::UnderWaySailing);
    t[9 as usize] = Some(NavigationStatus::ReservedForFutureAmendmentOfNavigationalStatusForHSC);
    t[10 as usize] = Some(NavigationStatus::ReservedForFutureAmendmentOfNavigationalStatusForWIG);
    t[11 as usize] = Some(NavigationStatus::PowerDrivenVesselTowingAstern);
    t[12 as usize] = Some(NavigationStatus::PowerDrivenVesselPushingAheadOrTowingAlongside);
    t[13 as usize] = Some(NavigationStatus::ReservedForFutureUse);
    t[14 as usize] = Some(NavigationStatus::AisSartIsActive);
    t[15 as usize] = None;
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
