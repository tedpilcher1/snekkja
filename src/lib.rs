pub(crate) mod checksum;
pub mod errors;
mod messages;
mod parser;
mod types;

pub use messages::AisMessage;
pub use messages::{
    base_station_report::BaseStationReport, position_report::PositionReport,
    static_voyage_data::StaticVoyageData,
};
pub use parser::Parser;
pub use types::{AisReportType, RadioChannel, TalkerId};

// TAIL_EXTRACT[k][j] selects from the last 16 bytes of the input such that:
//   result[j] = input[i + j]  for j < k (the k tail chars, right-aligned in last16)
//   result[j] = fallback      for j >= k (index 255 is out-of-range for vqtbx1q_u8)
const TAIL_EXTRACT: [[u8; 16]; 16] = {
    let mut t = [[255u8; 16]; 16];
    let mut k = 1usize;
    while k < 16 {
        let mut j = 0usize;
        while j < k {
            t[k][j] = (16 - k + j) as u8;
            j += 1;
        }
        k += 1;
    }
    t
};

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
