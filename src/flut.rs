use error::NoFrequencyForIDError;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::result::Result;
use util::Frequency;

/// Provides a Frequency value from a Frequency ID
pub trait FrequencyLookup {
    /// The function that does the link between the ID and the Frequency
    fn get_freq(&mut self, id: &usize) -> Result<Frequency, NoFrequencyForIDError>;
}

impl<S: BuildHasher> FrequencyLookup for HashMap<usize, Frequency, S> {
    fn get_freq(&mut self, id: &usize) -> Result<Frequency, NoFrequencyForIDError> {
        match self.get(id) {
            Some(f) => Ok(*f),
            None => Err(NoFrequencyForIDError { id: *id }),
        }
    }
}
