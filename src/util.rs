use error::{ForceInvalidError, TimeInvalidError};
use std::cmp::Ordering;
use std::result::Result;

pub type Frequency = Time;
pub type Duration = Time;

/// This type ensures that the contained f64 can be interpreted as a valid point in time in seconds
#[derive(Clone, Copy)]
pub struct Time {
    /// The value contained. It is in seconds
    value: f64,
}

impl Time {
    /// Creates a new Time by providing the time in seconds
    pub fn new(value: f64) -> Result<Time, TimeInvalidError> {
        if (value.is_finite()) & (value >= 0f64) {
            Ok(Time { value })
        } else {
            Err(TimeInvalidError { value })
        }
    }
    /// Returns the value
    pub fn get(self) -> f64 {
        self.value
    }
    /// Compares to another time
    pub fn compare(self, other: Time) -> Ordering {
        if self.value < other.value {
            Ordering::Less
        } else if self.value > other.value {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

/// Defines a time span, a duration of en event that starts and end at some point
#[derive(Clone, Copy)]
pub struct TimeSpan {
    /// When the span starts
    start_at: Time,
    /// When the span stops
    end_at: Time,
    /// How long is the span
    duration: Duration,
}

impl TimeSpan {
    /// Creates a new TimeSpan from a beginning and end date
    pub fn new(start_at: Time, end_at: Time) -> Result<TimeSpan, TimeInvalidError> {
        let duration = end_at.value - start_at.value;
        if (duration.is_finite()) & (duration > 0f64) {
            Ok(TimeSpan {
                start_at,
                end_at,
                duration: Duration { value: duration },
            })
        } else {
            Err(TimeInvalidError { value: duration })
        }
    }
    /// Creates a new TimeSpan from a Start time and a Duration
    pub fn new_rel(start_at: Time, duration: Duration) -> Result<TimeSpan, TimeInvalidError> {
        let end_at = start_at.value + duration.value;
        if (end_at.is_finite()) & (end_at > 0f64) {
            Ok(TimeSpan {
                start_at,
                end_at: Time { value: end_at },
                duration,
            })
        } else {
            Err(TimeInvalidError { value: end_at })
        }
    }
    /// Returns in seconds when the span starts
    pub fn start_at(&self) -> Time {
        self.start_at
    }
    /// Returns in seconds when the span ends
    pub fn end_at(&self) -> Time {
        self.end_at
    }
    /// Returns in seconds how long the span is
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

/// Describes the loudness of a Note. It forces the value to be in [0; 1].
#[derive(Copy, Clone)]
pub struct Volume {
    value: f64,
}

impl Volume {
    /// Creates a new Volume from an f64 value
    pub fn new(value: f64) -> Result<Volume, ForceInvalidError> {
        if value.is_finite() && value >= 0f64 && value <= 1f64 {
            return Ok(Volume { value });
        }
        Err(ForceInvalidError { value })
    }
    /// Returns the value
    pub fn get(self) -> f64 {
        self.value
    }
}
