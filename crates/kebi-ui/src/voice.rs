//! Voice types and shared state. Made by KebiLab

use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum VoiceEvent {
    State(VoiceState),
    Heard(String),
    Reply(String),
    Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceState {
    Idle,
    Listening,
    Recognizing,
    Thinking,
    Speaking,
    Error,
}

#[derive(Debug, Clone)]
pub struct VoiceStateCell {
    inner: Arc<parking_lot::Mutex<VoiceState>>,
}

impl VoiceStateCell {
    pub fn new() -> Self { Self { inner: Arc::new(parking_lot::Mutex::new(VoiceState::Idle)) } }
    pub fn get(&self) -> VoiceState { *self.inner.lock() }
    pub fn set(&self, s: VoiceState) { *self.inner.lock() = s; }
}

impl Default for VoiceStateCell { fn default() -> Self { Self::new() } }

pub type VoiceReceiver = mpsc::UnboundedReceiver<VoiceEvent>;
pub type VoiceSender = mpsc::UnboundedSender<VoiceEvent>;

/// Shared buffer of audio samples collected from the microphone.
/// Lives in the pipeline thread; UI does not touch it.
pub struct AudioBuffer {
    inner: Arc<parking_lot::Mutex<Vec<i16>>>,
}

impl AudioBuffer {
    pub fn new() -> Self { Self { inner: Arc::new(parking_lot::Mutex::new(Vec::new())) } }
    pub fn push(&self, samples: &[i16]) {
        let mut g = self.inner.lock();
        g.extend_from_slice(samples);
    }
    pub fn drain(&self) -> Vec<i16> {
        let mut g = self.inner.lock();
        std::mem::take(&mut *g)
    }
}

impl Default for AudioBuffer { fn default() -> Self { Self::new() } }

/// UI-side controller. Holds the shared state cell. Cheap to clone.
pub struct VoiceController {
    state: VoiceStateCell,
}

impl VoiceController {
    pub fn new() -> Self { Self { state: VoiceStateCell::new() } }
    pub fn state(&self) -> VoiceStateCell { self.state.clone() }
}

impl Default for VoiceController { fn default() -> Self { Self::new() } }
