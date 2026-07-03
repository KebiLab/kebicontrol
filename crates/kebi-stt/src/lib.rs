//! Speech-to-text. Made by KebiLab

pub mod vosk_engine;
pub mod whisper_api;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SttEngine: Send + Sync {
    /// Feed a chunk of i16 PCM samples at 16 kHz mono.
    async fn feed(&mut self, samples: &[i16]) -> Result<()>;
    /// Finalize and get the recognized text (partial OK).
    async fn finalize(&mut self) -> Result<String>;
    /// Reset state.
    async fn reset(&mut self) -> Result<()>;
}

pub use vosk_engine::VoskEngine;
pub use whisper_api::WhisperApi;
