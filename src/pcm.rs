use util::TimeSpan;

/// Holds PCM data as f64s.
/// When this is used in an instrument, the stream should be normalized as [-1; 1].
/// When this is used as the Synthesizer output, the stream should not be normalized, and further processing will allow to rewrite the stream as a more convenient format (u16 mostly).
#[derive(Clone)]
pub struct PCM {
    /// Information about the stream
    pub parameters: PCMParameters,
    /// Where does the music loop (if it does, leave empty otherwise)
    pub loop_info: Vec<TimeSpan>,
    /// The raw PCM data stored as f64s
    pub samples: Vec<f64>,
}

/// Parameters for a PCM stream
#[derive(Clone, Copy)]
pub struct PCMParameters {
    /// How many samples per second
    pub sample_rate: u32,
    /// How many channels
    pub nb_channels: u16,
}
