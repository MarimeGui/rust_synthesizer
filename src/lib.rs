//! This crate is all about using virtual Instruments and a music Sequence to produce usable music, playable by anything that can play Wave files.
//! You will need to create a Synthesizer first, providing a Sequence and Instruments.
//! You can use the SequenceHelper in the Helper mod to convert any Sequence format (MIDI for example) into this crate's format.
//! You need to manually instantiate Instrument structs and push them into the Vec of the Synthesizer.

extern crate ez_io;
extern crate rand;

/// Contains the errors in this library
pub mod error;
/// Allows to go from a Frequency ID to a Frequency Value
pub mod frequency_lookup;
/// Code for help on importing a sequence into something usable here
pub mod helper;
/// Instrument related data
pub mod instrument;
/// Generator for keys in instruments, also contains pre-made tone generators for use as instruments
pub mod key_generator;
/// Types for PCM Audio
pub mod pcm;
/// Sequence related data
pub mod sequence;
/// Useful things to make my life easier
pub mod util;
/// Handles writing and reading Wave files
pub mod wave;

/// The Result type used everywhere
type Result<T> = std::result::Result<T, error::SynthesizerError>;

use error::NoInstrumentError;
use frequency_lookup::FrequencyLookup;
use instrument::Instrument;
use pcm::{PCMParameters, PCM};
use sequence::Sequence;
use std::collections::HashMap;

/// The main Synthesizer. It holds the instruments, the sequence, and produces an hearable result
pub struct Synthesizer {
    /// The Sequence to play
    pub seq: Sequence,
    /// The Instruments used to play music
    pub inst: HashMap<usize, Instrument>,
    /// The Frequency Lookup used throughout the sequence and the instruments that provides all frequency values in an absolute way
    pub f_lu: Box<FrequencyLookup>,
    /// The parameters for the final output
    pub params: PCMParameters,
}

impl Synthesizer {
    /// Runs the Synthesizer and generates music
    pub fn run(&mut self) -> Result<PCM> {
        self.seq.sort_by_time();
        self.gen_inst_keys()?;
        let sample_rate_float = f64::from(self.params.sample_rate);
        let nb_channels_float = f64::from(self.params.nb_channels);
        let nb_samples =
            (self.seq.calc_music_duration()?.get() * sample_rate_float * nb_channels_float).round()
                as usize; // Lossy
        let mut out_pcm_data = vec![0f64; nb_samples];
        for note in &self.seq.notes {
            let mut to_add = self
                .inst
                .get(&note.i_id)
                .ok_or(NoInstrumentError { i_id: note.i_id })?
                .gen_sound(note.f_id, note.t_span.duration())?;
            let nb_samples = to_add.samples.len();
            // Forced falloff to prevent popping sounds
            let falloff = 100;
            if nb_samples > falloff {
                for i in 0..falloff {
                    to_add.samples[nb_samples - (falloff - i)] *=
                        (i as f64 * (-(1f64 / (falloff - 1) as f64))) + 1f64;
                }
            }
            let volumes = note.get_volume(self.params.nb_channels as usize); // Lossy
            let out_start_sample =
                (note.t_span.start_at().get() * sample_rate_float * nb_channels_float).round()
                    as usize; // Lossy
            for (sample_nb, sample) in to_add.samples.iter().enumerate() {
                for (channel, volume) in volumes.iter().enumerate() {
                    out_pcm_data[out_start_sample
                        + (sample_nb * self.params.nb_channels as usize)
                        + channel as usize] += sample * volume; // Lossy
                }
            }
        }
        Ok(PCM {
            parameters: self.params,
            loop_info: Vec::new(), // Needs to change
            samples: out_pcm_data,
        })
    }
    /// Generates all keys necessary for all Instruments
    pub fn gen_inst_keys(&mut self) -> Result<()> {
        for (i_id, f_id_duration) in &self.seq.list_freq_by_inst() {
            let inst = self
                .inst
                .get_mut(i_id)
                .ok_or(NoInstrumentError { i_id: *i_id })?;
            inst.gen_keys(self.params.sample_rate, f_id_duration, &self.f_lu)?;
        }
        Ok(())
    }
}
