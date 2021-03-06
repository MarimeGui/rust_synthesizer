use sequence::Note;
use sequence::Sequence;
use std::collections::HashMap;
use std::f64::EPSILON;
use util::{Duration, Frequency, Time, TimeSpan, Volume};
use Result;

/// Holds information about a currently playing note
#[derive(Clone)]
pub struct PartialNote {
    pub start_at: Time,
    pub vol: Vec<Volume>,
}

/// Helps the user to build a Sequence usable by the Synthesizer
#[derive(Clone, Default)]
pub struct SequenceHelper {
    /// Stores the currently playing notes. It follows the pattern InstrumentID -> FrequencyID -> PartialNote.
    pub current_notes: HashMap<usize, HashMap<usize, PartialNote>>,
    /// The Sequence being built. Once the helper is finished, grab this for the synthesizer
    pub sequence: Sequence,
    /// Where we are at in the sequence. Use functions to modify this and do not forget to reset it when changing tracks !
    pub at_time: f64,
}

/// Builds a Frequency Lookup Table if nothing else can be used.
#[derive(Clone, Default)]
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

    /// Moves time forward in the sequence
    pub fn time_forward(&mut self, time_passed: f64) {
        self.at_time += time_passed;
    }

    /// Resets the time to 0 and currently running notes. Useful when changing tracks.
    pub fn reset(&mut self) {
        self.at_time = 0f64;
        self.current_notes = HashMap::new();
    }

    /// Starts a note that will finish later.
    /// # Arguments
    /// * f_id - The Frequency ID
    /// * i_id - The Instrument ID
    /// * vol - The volumes of the note for each channel
    pub fn start_note(&mut self, f_id: usize, i_id: usize, vol: Vec<Volume>) -> Result<()> {
        let instrument_map = self.current_notes.entry(i_id).or_insert_with(HashMap::new);
        match instrument_map.get(&f_id) {
            None => {
                instrument_map.insert(
                    f_id,
                    PartialNote {
                        start_at: Time::new(self.at_time)?,
                        vol,
                    },
                );
            }
            Some(_) => {} // Ignore if there is already a Note running at the same frequency
        }
        Ok(())
    }

    /// Stops a note started earlier, finalizes it and adds it to the Sequence
    /// # Arguments
    /// * f_id - The Frequency ID
    /// * i_id - The Instrument ID
    pub fn stop_note(&mut self, f_id: usize, i_id: usize) -> Result<()> {
        let mut to_remove = true;
        if let Some(inst_map) = self.current_notes.get_mut(&i_id) {
            if let Some(partial_note) = inst_map.get(&f_id) {
                // Ignore events that have a duration of 0 seconds
                if let Ok(t) = TimeSpan::new(partial_note.start_at, Time::new(self.at_time)?) {
                    self.sequence.add_note(Note {
                        t_span: t,
                        vol: partial_note.vol.clone(),
                        f_id,
                        i_id,
                    });
                };
                to_remove = true;
            }
            if to_remove {
                inst_map.remove(&f_id);
            }
        }
        Ok(())
    }

    /// Directly adds a new note to the Sequence.
    /// # Arguments
    /// * f_id - The Frequency ID
    /// * i_id - The Instrument ID
    /// * duration - How long the Note plays for
    /// * vol - The (optional) velocity for the note
    pub fn new_note(
        &mut self,
        f_id: usize,
        i_id: usize,
        duration: Duration,
        vol: Vec<Volume>,
    ) -> Result<()> {
        self.sequence.add_note(Note {
            t_span: TimeSpan::new_rel(Time::new(self.at_time)?, duration)?,
            vol,
            f_id,
            i_id,
        });
        Ok(())
    }
}

impl FrequencyLookupTableBuilder {
    /// Creates a new builder
    pub fn new() -> FrequencyLookupTableBuilder {
        FrequencyLookupTableBuilder {
            builder: Vec::new(),
        }
    }
    /// Returns an ID for the specified frequency. If it already exists, it returns the already existing ID, but if it does not, it gets created.
    pub fn get_id(&mut self, frequency: Frequency) -> usize {
        match self
            .builder
            .iter()
            .position(|&x| (x.get() - frequency.get()).abs() < EPSILON)
        {
            Some(i) => i,
            None => {
                self.builder.push(frequency);
                self.builder.len() - 1
            }
        }
    }
}
