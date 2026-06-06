use crate::errors::{MalformedReason, ParseError};

const MAX_FRAGMENTS: usize = 9;
const MAX_ARMORED_LEN: usize = 82;
const NUM_MESSAGE_SLOTS: usize = 10;
pub(crate) const MAX_ASSEMBLED_LEN: usize = MAX_FRAGMENTS * MAX_ARMORED_LEN;

#[derive(Debug)]
struct FragmentSlot {
    num_fragments: u8,
    // Bit i is set once fragment (i+1) has been stored.
    // u16 covers bits 0..8 for up to 9 fragments.
    received_mask: u16,
    // fill_bits belongs to the highest-numbered fragment and may arrive out of
    // order, so it can't be sourced from the completing fragment at decode time.
    fill_bits: u8,
    // PERF: Fixed 9×82 = 738 bytes of armored storage per slot, 10 slots = 7,380
    // bytes always live in Fragments. A heap-allocated slot (Box or Vec per slot)
    // would reduce the baseline to zero for idle slots at the cost of one
    // allocation per active multi-sentence message.
    fragment_data: [[u8; MAX_ARMORED_LEN]; MAX_FRAGMENTS],
    fragment_lens: [u8; MAX_FRAGMENTS],
}

/// Returned by [`Fragments::insert`] when all fragments of a message have arrived.
pub(crate) struct Reassembled {
    pub num_fragments: u8,
    pub fill_bits: u8,
    // PERF: Armored bytes are copied into a contiguous buffer before being
    // passed to unarmor(). An offset-write API on Unarmored could let each
    // fragment unarmor directly into position, eliminating this intermediate copy.
    pub data: [u8; MAX_ASSEMBLED_LEN],
    pub len: usize,
}

#[derive(Debug)]
pub(crate) struct Fragments {
    // PERF: [Option<FragmentSlot>; 10] occupies ~7.6 KiB at all times.
    // Most slots will be None during typical operation.
    slots: [Option<FragmentSlot>; NUM_MESSAGE_SLOTS],
}

impl Default for Fragments {
    fn default() -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
        }
    }
}

impl Fragments {
    /// Store one fragment and return `Ok(Some(Reassembled))` when the message
    /// is complete, or `Ok(None)` while still waiting for more fragments.
    pub(crate) fn insert(
        &mut self,
        message_id: u8,
        fragment_num: u8,
        num_fragments: u8,
        armored: &[u8],
        fill_bits: u8,
    ) -> Result<Option<Reassembled>, ParseError> {
        let slot_idx = message_id as usize;
        if slot_idx >= NUM_MESSAGE_SLOTS {
            return Err(ParseError::Malformed(MalformedReason::MessageIdOutOfRange));
        }

        let fragment_idx = fragment_num.wrapping_sub(1) as usize;
        if fragment_idx >= MAX_FRAGMENTS {
            return Err(ParseError::Malformed(
                MalformedReason::FragmentNumOutOfRange,
            ));
        }

        let armored_len = armored.len();
        if armored_len > MAX_ARMORED_LEN {
            return Err(ParseError::Malformed(MalformedReason::ArmordPayloadTooLong));
        }

        let slot = &mut self.slots[slot_idx];

        let needs_reset = slot.as_ref().is_some_and(|s| {
            s.num_fragments != num_fragments || (s.received_mask & (1 << fragment_idx)) != 0
        });
        if needs_reset {
            *slot = None;
        }

        let s = slot.get_or_insert_with(|| FragmentSlot {
            num_fragments,
            received_mask: 0,
            fill_bits: 0,
            fragment_data: [[0u8; MAX_ARMORED_LEN]; MAX_FRAGMENTS],
            fragment_lens: [0u8; MAX_FRAGMENTS],
        });

        s.fragment_data[fragment_idx][..armored_len].copy_from_slice(armored);
        s.fragment_lens[fragment_idx] = armored_len as u8;
        s.received_mask |= 1 << fragment_idx;

        if fragment_num == s.num_fragments {
            s.fill_bits = fill_bits;
        }

        if s.received_mask != (1u16 << s.num_fragments) - 1 {
            return Ok(None);
        }

        let slot = self.slots[slot_idx].take().unwrap();

        let mut data = [0u8; MAX_ASSEMBLED_LEN];
        let mut len = 0usize;
        for i in 0..slot.num_fragments as usize {
            let frag_len = slot.fragment_lens[i] as usize;
            data[len..len + frag_len].copy_from_slice(&slot.fragment_data[i][..frag_len]);
            len += frag_len;
        }

        Ok(Some(Reassembled {
            num_fragments: slot.num_fragments,
            fill_bits: slot.fill_bits,
            data,
            len,
        }))
    }
}
