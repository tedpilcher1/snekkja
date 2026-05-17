use crate::{
    errors::{MalformedReason, ParseError},
    messages::AisMessage,
};

pub mod errors;
pub mod messages;

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn parse(&self, sentence: &[u8]) -> Result<AisSentence, ParseError> {
        let sentence = sentence.strip_prefix(b"!").unwrap_or(sentence);

        if sentence.len() < 15 {
            return Err(ParseError::Malformed(MalformedReason::SentenceTooShort));
        }

        let star_pos = sentence.len() - 3;

        if sentence[star_pos] != b'*' {
            return Err(ParseError::Malformed(
                MalformedReason::MissingChecksumDelimiter,
            ));
        }

        let hi = sentence
            .get(star_pos + 1)
            .copied()
            .ok_or(ParseError::Malformed(MalformedReason::SentenceTooShort))?;

        let lo = sentence
            .get(star_pos + 2)
            .copied()
            .ok_or(ParseError::Malformed(MalformedReason::SentenceTooShort))?;

        let expected_checksum = parse_hex_byte(hi, lo)?;

        let sentence = &sentence[..star_pos];

        if !valid_checksum(sentence, expected_checksum) {
            return Err(ParseError::InvalidChecksum);
        }

        // bytes 0 & 1 = TalkerId
        let talker_id = TalkerId::from(
            <&[u8; 2]>::try_from(&sentence[0..2])
                .map_err(|_| ParseError::Malformed(MalformedReason::SentenceTooShort))?,
        );

        // bytes 2,3,4 = Ais report type
        let ais_report_type = AisReportType::from(
            <&[u8; 3]>::try_from(&sentence[2..5])
                .map_err(|_| ParseError::Malformed(MalformedReason::SentenceTooShort))?,
        );

        // byte 6 = number of fragments
        let num_fragments = numeric_from_ascii(sentence[6]);

        // byte 8 = fragment number
        let fragment_num = numeric_from_ascii(sentence[8]);

        let (message_id, radio_channel, start_ais_message) = if num_fragments > 1 {
            if sentence.len() < 13 {
                return Err(ParseError::Malformed(MalformedReason::SentenceTooShort));
            }

            // byte 10 = message_id
            let message_id = numeric_from_ascii(sentence[10]);

            // byte 12 = radio channel
            let radio_channel = if sentence[12] == b',' {
                None
            } else {
                Some(RadioChannel::from(sentence[12]))
            };

            (Some(message_id), radio_channel, 14)
        } else {
            // byte 11 = radio channel
            let (radio_channel, start_ais_message) = if sentence[11] == b',' {
                (None, 12)
            } else {
                (Some(RadioChannel::from(sentence[11])), 13)
            };

            (None, radio_channel, start_ais_message)
        };

        let fill_bits = sentence
            .last()
            .copied()
            .map(numeric_from_ascii)
            .ok_or(ParseError::Malformed(MalformedReason::SentenceTooShort))?;

        let (message_type, message) = AisMessage::parse(
            &sentence[start_ais_message..sentence.len() - 2],
            usize::from(fill_bits),
        );

        Ok(AisSentence {
            talker_id,
            ais_report_type,
            num_fragments,
            fragment_num,
            message_id,
            radio_channel,
            fill_bits,
            message_type,
            message,
        })
    }
}

#[derive(Debug)]
pub struct AisSentence {
    pub talker_id: TalkerId,
    pub ais_report_type: AisReportType,
    pub num_fragments: u8,
    pub fragment_num: u8,
    pub message_id: Option<u8>,
    pub radio_channel: Option<RadioChannel>,
    pub fill_bits: u8,
    pub message_type: u8,
    pub message: Option<AisMessage>,
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
fn parse_hex_byte(hi: u8, lo: u8) -> Result<u8, ParseError> {
    fn nibble(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'A'..=b'F' => Some(b - b'A' + 10),
            b'a'..=b'f' => Some(b - b'a' + 10),
            _ => None,
        }
    }

    let hi = nibble(hi).ok_or(ParseError::Malformed(MalformedReason::InvalidHexDigit))?;
    let lo = nibble(lo).ok_or(ParseError::Malformed(MalformedReason::InvalidHexDigit))?;

    Ok(hi << 4 | lo)
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
    Unknown,
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
            _ => Self::Unknown,
        }
    }
}

/// NMEA sentence type of an AIS message
#[derive(Debug)]
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

#[derive(Debug)]
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
        match byte {
            b'A' | b'1' => RadioChannel::A,
            b'B' | b'2' => RadioChannel::B,
            _ => RadioChannel::Unknown,
        }
    }
}

