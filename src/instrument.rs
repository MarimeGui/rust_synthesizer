use error::NoKeyInInstrumentError;
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
#[derive(Clone)]
pub struct Key {
    /// Audio of this key
    pub audio: PCM,
    /// The frequency ID of the audio sound sample, should be the same as the index of the "keys" HashMap if the Key came from here
    pub frequency: Frequency,
}

impl Instrument {
    /// Generates keys provided as arguments
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
    /// "Plays" the instrument and returns a sound with the provided parameters
    pub fn gen_sound(&self, f_id: usize, duration: Duration) -> Result<PCM> {
        let key = self.keys.get(&f_id).ok_or(NoKeyInInstrumentError { f_id })?;
        let nb_samples = (duration.get() * f64::from(key.audio.parameters.sample_rate)) as usize;
        let mut pcm_out = Vec::with_capacity(nb_samples);
        let last_key_sample = key.audio.samples.len() - 1;
        for current_sample in 0..nb_samples {
            pcm_out.push(key.audio.samples[current_sample % last_key_sample]);
        }
        Ok(PCM {
            parameters: key.audio.parameters,
            loop_info: Vec::new(),
            samples: pcm_out,
        })
    }
}
