use crate::Parser;
use crate::{
    AisReportType, RadioChannel, TalkerId,
    checksum::valid_checksum,
    errors::{MalformedReason, ParseError},
};

pub(crate) struct RawSentence<'a> {
    pub talker_id: TalkerId,
    pub ais_report_type: AisReportType,
    pub num_fragments: u8,
    pub fragment_num: u8,
    pub message_id: Option<u8>,
    pub radio_channel: Option<RadioChannel>,
    pub fill_bits: u8,
    pub armored: &'a [u8],
}

impl Parser {
    #[inline(always)]
    pub(crate) fn parse_nmea_header(sentence: &[u8]) -> Result<RawSentence<'_>, ParseError> {
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

        if !unsafe { valid_checksum(sentence, expected_checksum) } {
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

        let armored = &sentence[start_ais_message..sentence.len() - 2];

        Ok(RawSentence {
            talker_id,
            ais_report_type,
            num_fragments,
            fragment_num,
            message_id,
            radio_channel,
            fill_bits,
            armored,
        })
    }
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
