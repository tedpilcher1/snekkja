#[derive(Debug, Default)]
pub struct Parser;

impl Parser {
    pub fn parse(&self, sentence: &[u8]) -> AisSentence {
        let sentence = sentence.strip_prefix(&[b'!']).unwrap_or(sentence);
        let split_sentence = sentence.split(|char| *char == b'*').collect::<Vec<&[u8]>>();

        let sentence = split_sentence[0];

        let expected_checksum =
            u8::from_str_radix(std::str::from_utf8(&split_sentence[1]).unwrap(), 16).unwrap();

        if !valid_checksum(sentence, expected_checksum) {
            panic!("Invalid checksum")
        }

        let fields = sentence.split(|char| *char == b',').collect::<Vec<&[u8]>>();

        let (talker_id, report_type) = fields[0].split_at_checked(2).unwrap();
        let talker_id = TalkerId::from(talker_id);
        let report_type = ReportType::from(report_type);

        let num_fragments =
            numeric_from_ascii_char(u8::from_be_bytes([*fields[1].first().unwrap()]));

        let fragment_num =
            numeric_from_ascii_char(u8::from_be_bytes([*fields[2].first().unwrap()]));

        let message_id = match fields[3].is_empty() {
            true => None,
            false => Some(1),
        };

        let channel = match fields[4].is_empty() {
            true => None,
            false => Some(RadioChannelCode::from(
                *fields.get(4).unwrap().first().unwrap(),
            )),
        };

        let _data_payload = *fields.get(5).unwrap();

        let fill_bits = &fields[6];

        let fill_bits = numeric_from_ascii_char(*fill_bits.first().unwrap());
        assert!(fill_bits < 6);

        AisSentence {
            talker_id,
            report_type,
            num_fragments,
            fragment_num,
            message_id,
            channel,
            fill_bits,
        }
    }
}

#[derive(Debug)]
/// NMEA AIS sentence
pub struct AisSentence {
    pub talker_id: TalkerId,
    pub report_type: ReportType,
    pub num_fragments: u8,
    pub fragment_num: u8,
    pub message_id: Option<u8>,
    pub channel: Option<RadioChannelCode>,
    pub fill_bits: u8,
}

fn numeric_from_ascii_char(char: u8) -> u8 {
    let mut numeric = char - 48;

    if numeric > 40 {
        numeric -= 8
    }

    numeric
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
    /// Unknown talker ID
    Unknown,
}

impl From<&[u8]> for TalkerId {
    fn from(bytes: &[u8]) -> Self {
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
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
/// NMEA sentence type of an AIS message
pub enum ReportType {
    /// Report from another ship
    VDM,
    /// Report from own ship
    VDO,
    /// Unknown report type
    Unknown,
}

impl From<&[u8]> for ReportType {
    fn from(bytes: &[u8]) -> Self {
        match bytes {
            b"VDM" => Self::VDM,
            b"VDO" => Self::VDO,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum RadioChannelCode {
    /// AIS Channel A is 161.975Mhz (87B)
    A,
    /// AIS Channel B is 162.025Mhz (88B)
    B,
}

impl From<u8> for RadioChannelCode {
    fn from(byte: u8) -> Self {
        match byte {
            b'A' => RadioChannelCode::A,
            b'1' => RadioChannelCode::A,
            b'B' => RadioChannelCode::B,
            b'2' => RadioChannelCode::B,
            _ => panic!("Unknown byte detected: {:?}", byte),
        }
    }
}

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
