//! Microphone capture. Made by KebiLab

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, Stream};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct AudioCapture {
    stream: Option<Stream>,
    buffer: Arc<Mutex<Vec<i16>>>,
    sample_rate: u32,
}

impl AudioCapture {
    pub fn new(sample_rate: u32) -> Self {
        Self { stream: None, buffer: Arc::new(Mutex::new(Vec::new())), sample_rate }
    }

    pub fn buffer(&self) -> Arc<Mutex<Vec<i16>>> { self.buffer.clone() }
    pub fn sample_rate(&self) -> u32 { self.sample_rate }

    pub fn start<F>(&mut self, mut cb: F) -> Result<()>
    where
        F: FnMut(&[i16]) + Send + 'static,
    {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| anyhow::anyhow!("no input device"))?;
        let cfg_in = device.default_input_config()?;
        let sr_in = cfg_in.sample_rate().0;
        let buf = self.buffer.clone();
        let sr_out = self.sample_rate;

        let stream = device.build_input_stream(
            &cfg_in.config(),
            move |data: &[f32], _| {
                let ratio = (sr_in as f32) / (sr_out as f32);
                if (ratio - 1.0).abs() < 0.05 {
                    let v: Vec<i16> = data.iter()
                        .map(|s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                        .collect();
                    let mut b = buf.lock();
                    b.extend_from_slice(&v);
                    cb(&v);
                } else {
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
            },
            |err| tracing::error!(?err, "cpal"),
            None,
        )?;
        stream.play()?;
        self.stream = Some(stream);
        let _ = SampleRate(0);
        let _ = SampleFormat::F32;
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(s) = self.stream.take() { let _ = s.pause(); }
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) { self.stop(); }
}
