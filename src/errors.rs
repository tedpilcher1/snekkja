use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ParseError {
    Malformed(MalformedReason),
    InvalidChecksum,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed(reason) => write!(f, "malformed sentence: {reason}"),
            Self::InvalidChecksum => write!(f, "invalid checksum"),
        }
    }
}

impl fmt::Display for MalformedReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingChecksumDelimiter => write!(f, "missing '*' checksum delimiter"),
            Self::SentenceTooShort => write!(f, "sentence too short"),
            Self::InvalidHexDigit => write!(f, "invalid hex digit"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MalformedReason {
    MissingChecksumDelimiter,
    SentenceTooShort,
    InvalidHexDigit,
}
