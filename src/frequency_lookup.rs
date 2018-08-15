use error::NoFrequencyForIDError;
use std::collections::HashMap;
use std::hash::BuildHasher;
use util::Frequency;
use Result;

/// Provides a Frequency value from a Frequency ID
pub trait FrequencyLookup {
    /// The function that does the link between the ID and the Frequency
    fn get_freq(&self, id: &usize) -> Result<Frequency>;
}

impl<S: BuildHasher> FrequencyLookup for HashMap<usize, Frequency, S> {
    fn get_freq(&self, id: &usize) -> Result<Frequency> {
        match self.get(id) {
            Some(f) => Ok(*f),
            None => Err(NoFrequencyForIDError { id: *id }.into()),
        }
    }
}

impl FrequencyLookup for Vec<Frequency> {
    fn get_freq(&self, id: &usize) -> Result<Frequency> {
        match self.get(*id) {
            Some(f) => Ok(*f),
            None => Err(NoFrequencyForIDError { id: *id }.into()),
        }
    }
}

/// Example implementation of FrequencyLookup for MIDI
pub struct MIDIFrequencyLookup {}

impl FrequencyLookup for MIDIFrequencyLookup {
    fn get_freq(&self, id: &usize) -> Result<Frequency> {
        Ok(Frequency::new(
            2f64.powf((id - 69) as f64 / 12f64) * 440f64,
        )?) // Lossy
    }
}
