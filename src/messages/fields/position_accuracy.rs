static TABLE: [PositionAccuracy; 2] = [PositionAccuracy::UnaugmentedGnss, PositionAccuracy::Dgps];

#[derive(Debug, Clone, Copy)]
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
        // Saftey: single masked bit passed
        unsafe { *TABLE.get_unchecked(bit as usize) }
    }
}
