use sequence::Sequence;
use util::Frequency;
use std::collections::HashMap;
use std::f64::EPSILON;

/// Holds information about a currently playing note
#[derive(Copy, Clone)]
pub struct PartialNote {
    pub start_at: f64,
    pub on_velocity: f64,
}

/// Helps the user to build a Sequence usable by the Synthesizer
#[derive(Default)]
pub struct SequenceHelper {
    // InstrumentID -> FrequencyID -> PartialNote
    pub current_notes: HashMap<usize, HashMap<usize, PartialNote>>,
    pub sequence: Sequence,
    pub at_time: f64,
}

/// Builds a Frequency Lookup Table if nothing else can be used.
pub struct FrequencyLookupTableBuilder {
    /// The internal vector that stores the frequencies. You can re-use this vector as the FrequencyLookup in the synthesizer.
    pub builder: Vec<Frequency>,
}

impl SequenceHelper {
    /// Creates an empty new SequenceHelper.
    pub fn new() -> SequenceHelper {
        SequenceHelper {
            current_notes: HashMap::new(),
            sequence: Sequence::new(),
            at_time: 0f64,
        }
    }
}

impl FrequencyLookupTableBuilder {
    /// Returns an ID for the specified frequency. If it already exists, it returns the already existing ID, but if it does not, it creates it.
    pub fn get_id(&mut self, frequency: Frequency) -> usize {
        match self.builder.iter().position(|&x| (x.get() - frequency.get()).abs() < EPSILON) {
            Some(i) => i,
            None => {
                self.builder.push(frequency);
                self.builder.len() - 1
            }
        }
    }
}