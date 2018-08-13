use key_generator::KeyGenerator;
use pcm::PCM;
use std::collections::HashMap;

/// Defines an instrument capable of playing notes
pub struct Instrument {
    /// Keys of the instrument. Index is frequency ID defined in the FLUT.
    pub keys: HashMap<usize, Key>,
    /// The key generators that creates new keys when needed
    pub key_gen: Box<KeyGenerator>,
    /// When we reach the end of the sound sample when playing this instrument, should we loop back to the beginning or just stop here ?
    pub loopable: bool,
}

pub struct Key {
    /// Audio of this key
    pub audio: PCM,
    /// The frequency ID of the audio sound sample, should be the same as the index of the "keys" HashMap if the Key came from here
    pub f_id: usize,
}