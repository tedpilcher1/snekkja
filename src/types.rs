// Hash function for the TalkerId lookup tables.
// We use bytes[0] + bytes[1]*2 (mod 256) rather than XOR because XOR is
// commutative — 'A'^'S' == 'S'^'A' — so AS and SA would always collide.
// This non-commutative form keeps all 10 known talker IDs collision-free
// with a single multiply and add rather than phf's SipHash overhead.
const fn talker_hash(a: u8, b: u8) -> usize {
    a.wrapping_add(b.wrapping_mul(2)) as usize
}

static TALKER_VALUES: [TalkerId; 256] = {
    let mut t = [TalkerId::Unknown; 256];
    t[talker_hash(b'A', b'B')] = TalkerId::AB;
    t[talker_hash(b'A', b'D')] = TalkerId::AD;
    t[talker_hash(b'A', b'I')] = TalkerId::AI;
    t[talker_hash(b'A', b'N')] = TalkerId::AN;
    t[talker_hash(b'A', b'R')] = TalkerId::AR;
    t[talker_hash(b'A', b'S')] = TalkerId::AS;
    t[talker_hash(b'A', b'T')] = TalkerId::AT;
    t[talker_hash(b'A', b'X')] = TalkerId::AX;
    t[talker_hash(b'B', b'S')] = TalkerId::BS;
    t[talker_hash(b'S', b'A')] = TalkerId::SA;
    t
};

// Parallel key table: stores the packed u16 of the expected input at each
// hash slot so we can reject inputs that hash to a valid slot by accident.
static TALKER_KEYS: [u16; 256] = {
    let mut t = [0u16; 256];
    t[talker_hash(b'A', b'B')] = u16::from_ne_bytes(*b"AB");
    t[talker_hash(b'A', b'D')] = u16::from_ne_bytes(*b"AD");
    t[talker_hash(b'A', b'I')] = u16::from_ne_bytes(*b"AI");
    t[talker_hash(b'A', b'N')] = u16::from_ne_bytes(*b"AN");
    t[talker_hash(b'A', b'R')] = u16::from_ne_bytes(*b"AR");
    t[talker_hash(b'A', b'S')] = u16::from_ne_bytes(*b"AS");
    t[talker_hash(b'A', b'T')] = u16::from_ne_bytes(*b"AT");
    t[talker_hash(b'A', b'X')] = u16::from_ne_bytes(*b"AX");
    t[talker_hash(b'B', b'S')] = u16::from_ne_bytes(*b"BS");
    t[talker_hash(b'S', b'A')] = u16::from_ne_bytes(*b"SA");
    t
};

static RADIO_TABLE: [RadioChannel; 256] = {
    let mut t = [RadioChannel::Unknown; 256];
    t[b'A' as usize] = RadioChannel::A;
    t[b'1' as usize] = RadioChannel::A;
    t[b'B' as usize] = RadioChannel::B;
    t[b'2' as usize] = RadioChannel::B;
    t
};

/// Talker ID
///
/// Identifies the type of device or station transmitting the data
#[derive(Debug, Copy, Clone)]
pub enum TalkerId {
    /// NMEA 4.0 Base AIS station
    AB,
    /// NMEA 4.0 Dependent AIS Base station
    AD,
    /// Mobile AIS station
    AI,
    /// NMEA 4.0 Aid to Navigation AIS station
    AN,
    /// NMEA 4.0 AIS Receiving station
    AR,
    /// NMEA 4.0 Limited Base station
    AS,
    /// NMEA 4.0 AIS Transmitting station
    AT,
    /// NMEA 4.0 Repeater AIS station
    AX,
    /// Base AIS station (deprecated)
    BS,
    /// NMEA 4.0 Physical Shore AIS station
    SA,
    Unknown,
}

impl From<&[u8; 2]> for TalkerId {
    #[inline(always)]
    fn from(bytes: &[u8; 2]) -> Self {
        let hash = talker_hash(bytes[0], bytes[1]);
        // Safety: produced hash modded by 256 (size of table)
        if unsafe { *TALKER_KEYS.get_unchecked(hash) } == u16::from_ne_bytes(*bytes) {
            unsafe { *TALKER_VALUES.get_unchecked(hash) }
        } else {
            Self::Unknown
        }
    }
}

/// NMEA sentence type of an AIS message
#[derive(Debug, Clone, Copy)]
pub enum AisReportType {
    /// Report from another ship
    VDM,
    /// Report from own ship
    VDO,
    Unknown,
}

impl From<&[u8; 3]> for AisReportType {
    #[inline(always)]
    fn from(bytes: &[u8; 3]) -> Self {
        match bytes {
            b"VDM" => Self::VDM,
            b"VDO" => Self::VDO,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RadioChannel {
    /// AIS Channel A is 161.975Mhz (87B)
    A,
    /// AIS Channel B is 162.025Mhz (88B)
    B,
    Unknown,
}

impl From<u8> for RadioChannel {
    #[inline(always)]
    fn from(byte: u8) -> Self {
        unsafe { *RADIO_TABLE.get_unchecked(byte as usize) }
    }
}
