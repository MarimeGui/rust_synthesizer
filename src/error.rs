use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// The main error type, contains all errors that could be thrown when running the Synthesizer
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

/// Raised when some f64 value cannot be used as a valid Time (negative, not finite, not a number)
#[derive(Debug)]
pub struct TimeInvalidError {
    /// The incorrect value
    pub value: f64,
}

impl Error for TimeInvalidError {
    fn description(&self) -> &str {
        "A provided value cannot be interpreted as a valid Time, Frequency or Duration"
    }
}

impl Display for TimeInvalidError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Wrong value: {}", self.value)
    }
}

/// Raised when a the Frequency Lookup failed (an ID could not get interpreted as a value)
#[derive(Debug)]
pub struct NoFrequencyForIDError {
    /// The failed ID
    pub id: usize,
}

impl Error for NoFrequencyForIDError {
    fn description(&self) -> &str {
        "The Frequency Lookup failed, the provided IS was invalid"
    }
}

impl Display for NoFrequencyForIDError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Wrong ID: {}", self.id)
    }
}
