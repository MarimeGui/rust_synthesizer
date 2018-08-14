//! This crate is all about using virtual Instruments and a music Sequence to produce usable music, playable by anything that can play Wave files.
//! You will need to create a Synthesizer first, providing a Sequence and Instruments.
//! You can use the SequenceHelper in the Helper mod to convert any Sequence format (MIDI for example) into this crate's format.
//! You need to manually instantiate Instrument structs and push them into the Vec of the Synthesizer.

/// Contains the errors in this library
pub mod error;
/// Allows to go from a Frequency ID to a Frequency Value
pub mod flut;
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

use flut::FrequencyLookup;
use instrument::Instrument;
use sequence::Sequence;
use std::collections::HashMap;

/// The main Synthesizer. It holds the instruments, the sequence, and produces an hearable result
pub struct Synthesizer {
    /// The Sequence to play
    pub seq: Sequence,
    /// The Instruments used to play music
    pub inst: HashMap<usize, Instrument>,
    /// The Frequency Lookup used throughout the sequence and the instruments that provides all frequency values in an absolute way
    pub f_lut: Box<FrequencyLookup>,
}
