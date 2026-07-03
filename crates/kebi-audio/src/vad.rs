//! Simple RMS-based voice activity detection. Made by KebiLab

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VadEvent {
    Silence,
    Speech,
}

pub struct Vad {
    pub threshold: f32,
    pub min_silence_ms: u64,
    pub min_speech_ms: u64,
    last_voice: Option<Instant>,
    last_silence: Option<Instant>,
    state: VadEvent,
}

impl Vad {
    pub fn new(threshold: f32, min_silence_ms: u64, min_speech_ms: u64) -> Self {
        Self {
            threshold,
            min_silence_ms,
            min_speech_ms,
            last_voice: None,
            last_silence: None,
            state: VadEvent::Silence,
        }
    }

    /// Process a chunk of i16 PCM samples. Returns state-changing events.
    pub fn process(&mut self, samples: &[i16]) -> VadEvent {
        let rms = rms_i16(samples);
        let is_voice = rms >= self.threshold;
        let now = Instant::now();
        let mut produced = VadEvent::Silence;

        if is_voice {
            self.last_voice = Some(now);
            if self.state == VadEvent::Silence {
                if self.last_silence
                    .map(|t| now.duration_since(t) >= Duration::from_millis(self.min_speech_ms))
                    .unwrap_or(true)
                {
                    self.state = VadEvent::Speech;
                    produced = VadEvent::Speech;
                }
            }
        } else {
            self.last_silence = Some(now);
            if self.state == VadEvent::Speech {
                if self.last_voice
                    .map(|t| now.duration_since(t) >= Duration::from_millis(self.min_silence_ms))
                    .unwrap_or(true)
                {
                    self.state = VadEvent::Silence;
                    produced = VadEvent::Silence;
                }
            }
        }
        produced
    }

    pub fn state(&self) -> VadEvent { self.state }
}

fn rms_i16(samples: &[i16]) -> f32 {
    if samples.is_empty() { return 0.0; }
    let sum: f64 = samples.iter().map(|s| {
        let v = *s as f64 / i16::MAX as f64;
        v * v
    }).sum();
    ((sum / samples.len() as f64) as f32).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silence_stays_silence() {
        let mut v = Vad::new(0.01, 50, 50);
        let z = vec![0i16; 1024];
        assert_eq!(v.process(&z), VadEvent::Silence);
    }

    #[test]
    fn speech_becomes_speech() {
        let mut v = Vad::new(0.01, 50, 0);
        let s: Vec<i16> = (0..1024).map(|i| ((i as f32 / 1024.0).sin() * 10000.0) as i16).collect();
        assert_eq!(v.process(&s), VadEvent::Speech);
    }
}
