use instrument::Key;
use pcm::{PCMParameters, PCM};
use rand::Rng;
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
    fn gen(&mut self, sample_rate: u32, frequency: Frequency, duration: Duration) -> Key;
}

/// Example implementation of a Key Generator that creates Square Wave Signals
#[derive(Clone, Copy)]
pub struct SquareWaveGenerator {}

impl KeyGenerator for SquareWaveGenerator {
    fn gen(&mut self, sample_rate: u32, frequency: Frequency, duration: Duration) -> Key {
        let mut samples = Vec::new();
        let sample_rate_float = f64::from(sample_rate);
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
                    sample_rate,
                    nb_channels: 1,
                },
                loop_info: Vec::new(),
                samples,
            },
            frequency,
        }
    }
}

/// Example implementation of a Key Generator that creates Triangle Wave Signals
#[derive(Clone, Copy)]
pub struct TriangleWaveGenerator {}

impl KeyGenerator for TriangleWaveGenerator {
    fn gen(&mut self, sample_rate: u32, frequency: Frequency, duration: Duration) -> Key {
        let mut samples = Vec::new();
        let sample_rate_float = f64::from(sample_rate);
        let sample_period = sample_rate_float.recip();
        let nb_samples = (duration.get() * sample_rate_float) as u64; // Lossy
        let note_period = frequency.get().recip();
        let quarter_note_period = note_period / 4f64;
        let mut pos_seconds = 0f64;
        for _ in 0..nb_samples {
            let period_pos = pos_seconds % note_period;
            if period_pos < quarter_note_period {
                samples.push(period_pos * (4f64 / note_period));
            } else if period_pos < (3f64 * quarter_note_period) {
                samples.push(((period_pos - quarter_note_period) * -(4f64 / note_period)) + 1f64);
            } else {
                samples.push(
                    ((period_pos - (3f64 * quarter_note_period)) * (4f64 / note_period)) - 1f64,
                );
            }
            pos_seconds += sample_period;
        }
        Key {
            audio: PCM {
                parameters: PCMParameters {
                    sample_rate,
                    nb_channels: 1,
                },
                loop_info: Vec::new(),
                samples,
            },
            frequency,
        }
    }
}

/// Example implementation of a Key Generator that creates Sawtooth Wave Signals
#[derive(Clone, Copy)]
pub struct SawtoothWaveGenerator {}

impl KeyGenerator for SawtoothWaveGenerator {
    fn gen(&mut self, sample_rate: u32, frequency: Frequency, duration: Duration) -> Key {
        let mut samples = Vec::new();
        let sample_rate_float = f64::from(sample_rate);
        let sample_period = sample_rate_float.recip();
        let nb_samples = (duration.get() * sample_rate_float) as u64; // Lossy
        let note_period = frequency.get().recip();
        let mut pos_seconds = 0f64;
        for _ in 0..nb_samples {
            samples.push(1f64 + ((pos_seconds % note_period) * (-2f64 / note_period)));
            pos_seconds += sample_period;
        }
        Key {
            audio: PCM {
                parameters: PCMParameters {
                    sample_rate,
                    nb_channels: 1,
                },
                loop_info: Vec::new(),
                samples,
            },
            frequency,
        }
    }
}

/// A KeyGenerator that generates just plain white noise
#[derive(Clone, Copy)]
pub struct NoiseGenerator {}

impl KeyGenerator for NoiseGenerator {
    fn gen(&mut self, sample_rate: u32, frequency: Frequency, duration: Duration) -> Key {
        let nb_samples = (duration.get() * f64::from(sample_rate)) as usize; // Lossy
        let mut samples = Vec::with_capacity(nb_samples);
        let mut rng = rand::thread_rng();
        for _ in 0..nb_samples {
            samples.push(rng.gen_range(-1f64, 1f64));
        }
        Key {
            audio: PCM {
                parameters: PCMParameters {
                    sample_rate,
                    nb_channels: 1,
                },
                loop_info: Vec::new(),
                samples,
            },
            frequency,
        }
    }
}
