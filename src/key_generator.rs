use instrument::Key;
use pcm::{PCMParameters, PCM};
use util::{Duration, Frequency};

/// Generates new keys to add to an Instrument
pub trait KeyGenerator {
    /// Generates a new key for an instrument.
    /// Note that the generated PCM should always be in Mono.
    /// # Arguments
    /// * sample_rate - The number of samples per second that should be produced.
    /// * frequency - The frequency that this key should produce.
    /// * duration - The longest time this key will be held for.
    /// This is useful if the generator creates non-loopable sounds.
    /// Can be ignored if the source is for example a pre-recorded sound sample that then gets pitch-shifted.
    fn gen(&mut self, sample_rate: &u32, frequency: &Frequency, duration: &Duration) -> Key;
}

/// Example implementation of a Key Generator that creates Square Wave Signals
#[derive(Clone, Copy)]
pub struct SquareWaveGenerator {}

impl KeyGenerator for SquareWaveGenerator {
    fn gen(&mut self, sample_rate: &u32, frequency: &Frequency, duration: &Duration) -> Key {
        let mut samples = Vec::new();
        let sample_rate_float = f64::from(*sample_rate);
        let sample_period = sample_rate_float.recip();
        let nb_samples = (duration.get() * sample_rate_float) as u64; // Lossy
        let note_period = frequency.get().recip();
        let half_note_period = note_period / 2f64;
        let mut pos_seconds = 0f64;
        for _ in 0..nb_samples {
            if (pos_seconds % note_period) < half_note_period {
                samples.push(1f64);
            } else {
                samples.push(-1f64);
            }
            pos_seconds += sample_period;
        }
        Key {
            audio: PCM {
                parameters: PCMParameters {
                    sample_rate: *sample_rate,
                    nb_channels: 1,
                },
                loop_info: Vec::new(),
                samples,
            },
            frequency: *frequency,
        }
    }
}
