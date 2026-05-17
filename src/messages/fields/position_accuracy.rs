#[derive(Debug)]
pub enum PositionAccuracy {
    /// DGPS-quality fix with an accuracy of < 10ms
    ///
    /// 1
    Dgps,
    /// Indicates an unaugmented GNSS fix with accuracy > 10m
    ///
    /// 0, the default
    UnaugmentedGnss,
}

impl From<u8> for PositionAccuracy {
    #[inline(always)]
    fn from(bit: u8) -> Self {
        match bit {
            0 => Self::UnaugmentedGnss,
            1 => Self::Dgps,
            _ => unreachable!(),
        }
    }
}
