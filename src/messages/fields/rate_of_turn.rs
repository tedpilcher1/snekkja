#[derive(Debug, Clone, PartialEq)]
pub enum RateOfTurn {
    /// No turn information available
    ///
    /// -128
    NotAvailable,

    /// Turning right at more than 5deg/30s (No TI available)
    ///
    /// 127
    TurningRight,

    /// Turning left at more than 5deg/30s (No TI available)
    ///
    /// -127
    TurningLeft,

    Degrees(f64),
}

impl RateOfTurn {
    #[inline(always)]
    pub fn parse(raw: i8) -> Option<Self> {
        match raw {
            -128 => Some(Self::NotAvailable),
            127 => Some(Self::TurningRight),
            -127 => Some(Self::TurningLeft),
            0 => None,
            _ => {
                let sign = if raw > 0 { 1.0 } else { -1.0 };
                let val = raw as f64 / 4.733;
                Some(Self::Degrees(sign * val * val))
            }
        }
    }
}
