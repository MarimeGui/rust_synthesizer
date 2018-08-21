use error::WriteError;
use ez_io::WriteE;
use pcm::PCM;
use std::io::Write;
use std::result::Result;

/// Represents a Wave File
pub struct Wave {
    pub pcm: PCM,
    pub sample_type: SampleType,
}

/// The type of sample to use
pub enum SampleType {
    /// 8 Bits Unsigned Integer
    Unsigned8,
    /// 16 Bits Signed Integer
    Signed16,
    /// 32 bits Signed Integer
    Signed32,
}

impl Wave {
    /// Write to a Wave file
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), WriteError> {
        // Error for bigger than 32 bits streams
        let extreme = self.pcm.get_extreme()?;
        let max_value = self.sample_type.get_max_value();
        let data_chunk_interior_size =
            self.pcm.samples.len() as u32 * u32::from(self.sample_type.get_sample_size());
        writer.write_all(&[b'R', b'I', b'F', b'F'])?; // RIFF Chunk
        writer.write_le_to_u32(36 + data_chunk_interior_size)?;
        writer.write_all(&[b'W', b'A', b'V', b'E'])?; // WAVE Format
        writer.write_all(&[b'f', b'm', b't', b' '])?; // Format Chunk
        writer.write_le_to_u32(16)?;
        writer.write_le_to_u16(1)?;
        writer.write_le_to_u16(self.pcm.parameters.nb_channels)?;
        writer.write_le_to_u32(self.pcm.parameters.sample_rate)?;
        writer.write_le_to_u32(
            self.pcm.parameters.sample_rate
                * u32::from(self.pcm.parameters.nb_channels)
                * u32::from(self.sample_type.get_sample_size()),
        )?; // Byte Rate
        writer.write_le_to_u16(
            self.pcm.parameters.nb_channels
                * u16::from(self.sample_type.get_sample_size()),
        )?; // Block Align
        writer.write_le_to_u16(u16::from(self.sample_type.get_sample_size()) * 8)?;
        writer.write_all(&[b'd', b'a', b't', b'a'])?; // Sub-chunk 2 ID
        writer.write_le_to_u32(data_chunk_interior_size)?;
        for sample in &self.pcm.samples {
            match self.sample_type {
                SampleType::Unsigned8 => {
                    writer.write_to_u8(((((sample / extreme) + 1f64) / 2f64) * max_value).round() as u8)?;
                }
                SampleType::Signed16 => {
                    writer.write_le_to_i16(((sample / extreme) * max_value).round() as i16)?;
                }
                SampleType::Signed32 => {
                    writer.write_le_to_u32(((sample / extreme) * max_value).round() as u32)?;
                }
            }
        }
        Ok(())
    }
}

impl SampleType {
    /// Get the size of a single sample in bytes
    pub fn get_sample_size(&self) -> u8 {
        match *self {
            SampleType::Unsigned8 => 1,
            SampleType::Signed16 => 2,
            SampleType::Signed32 => 4,
        }
    }
    pub fn get_max_value(&self) -> f64 {
        match *self {
            SampleType::Unsigned8 => f64::from(<u8>::max_value()),
            SampleType::Signed16 => f64::from(<i16>::max_value()),
            SampleType::Signed32 => f64::from(<i32>::max_value()),
        }
    }
}
