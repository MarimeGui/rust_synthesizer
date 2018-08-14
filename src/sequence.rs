use error::EmptySequenceError;
use std::result::Result;
use util::{Duration, Force, Order, TimeSpan};

/// Represents a Sequence of notes forming music. Think of it as a music sheet
#[derive(Default)]
pub struct Sequence {
    /// Vector of notes composing the sequence
    pub notes: Vec<Note>,
    /// The optional Looping information about the music
    pub loop_info: Vec<TimeSpan>,
}

/// A single note played by a single instrument a a certain point in time. It is part of a Sequence
pub struct Note {
    /// Defines exactly when this note is played in the sequence
    pub t_span: TimeSpan,
    /// Velocities of the note
    pub vel: Velocity,
    /// ID defined by the Frequency Lookup
    pub f_id: usize,
    /// Specifies which instrument to use when playing this note
    pub i_id: usize,
}

/// Holds velocities for a Note
pub struct Velocity {
    /// Velocity when pressed
    pub on: Option<Force>,
    /// Velocity when released
    pub off: Option<Force>,
}

impl Sequence {
    /// Creates a new empty sequence
    pub fn new() -> Sequence {
        Sequence {
            notes: Vec::new(),
            loop_info: Vec::new(),
        }
    }
    /// Adds a Note to the Sequence
    pub fn add_note(&mut self, other: Note) {
        self.notes.push(other);
    }
    /// Sorts all Notes in the sequence by time
    pub fn sort_by_time(&mut self) {
        self.notes
            .sort_by(|a, b| a.t_span.start_at().compare(b.t_span.start_at()).into());
    }
    /// Find out how long the music is
    pub fn calc_music_duration(&self) -> Result<Duration, EmptySequenceError> {
        let mut last_note_end_at = None;
        for note in &self.notes {
            match last_note_end_at {
                None => last_note_end_at = Some(note.t_span.end_at()),
                Some(lst) => {
                    if let Order::After = note.t_span.end_at().compare(lst) {
                        last_note_end_at = Some(note.t_span.end_at())
                    }
                }
            }
        }
        match last_note_end_at {
            Some(d) => Ok(d),
            None => Err(EmptySequenceError {}),
        }
    }
}
