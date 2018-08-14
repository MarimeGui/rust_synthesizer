use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// The main error type, contains all errors that could be thrown when running the Synthesizer
#[derive(Debug)]
pub enum SynthesizerError {
    TimeInvalid(TimeInvalidError),
    NoFrequencyForID(NoFrequencyForIDError),
    ForceInvalid(ForceInvalidError),
    EmptySequence(EmptySequenceError),
}

impl Error for SynthesizerError {
    fn description(&self) -> &str {
        match *self {
            SynthesizerError::TimeInvalid(ref e) => e.description(),
            SynthesizerError::NoFrequencyForID(ref e) => e.description(),
            SynthesizerError::ForceInvalid(ref e) => e.description(),
            SynthesizerError::EmptySequence(ref e) => e.description(),
        }
    }
}

impl Display for SynthesizerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SynthesizerError::TimeInvalid(ref e) => e.fmt(f),
            SynthesizerError::NoFrequencyForID(ref e) => e.fmt(f),
            SynthesizerError::ForceInvalid(ref e) => e.fmt(f),
            SynthesizerError::EmptySequence(ref e) => e.fmt(f),
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

impl From<ForceInvalidError> for SynthesizerError {
    fn from(e: ForceInvalidError) -> SynthesizerError {
        SynthesizerError::ForceInvalid(e)
    }
}

impl From<EmptySequenceError> for SynthesizerError {
    fn from(e: EmptySequenceError) -> SynthesizerError {
        SynthesizerError::EmptySequence(e)
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

/// Raised when a value that had to be interpreted as a Force was not between [0; 1].
#[derive(Debug)]
pub struct ForceInvalidError {
    /// The invalid force
    pub value: f64,
}

impl Error for ForceInvalidError {
    fn description(&self) -> &str {
        "A value that was tried to be used a force was not valid for this purpose."
    }
}

impl Display for ForceInvalidError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Value: {}", self.value)
    }
}

/// Raised when some operation is tried on an empty Sequence.
#[derive(Debug)]
pub struct EmptySequenceError {}

impl Error for EmptySequenceError {
    fn description(&self) -> &str {
        "Tried to perform operations on an empty Sequence"
    }
}

impl Display for EmptySequenceError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Sequence is empty")
    }
}