#[inline(always)]
fn valid_checksum(sentence: &[u8], expected_checksum: u8) -> bool {
    sentence.iter().fold(0u8, |acc, &item| acc ^ item) == expected_checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_packet(body: &[u8]) -> Vec<u8> {
        let checksum = body.iter().fold(0u8, |acc, &b| acc ^ b);
        let mut packet = b"!".to_vec();
        packet.extend_from_slice(body);
        packet.extend_from_slice(format!("*{checksum:02X}").as_bytes());
        packet
    }

    #[test]
    fn parses_single_fragment() {
        let parser = Parser;
        let packet = b"!AIVDM,1,1,,B,177KQJ5000G?tO`K>RA1wUbN0TKH,0*5C";
        let s = parser.parse(packet).unwrap();

        assert!(matches!(s.talker_id, TalkerId::AI));
        assert!(matches!(s.ais_report_type, AisReportType::VDM));
        assert_eq!(s.num_fragments, 1);
        assert_eq!(s.fragment_num, 1);
        assert!(s.message_id.is_none());
        assert!(matches!(s.radio_channel, Some(RadioChannel::B)));
        assert_eq!(s.fill_bits, 0);
    }

    #[test]
    fn parses_without_leading_bang() {
        let parser = Parser;
        let result =
            parser.parse(b"AIVDM,1,1,,B,E>kb9O9aS@7PUh10dh19@;0Tah2cWrfP:l?M`00003vP100,0*01");
        assert!(result.is_ok());
    }

    #[test]
    fn parses_multi_fragment() {
        let parser = Parser;
        let packet = make_packet(b"AIVDM,2,1,3,B,0000000,0");
        let s = parser.parse(&packet).unwrap();

        assert_eq!(s.num_fragments, 2);
        assert_eq!(s.fragment_num, 1);
        assert_eq!(s.message_id, Some(3));
        assert!(matches!(s.radio_channel, Some(RadioChannel::B)));
    }

    #[test]
    fn error_too_short() {
        let parser = Parser;
        for input in [b"".as_ref(), b"*01", b"AIVDM*01"] {
            let result = parser.parse(input);
            assert!(
                matches!(
                    result,
                    Err(ParseError::Malformed(MalformedReason::SentenceTooShort))
                ),
                "expected SentenceTooShort for input {:?}",
                input
            );
        }
    }

    #[test]
    fn error_missing_checksum_delimiter() {
        let parser = Parser;
        let result = parser.parse(b"!AIVDM,1,1,,B,data,0");
        assert!(matches!(
            result,
            Err(ParseError::Malformed(
                MalformedReason::MissingChecksumDelimiter
            ))
        ));
    }

    #[test]
    fn error_truncated_checksum() {
        let parser = Parser;
        let result = parser.parse(b"!AIVDM,1,1,,B,data,0*");
        assert!(matches!(
            result,
            Err(ParseError::Malformed(
                MalformedReason::MissingChecksumDelimiter
            ))
        ));
    }

    #[test]
    fn error_invalid_hex_digit() {
        let parser = Parser;
        let result = parser.parse(b"!AIVDM,1,1,,B,data,0*GG");
        assert!(matches!(
            result,
            Err(ParseError::Malformed(MalformedReason::InvalidHexDigit))
        ));
    }

    #[test]
    fn error_checksum_mismatch() {
        let parser = Parser;
        let result =
            parser.parse(b"!AIVDM,1,1,,B,E>kb9O9aS@7PUh10dh19@;0Tah2cWrfP:l?M`00003vP100,0*FF");
        assert!(matches!(result, Err(ParseError::InvalidChecksum)));
    }

    #[test]
    fn error_sentence_too_short() {
        let parser = Parser;
        let packet = make_packet(b"AIVDM,1,1,");
        let result = parser.parse(&packet);
        assert!(matches!(
            result,
            Err(ParseError::Malformed(MalformedReason::SentenceTooShort))
        ));
    }

    #[test]
    fn unknown_talker_id_produces_variant() {
        let parser = Parser;
        let packet = make_packet(b"XXVDM,1,1,,B,0000000,0");
        let s = parser.parse(&packet).unwrap();
        assert!(matches!(s.talker_id, TalkerId::Unknown));
    }

    #[test]
    fn unknown_report_type_produces_variant() {
        let parser = Parser;
        let packet = make_packet(b"AIZAP,1,1,,B,0000000,0");
        let s = parser.parse(&packet).unwrap();
        assert!(matches!(s.ais_report_type, AisReportType::Unknown));
    }

    #[test]
    fn unknown_radio_channel_produces_variant() {
        let parser = Parser;
        let packet = make_packet(b"AIVDM,1,1,,Z,0000000,0");
        let s = parser.parse(&packet).unwrap();
        assert!(matches!(s.radio_channel, Some(RadioChannel::Unknown)));
    }
}
