use std::error::Error;
use std::fmt;

use crate::wave_writer;

pub type PiperResult<T> = Result<T, PiperError>;
pub type PiperWaveResult = PiperResult<PiperWaveSamples>;

#[derive(Debug)]
pub enum PiperError {
    FailedToLoadResource(String),
    PhonemizationError(String),
    OperationError(String),
}

impl Error for PiperError {}

impl fmt::Display for PiperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_message = match self {
            PiperError::FailedToLoadResource(msg) => {
                format!("Failed to load resource from. Error `{}`", msg)
            }
            PiperError::PhonemizationError(msg) => msg.to_string(),
            PiperError::OperationError(msg) => msg.to_string(),
        };
        write!(f, "{}", err_message)
    }
}

impl From<wave_writer::WaveWriterError> for PiperError {
    fn from(error: wave_writer::WaveWriterError) -> Self {
        PiperError::OperationError(error.to_string())
    }
}

/// A wrapper type that holds sentence phonemes
pub struct Phonemes(pub Vec<String>);

impl Phonemes {
    pub fn sentences(&self) -> &Vec<String> {
        &self.0
    }

    pub fn to_vec(self) -> Vec<String> {
        self.0
    }

    pub fn num_sentences(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<String>> for Phonemes {
    fn from(other: Vec<String>) -> Self {
        Self(other)
    }
}

impl std::string::ToString for Phonemes {
    fn to_string(&self) -> String {
        self.0.join(" ")
    }
}

#[derive(Debug, Clone)]
pub struct PiperWaveInfo {
    pub sample_rate: usize,
    pub num_channels: usize,
    pub sample_width: usize,
}

#[derive(Debug, Clone)]
#[must_use]
pub struct PiperWaveSamples {
    pub samples: Vec<i16>,
    pub info: PiperWaveInfo,
    pub inference_ms: Option<f32>,
}

impl PiperWaveSamples {
    pub fn new(samples: Vec<i16>, sample_rate: usize, inference_ms: Option<f32>) -> Self {
        Self {
            samples,
            inference_ms,
            info: PiperWaveInfo {
                sample_rate,
                num_channels: 1,
                sample_width: 2,
            },
        }
    }

    pub fn to_vec(self) -> Vec<i16> {
        self.samples
    }

    pub fn as_wave_bytes(&self) -> Vec<u8> {
        self.samples.iter().flat_map(|i| i.to_le_bytes()).collect()
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn duration_ms(&self) -> f32 {
        (self.len() as f32 / self.info.sample_rate as f32) * 1000.0f32
    }

    pub fn inference_ms(&self) -> Option<f32> {
        self.inference_ms
    }

    pub fn real_time_factor(&self) -> Option<f32> {
        let Some(infer_ms) = self.inference_ms else {
            return None;
        };
        let audio_duration = self.duration_ms();
        if audio_duration == 0. {
            return Some(0.);
        }
        Some(infer_ms / audio_duration)
    }

    pub fn save_to_file(&self, filename: &str) -> PiperResult<()> {
        Ok(wave_writer::write_wave_samples_to_file(
            filename.into(),
            self.samples.iter(),
            self.info.sample_rate as u32,
            self.info.num_channels as u32,
            self.info.sample_width as u32,
        )?)
    }
}

impl IntoIterator for PiperWaveSamples {
    type Item = i16;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.samples.into_iter()
    }
}

pub trait PiperModel {
    fn phonemize_text(&self, text: &str) -> PiperResult<Phonemes>;
    fn speak_batch(&self, phoneme_batches: Vec<String>) -> PiperResult<Vec<PiperWaveSamples>>;
    fn speak_one_sentence(&self, phonemes: String) -> PiperWaveResult;
    fn wave_info(&self) -> PiperResult<PiperWaveInfo>;
}
