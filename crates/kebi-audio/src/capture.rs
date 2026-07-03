//! Microphone capture. Made by KebiLab

use anyhow::{anyhow, Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, Stream};
use parking_lot::Mutex;
use std::sync::Arc;

/// Captures 16 kHz mono PCM frames from the default input device.
pub struct AudioCapture {
    stream: Option<Stream>,
    buffer: Arc<Mutex<Vec<i16>>>,
    sample_rate: u32,
}

impl AudioCapture {
    pub fn new(sample_rate: u32) -> Self {
        Self { stream: None, buffer: Arc::new(Mutex::new(Vec::new())), sample_rate }
    }

    pub fn buffer(&self) -> Arc<Mutex<Vec<i16>>> {
        self.buffer.clone()
    }

    pub fn sample_rate(&self) -> u32 { self.sample_rate }

    /// Start streaming, invoking `cb` for each chunk of i16 samples.
    pub fn start<F>(&mut self, mut cb: F) -> Result<()>
    where
        F: FnMut(&[i16]) + Send + 'static,
    {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .context("no default input device")?;
        let cfg = cpal::StreamConfig {
            channels: 1,
            sample_rate: SampleRate(self.sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        let err_fn = |err| tracing::error!(?err, "cpal error");
        let buffer = self.buffer.clone();

        let stream = match device.default_input_config()? {
            cpal::SupportedStreamConfig { sample_rate: sr, channels: ch, sample_format: f, .. } => {
                let sr_in = sr.0;
                // We re-build config to the requested rate.
                let cfg2 = cpal::StreamConfig { channels: 1, sample_rate: SampleRate(self.sample_rate), buffer_size: cpal::BufferSize::Default };
                let buf = buffer.clone();
                let mut cb_inner = |data: &[f32]| {
                    // Resample trivially: take 1 sample of N (or average).
                    let ratio = (sr_in as f32) / (self.sample_rate as f32);
                    if (ratio - 1.0).abs() < 0.05 {
                        let v: Vec<i16> = data.iter().map(|s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16).collect();
                        let mut b = buf.lock();
                        b.extend_from_slice(&v);
                        cb(&v);
                    } else {
                        // simple nearest-neighbor downsample
                        let mut v: Vec<i16> = Vec::with_capacity((data.len() as f32 / ratio) as usize);
                        let mut i = 0.0f32;
                        while (i as usize) < data.len() {
                            let s = data[i as usize];
                            v.push((s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16);
                            i += ratio;
                        }
                        let mut b = buf.lock();
                        b.extend_from_slice(&v);
                        cb(&v);
                    }
                };
                let _ = f; let _ = ch;
                let stream = device.build_input_stream(
                    &cfg2,
                    move |data: &[f32], _| cb_inner(data),
                    err_fn,
                    None,
                )?;
                let _ = cfg;
                stream
            }
        };

        stream.play()?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(s) = self.stream.take() { let _ = s.pause(); }
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) { self.stop(); }
}

#[allow(dead_code)]
fn _silence(_: SampleFormat) -> Result<()> { Err(anyhow!("unused")) }
