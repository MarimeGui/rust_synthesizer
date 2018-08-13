use instrument::Key;
use util::{Frequency, Time};

/// Generates new keys to add to an Instrument
pub trait KeyGenerator {
    /// Generates a new key for an instrument
    /// # Arguments
    /// * frequency - The frequency that this key should produce
    /// * duration - The longest time this key will be held for.
    /// This is useful if the generator creates non-loopable sounds.
    /// Can be ignored if the source is for example a pre-recorded sound sample that then gets pitch-shifted.
    fn gen(&self, frequency: &Frequency, duration: &Time) -> Key;
}
