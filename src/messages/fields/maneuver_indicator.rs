static TABLE: [Option<ManeuverIndicator>; 256] = {
    let mut t = [Some(ManeuverIndicator::Unknown); 256];
    t[0 as usize] = None;
    t[1 as usize] = Some(ManeuverIndicator::NoSpecialManeuver);
    t[2 as usize] = Some(ManeuverIndicator::SpecialManeuver);

    t
};

#[derive(Debug, Clone, Copy)]
pub enum ManeuverIndicator {
    /// 1
    NoSpecialManeuver,

    /// Such as regional passing arrangement
    ///
    /// 2
    SpecialManeuver,

    Unknown,
}

impl ManeuverIndicator {
    #[inline(always)]
    pub fn parse(crumb: u8) -> Option<Self> {
        // Safety: Table has size of 256
        unsafe { *TABLE.get_unchecked(crumb as usize) }
    }
}
