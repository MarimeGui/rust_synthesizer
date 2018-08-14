use error::{SynthesizerError, NoFrequencyForIDError};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::result::Result;
use util::Frequency;

/// Provides a Frequency value from a Frequency ID
pub trait FrequencyLookup {
    /// The function that does the link between the ID and the Frequency
    fn get_freq(&mut self, id: &usize) -> Result<Frequency, SynthesizerError>;
}

impl<S: BuildHasher> FrequencyLookup for HashMap<usize, Frequency, S> {
    fn get_freq(&mut self, id: &usize) -> Result<Frequency, SynthesizerError> {
        match self.get(id) {
            Some(f) => Ok(*f),
            None => Err(SynthesizerError::NoFrequencyForID(NoFrequencyForIDError{id: *id,})),
        }
    }
}

/// Example implementation of FrequencyLookup for MIDI
pub struct MIDIFrequencyLookup {}

impl FrequencyLookup for MIDIFrequencyLookup {
    fn get_freq(&mut self, id: &usize) -> Result<Frequency, SynthesizerError> {
        Ok(Frequency::new(2f64.powf((id - 69) as f64 / 12f64) * 440f64)?)
    }
}