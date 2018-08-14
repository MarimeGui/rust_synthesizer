use util::TimeSpan;

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
    pub vel: Option<Velocity>,
    /// ID defined by the Frequency Lookup
    pub f_id: usize,
    /// Specifies which instrument to use when playing this note
    pub i_id: usize,
}

/// Holds velocities for a Note
pub struct Velocity {
    /// Velocity when pressed
    pub pressed: f64,
    /// Velocity when released
    pub released: f64,
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
}
