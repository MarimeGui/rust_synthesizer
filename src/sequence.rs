use util::TimeSpan;

/// Represents a Sequence of notes forming music. Think of it as a music sheet
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
    /// ID for the FLUT to get the frequency of this note
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
