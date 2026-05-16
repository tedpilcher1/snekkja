use memchr::memchr;

#[derive(Debug, Default)]
pub struct Parser;

impl Parser {
    pub fn parse(&self, sentence: &[u8]) -> AisSentence {
        let sentence = sentence.strip_prefix(&[b'!']).unwrap_or(sentence);
        let star_pos = memchr(b'*', sentence).unwrap();
        let expected_checksum = parse_hex_byte(sentence[star_pos + 1], sentence[star_pos + 2]);
        let sentence = &sentence[..star_pos];

        if !valid_checksum(sentence, expected_checksum) {
            panic!("Invalid checksum")
        }

        // bytes 0 & 1 = TalkerId
        let talker_id = TalkerId::from(&sentence[0..2].try_into().unwrap());

        // bytes 2,3,4 = Ais report type
        let ais_report_type = AisReportType::from(&sentence[2..5].try_into().unwrap());

        // byte 5 = ','

        // byte 6 = number of fragments
        let num_fragments = numeric_from_ascii(sentence[6].try_into().unwrap());

        // byte 7 = ','

        // byte 8 = fragment number
        let fragment_num = numeric_from_ascii(sentence[8].try_into().unwrap());

        // byte 9 = ','

        let (message_id, radio_channel, _idx_last_byte) = if num_fragments > 1 {
            // byte 10 = message_id
            let messsage_id = numeric_from_ascii(sentence[10].try_into().unwrap());

            // byte 11 = ','

            // byte 12 = radio channel
            let byte_12 = &sentence[12];

            let radio_channel = if *byte_12 == b',' {
                None
            } else {
                Some(RadioChannel::from(byte_12))
            };

            (Some(messsage_id), radio_channel, 12_usize)
        } else {
            // byte 10 = ','

            // byte 11 = radio channel
            let byte_11 = &sentence[11];

            let radio_channel = if *byte_11 == b',' {
                None
            } else {
                Some(RadioChannel::from(byte_11))
            };

            (None, radio_channel, 11_usize)
        };

        let fill_bits = numeric_from_ascii(sentence[sentence.len() - 1].try_into().unwrap());

        AisSentence {
            talker_id,
            ais_report_type,
            num_fragments,
            fragment_num,
            message_id,
            radio_channel,
            fill_bits,
        }
    }
}

#[derive(Debug)]
/// NMEA AIS sentence
pub struct AisSentence {
    pub talker_id: TalkerId,
    pub ais_report_type: AisReportType,
    pub num_fragments: u8,
    pub fragment_num: u8,
    pub message_id: Option<u8>,
    pub radio_channel: Option<RadioChannel>,
    pub fill_bits: u8,
}

#[inline(always)]
fn numeric_from_ascii(char: u8) -> u8 {
    let mut numeric = char - 48;

    if numeric > 40 {
        numeric -= 8
    }

    numeric
}

#[inline(always)]
fn parse_hex_byte(hi: u8, lo: u8) -> u8 {
    fn nibble(b: u8) -> u8 {
        if b >= b'A' { b - b'A' + 10 } else { b - b'0' }
    }
    nibble(hi) << 4 | nibble(lo)
}

/// Talker ID
///
/// Identifies the type of device or station transmitting the data
#[derive(Debug)]
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
}

impl From<&[u8; 2]> for TalkerId {
    #[inline(always)]
    fn from(bytes: &[u8; 2]) -> Self {
        match bytes {
            b"AB" => Self::AB,
            b"AD" => Self::AD,
            b"AI" => Self::AI,
            b"AN" => Self::AN,
            b"AR" => Self::AR,
            b"AS" => Self::AS,
            b"AT" => Self::AT,
            b"AX" => Self::AX,
            b"BS" => Self::BS,
            b"SA" => Self::SA,
            _ => panic!("Unknown talker id: {:?}", bytes),
        }
    }
}

#[derive(Debug)]
/// NMEA sentence type of an AIS message
pub enum AisReportType {
    /// Report from another ship
    VDM,
    /// Report from own ship
    VDO,
}

impl From<&[u8; 3]> for AisReportType {
    #[inline(always)]
    fn from(bytes: &[u8; 3]) -> Self {
        match bytes {
            b"VDM" => Self::VDM,
            b"VDO" => Self::VDO,
            _ => panic!("Unknown report type: {:?}", bytes),
        }
    }
}

#[derive(Debug)]
pub enum RadioChannel {
    /// AIS Channel A is 161.975Mhz (87B)
    A,
    /// AIS Channel B is 162.025Mhz (88B)
    B,
}

impl From<&u8> for RadioChannel {
    #[inline(always)]
    fn from(byte: &u8) -> Self {
        match byte {
            b'A' => RadioChannel::A,
            b'1' => RadioChannel::A,
            b'B' => RadioChannel::B,
            b'2' => RadioChannel::B,
            _ => panic!("Unknown byte detected: {:?}", byte),
        }
    }
}

#[inline(always)]
fn valid_checksum(sentence: &[u8], expected_checksum: u8) -> bool {
    let received_checksum = sentence.iter().fold(0u8, |acc, &item| acc ^ item);

    if expected_checksum != received_checksum {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let parser = Parser::default();

        let packet = b"!AIVDM,1,1,,B,E>kb9O9aS@7PUh10dh19@;0Tah2cWrfP:l?M`00003vP100,0*01";

        let sentence = parser.parse(packet);

        println!("{sentence:?}")
    }
}
