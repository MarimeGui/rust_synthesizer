use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum SynthesizerError {
    TimeInvalid(TimeInvalidError),
    NoFrequencyForID(NoFrequencyForIDError),
}

impl Error for SynthesizerError {
    fn description(&self) -> &str {
        match *self {
            SynthesizerError::TimeInvalid(ref e) => e.description(),
            SynthesizerError::NoFrequencyForID(ref e) => e.description(),
        }
    }
}

impl Display for SynthesizerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SynthesizerError::TimeInvalid(ref e) => e.fmt(f),
            SynthesizerError::NoFrequencyForID(ref e) => e.fmt(f),
        }
    }
}

impl From<TimeInvalidError> for SynthesizerError {
    fn from(e: TimeInvalidError) -> SynthesizerError {
        SynthesizerError::TimeInvalid(e)
    }
}

impl From<NoFrequencyForIDError> for SynthesizerError {
    fn from(e: NoFrequencyForIDError) -> SynthesizerError {
        SynthesizerError::NoFrequencyForID(e)
    }
}

#[derive(Debug)]
pub struct TimeInvalidError {
    pub value: f64,
}

impl Error for TimeInvalidError {
    fn description(&self) -> &str {
        "A provided value cannot be interpreted as a valid Time, Frequency or Duration"
    }
}

impl Display for TimeInvalidError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Wrong value: {}",
            self.value
        )
    }
}

#[derive(Debug)]
pub struct NoFrequencyForIDError {
    pub id: usize,
}

impl Error for NoFrequencyForIDError {
    fn description(&self) -> &str {
        "The Frequency Lookup failed, the provided IS was invalid"
    }
}

impl Display for NoFrequencyForIDError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Wrong ID: {}",
            self.id
        )
    }
}