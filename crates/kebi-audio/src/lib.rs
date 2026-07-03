//! Audio capture + VAD. Made by KebiLab

pub mod capture;
pub mod vad;

pub use capture::AudioCapture;
pub use vad::Vad;
