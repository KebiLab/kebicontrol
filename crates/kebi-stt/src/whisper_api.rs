//! Whisper-compatible HTTP STT. Made by KebiLab

use super::SttEngine;
use anyhow::{Context, Result};
use async_trait::async_trait;
use hound::{SampleFormat, WavSpec, WavWriter};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WhisperResp { text: String }

/// Streams i16 PCM chunks to a Whisper-compatible endpoint by concatenating
/// into WAV, then on `finalize` POSTs the buffer.
pub struct WhisperApi {
    endpoint: String,
    api_key: String,
    model: String,
    language: String,
    buffer: Vec<i16>,
}

impl WhisperApi {
    pub fn new(endpoint: impl Into<String>, api_key: impl Into<String>, model: impl Into<String>, language: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            api_key: api_key.into(),
            model: model.into(),
            language: language.into(),
            buffer: Vec::new(),
        }
    }
}

#[async_trait]
impl SttEngine for WhisperApi {
    async fn feed(&mut self, samples: &[i16]) -> Result<()> {
        self.buffer.extend_from_slice(samples);
        Ok(())
    }

    async fn finalize(&mut self) -> Result<String> {
        if self.buffer.is_empty() { return Ok(String::new()); }
        let spec = WavSpec { channels: 1, sample_rate: 16000, bits_per_sample: 16, sample_format: SampleFormat::Int };
        let mut cursor = std::io::Cursor::new(Vec::<u8>::new());
        {
            let mut w = WavWriter::new(&mut cursor, spec).context("wav")?;
            for s in &self.buffer {
                w.write_sample(*s).context("wav sample")?;
            }
            w.finalize().context("wav finalize")?;
        }
        let bytes = cursor.into_inner();
        let len = bytes.len();
        self.buffer.clear();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;
        let part = reqwest::multipart::Part::bytes(bytes)
            .file_name("audio.wav")
            .mime_str("audio/wav")?;
        let mut form = reqwest::multipart::Form::new()
            .text("model", self.model.clone())
            .text("response_format", "json")
            .text("language", self.language.clone())
            .part("file", part);
        if self.api_key.is_empty() {
            anyhow::bail!("Whisper: API key is empty");
        }
        let resp = client.post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await
            .context("whisper http")?;
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            anyhow::bail!("whisper status {status} ({} bytes): {}", len, body);
        }
        let v: WhisperResp = serde_json::from_str(&body).context("whisper json")?;
        Ok(v.text.trim().to_string())
    }

    async fn reset(&mut self) -> Result<()> {
        self.buffer.clear();
        Ok(())
    }
}
