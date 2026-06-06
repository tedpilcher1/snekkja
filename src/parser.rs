use crate::{
    AisFragments, SentenceHeader,
    errors::{MalformedReason, ParseError},
    fragments::Fragments,
    messages::{AisMessage, Unarmored},
};

#[derive(Debug, Default)]
pub struct Parser {
    unarmored_buf: Unarmored,
    fragments: Fragments,
}

impl Parser {
    /// Returns `None` while fragments are still being buffered, and `Some`
    /// once the message is fully assembled and decoded.
    pub fn parse_message(&mut self, sentence: &[u8]) -> Result<Option<AisMessage>, ParseError> {
        Ok(match self.parse(sentence)? {
            AisFragments::Complete { message, .. } => message,
            AisFragments::Incomplete(_) => None,
        })
    }

    /// Parse one NMEA sentence, buffering fragments until all parts of a
    /// multi-sentence message have arrived.
    pub fn parse(&mut self, sentence: &[u8]) -> Result<AisFragments, ParseError> {
        let raw = Self::parse_nmea_header(sentence)?;

        if raw.num_fragments == 1 {
            return Ok(AisFragments::Complete {
                header: SentenceHeader {
                    talker_id: raw.talker_id,
                    ais_report_type: raw.ais_report_type,
                    num_fragments: 1,
                    fragment_num: 1,
                    message_id: None,
                    radio_channel: raw.radio_channel,
                    fill_bits: raw.fill_bits,
                },
                message: AisMessage::parse(
                    &mut self.unarmored_buf,
                    raw.armored,
                    usize::from(raw.fill_bits),
                ),
            });
        }

        let message_id = raw.message_id.ok_or(ParseError::Malformed(
            MalformedReason::MissingFragmentMessageId,
        ))?;

        match self.fragments.insert(
            message_id,
            raw.fragment_num,
            raw.num_fragments,
            raw.armored,
            raw.fill_bits,
        )? {
            Some(payload) => Ok(AisFragments::Complete {
                header: SentenceHeader {
                    talker_id: raw.talker_id,
                    ais_report_type: raw.ais_report_type,
                    num_fragments: payload.num_fragments,
                    fragment_num: payload.num_fragments,
                    message_id: Some(message_id),
                    radio_channel: raw.radio_channel,
                    fill_bits: payload.fill_bits,
                },
                message: AisMessage::parse(
                    &mut self.unarmored_buf,
                    &payload.data[..payload.len],
                    usize::from(payload.fill_bits),
                ),
            }),

            None => Ok(AisFragments::Incomplete(SentenceHeader {
                talker_id: raw.talker_id,
                ais_report_type: raw.ais_report_type,
                num_fragments: raw.num_fragments,
                fragment_num: raw.fragment_num,
                message_id: Some(message_id),
                radio_channel: raw.radio_channel,
                fill_bits: raw.fill_bits,
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AisReportType, RadioChannel, TalkerId};

    fn make_packet(body: &[u8]) -> Vec<u8> {
        let checksum = body.iter().fold(0u8, |acc, &b| acc ^ b);
        let mut packet = b"!".to_vec();
        packet.extend_from_slice(body);
        packet.extend_from_slice(format!("*{checksum:02X}").as_bytes());
        packet
    }

    fn type5_frag1() -> Vec<u8> {
        make_packet(b"AIVDM,2,1,3,B,55?MbV02>H9c<H4eN10Ep4pT4@Dn2222220l1@O4i4i0Cm0CSlmD`880000,0")
    }
    fn type5_frag2() -> Vec<u8> {
        make_packet(b"AIVDM,2,2,3,B,00000000000,2")
    }

    #[test]
    fn parses_single_fragment() {
        let mut parser = Parser::default();
        let packet = b"!AIVDM,1,1,,B,177KQJ5000G?tO`K>RA1wUbN0TKH,0*5C";
        let header = match parser.parse(packet).unwrap() {
            AisFragments::Complete { header, .. } => header,
            AisFragments::Incomplete(_) => panic!("expected Complete"),
        };
        assert!(matches!(header.talker_id, TalkerId::AI));
        assert!(matches!(header.ais_report_type, AisReportType::VDM));
        assert_eq!(header.num_fragments, 1);
        assert_eq!(header.fragment_num, 1);
        assert!(header.message_id.is_none());
        assert!(matches!(header.radio_channel, Some(RadioChannel::B)));
        assert_eq!(header.fill_bits, 0);
    }

    #[test]
    fn parses_without_leading_bang() {
        let mut parser = Parser::default();
        assert!(
            parser
                .parse(b"AIVDM,1,1,,B,E>kb9O9aS@7PUh10dh19@;0Tah2cWrfP:l?M`00003vP100,0*01")
                .is_ok()
        );
    }

    #[test]
    fn multi_fragment_buffers_then_completes() {
        let mut parser = Parser::default();

        assert!(matches!(
            parser.parse(&type5_frag1()).unwrap(),
            AisFragments::Incomplete(_)
        ));

        match parser.parse(&type5_frag2()).unwrap() {
            AisFragments::Complete { header, message } => {
                assert_eq!(header.num_fragments, 2);
                assert_eq!(header.message_id, Some(3));
                assert!(matches!(message, Some(AisMessage::StaticVoyageData(_))));
            }
            AisFragments::Incomplete(_) => panic!("expected Complete"),
        };
    }

    #[test]
    fn multi_fragment_out_of_order() {
        let mut parser = Parser::default();
        assert!(matches!(
            parser.parse(&type5_frag2()).unwrap(),
            AisFragments::Incomplete(_)
        ));

        match parser.parse(&type5_frag1()).unwrap() {
            AisFragments::Complete { message, .. } => {
                assert!(matches!(message, Some(AisMessage::StaticVoyageData(_))))
            }
            AisFragments::Incomplete(_) => panic!("expected Complete"),
        }
    }

    #[test]
    fn slot_reset_on_conflicting_num_fragments() {
        let mut parser = Parser::default();
        parser
            .parse(&make_packet(b"AIVDM,3,1,1,A,0000000,0"))
            .unwrap();
        match parser
            .parse(&make_packet(b"AIVDM,2,1,1,A,0000000,0"))
            .unwrap()
        {
            AisFragments::Incomplete(s) => assert_eq!(s.num_fragments, 2),
            AisFragments::Complete { .. } => panic!("expected Incomplete after reset"),
        }
    }

    #[test]
    fn slot_reset_on_duplicate_fragment_num() {
        let mut parser = Parser::default();
        parser
            .parse(&make_packet(b"AIVDM,2,1,1,A,0000000,0"))
            .unwrap();
        assert!(matches!(
            parser
                .parse(&make_packet(b"AIVDM,2,1,1,A,0000000,0"))
                .unwrap(),
            AisFragments::Incomplete(_)
        ));
    }

    #[test]
    fn independent_message_ids_do_not_interfere() {
        let mut parser = Parser::default();
        parser
            .parse(&make_packet(b"AIVDM,2,1,1,A,0000000,0"))
            .unwrap();
        parser
            .parse(&make_packet(b"AIVDM,2,1,2,A,0000000,0"))
            .unwrap();
        assert!(matches!(
            parser
                .parse(&make_packet(b"AIVDM,2,2,1,A,0000000,0"))
                .unwrap(),
            AisFragments::Complete { .. }
        ));
        assert!(matches!(
            parser
                .parse(&make_packet(b"AIVDM,2,2,2,A,0000000,0"))
                .unwrap(),
            AisFragments::Complete { .. }
        ));
    }

    #[test]
    fn parse_message_returns_none_while_buffering() {
        let mut parser = Parser::default();
        assert!(parser.parse_message(&type5_frag1()).unwrap().is_none());
        assert!(parser.parse_message(&type5_frag2()).unwrap().is_some());
    }

    #[test]
    fn error_too_short() {
        let mut parser = Parser::default();
        for input in [b"".as_ref(), b"*01", b"AIVDM*01"] {
            assert!(matches!(
                parser.parse(input),
                Err(ParseError::Malformed(MalformedReason::SentenceTooShort))
            ));
        }
    }

    #[test]
    fn error_missing_checksum_delimiter() {
        let mut parser = Parser::default();
        assert!(matches!(
            parser.parse(b"!AIVDM,1,1,,B,data,0"),
            Err(ParseError::Malformed(
                MalformedReason::MissingChecksumDelimiter
            ))
        ));
    }

    #[test]
    fn error_truncated_checksum() {
        let mut parser = Parser::default();
        assert!(matches!(
            parser.parse(b"!AIVDM,1,1,,B,data,0*"),
            Err(ParseError::Malformed(
                MalformedReason::MissingChecksumDelimiter
            ))
        ));
    }

    #[test]
    fn error_invalid_hex_digit() {
        let mut parser = Parser::default();
        assert!(matches!(
            parser.parse(b"!AIVDM,1,1,,B,data,0*GG"),
            Err(ParseError::Malformed(MalformedReason::InvalidHexDigit))
        ));
    }

    #[test]
    fn error_checksum_mismatch() {
        let mut parser = Parser::default();
        assert!(matches!(
            parser.parse(b"!AIVDM,1,1,,B,E>kb9O9aS@7PUh10dh19@;0Tah2cWrfP:l?M`00003vP100,0*FF"),
            Err(ParseError::InvalidChecksum)
        ));
    }

    #[test]
    fn error_sentence_too_short() {
        let mut parser = Parser::default();
        assert!(matches!(
            parser.parse(&make_packet(b"AIVDM,1,1,")),
            Err(ParseError::Malformed(MalformedReason::SentenceTooShort))
        ));
    }

    #[test]
    fn unknown_talker_id_produces_variant() {
        let mut parser = Parser::default();
        let header = match parser
            .parse(&make_packet(b"XXVDM,1,1,,B,0000000,0"))
            .unwrap()
        {
            AisFragments::Complete { header, .. } => header,
            _ => panic!(),
        };
        assert!(matches!(header.talker_id, TalkerId::Unknown));
    }

    #[test]
    fn unknown_report_type_produces_variant() {
        let mut parser = Parser::default();
        let header = match parser
            .parse(&make_packet(b"AIZAP,1,1,,B,0000000,0"))
            .unwrap()
        {
            AisFragments::Complete { header, .. } => header,
            _ => panic!(),
        };
        assert!(matches!(header.ais_report_type, AisReportType::Unknown));
    }

    #[test]
    fn unknown_radio_channel_produces_variant() {
        let mut parser = Parser::default();
        let header = match parser
            .parse(&make_packet(b"AIVDM,1,1,,Z,0000000,0"))
            .unwrap()
        {
            AisFragments::Complete { header, .. } => header,
            _ => panic!(),
        };
        assert!(matches!(header.radio_channel, Some(RadioChannel::Unknown)));
    }
}
