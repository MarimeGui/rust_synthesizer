use frequency_lookup::FrequencyLookup;
use key_generator::KeyGenerator;
use pcm::PCM;
use std::collections::HashMap;
use util::{Duration, Frequency};
use Result;

/// Defines an instrument capable of playing notes
pub struct Instrument {
    /// Keys of the instrument. Index is the frequency ID defined by the Frequency Lookup.
    pub keys: HashMap<usize, Key>,
    /// The key generators that creates new keys when needed
    pub key_gen: Box<KeyGenerator>,
    /// When we reach the end of the sound sample when playing this instrument, should we loop back to the beginning or just stop here ?
    pub loopable: bool,
}

/// Key of an Instrument. Think of it as an Instrument having multiple physical keys to press, and everyone of them produces a different sound from each other.
pub struct Key {
    /// Audio of this key
    pub audio: PCM,
    /// The frequency ID of the audio sound sample, should be the same as the index of the "keys" HashMap if the Key came from here
    pub frequency: Frequency,
}

impl Instrument {
    #[allow(borrowed_box)] // I do not know how to fix the Clippy warning
    pub fn gen_keys(
        &mut self,
        sample_rate: u32,
        f_id_duration: &[(usize, Duration)],
        f_lu: &Box<FrequencyLookup>,
    ) -> Result<()> {
        for (f_id, duration) in f_id_duration {
            let freq = f_lu.get_freq(f_id)?;
            self.keys
                .insert(*f_id, self.key_gen.gen(&sample_rate, &freq, duration));
        }
        Ok(())
    }
}
