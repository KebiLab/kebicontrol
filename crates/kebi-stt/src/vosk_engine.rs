//! Vosk offline STT. Made by KebiLab

use super::SttEngine;
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use vosk::{Model, Recognizer};

/// Vosk-based recognizer. We only enable this on platforms where the FFI lib
/// is bundled. For dev builds without the model we report a clear error.
pub struct VoskEngine {
    model_dir: PathBuf,
    recognizer: Option<Arc<parking_lot::Mutex<Recognizer>>>,
}

impl VoskEngine {
    pub fn new(model_dir: impl AsRef<Path>) -> Self {
        Self { model_dir: model_dir.as_ref().to_path_buf(), recognizer: None }
    }

    pub fn is_ready(&self) -> bool { self.recognizer.is_some() }

    pub fn try_init(&mut self) -> Result<()> {
        let model = Model::new(self.model_dir.to_str().context("model path")?)
            .context("failed to load vosk model — download it first")?;
        let rec = Recognizer::new(&model, 16000.0)
            .context("recognizer")?;
        self.recognizer = Some(Arc::new(parking_lot::Mutex::new(rec)));
        Ok(())
    }
}

#[async_trait]
impl SttEngine for VoskEngine {
    async fn feed(&mut self, samples: &[i16]) -> Result<()> {
        let Some(rec) = self.recognizer.clone() else {
            return Ok(()); // not initialized: silently drop
        };
        let mut r = rec.lock();
        r.accept_waveform(samples);
        Ok(())
    }

    async fn finalize(&mut self) -> Result<String> {
        let Some(rec) = self.recognizer.clone() else {
            return Ok(String::new());
        };
        let mut r = rec.lock();
        let s = r.final_result().single().unwrap_or_default();
        Ok(s.text.to_string())
    }

    async fn reset(&mut self) -> Result<()> {
        if let Some(rec) = &self.recognizer {
            rec.lock().reset();
        }
        Ok(())
    }
}
