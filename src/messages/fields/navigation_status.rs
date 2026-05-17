#[derive(Debug)]
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
    Unknown(u8),
}

impl NavigationStatus {
    #[inline(always)]
    pub fn parse(nibble: u8) -> Option<Self> {
        match nibble {
            0 => Some(Self::UnderWayUsingEngine),
            1 => Some(Self::AtAnchor),
            2 => Some(Self::NotUnderCommand),
            3 => Some(Self::RestrictedManouverability),
            4 => Some(Self::ConstrainedByDraught),
            5 => Some(Self::Moored),
            6 => Some(Self::Aground),
            7 => Some(Self::EngagedInFishing),
            8 => Some(Self::UnderWaySailing),
            9 => Some(Self::ReservedForFutureAmendmentOfNavigationalStatusForHSC),
            10 => Some(Self::ReservedForFutureAmendmentOfNavigationalStatusForWIG),
            11 => Some(Self::PowerDrivenVesselTowingAstern),
            12 => Some(Self::PowerDrivenVesselPushingAheadOrTowingAlongside),
            13 => Some(Self::ReservedForFutureUse),
            14 => Some(Self::AisSartIsActive),
            15 => None,
            _ => Some(Self::Unknown(nibble)),
        }
    }
}
