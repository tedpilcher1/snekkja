#[derive(Debug)]
pub enum ManeuverIndicator {
    /// 1
    NoSpecialManeuver,

    /// Such as regional passing arrangement
    ///
    /// 2
    SpecialManeuver,

    Unknown(u8),
}

impl ManeuverIndicator {
    #[inline(always)]
    pub fn parse(crumb: u8) -> Option<Self> {
        match crumb {
            0 => None,
            1 => Some(Self::NoSpecialManeuver),
            2 => Some(Self::SpecialManeuver),
            _ => Some(Self::Unknown(crumb)),
        }
    }
}
