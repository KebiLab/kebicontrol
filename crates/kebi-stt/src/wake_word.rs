//! Wake word detection: short audio chunks are sent to the configured
//! Whisper endpoint and checked for the wake phrase. This is a simple
//! cloud-based approach that requires no extra local model.
//!
//! Made by KebiLab

use anyhow::Result;

/// Checks if a given text starts with the wake phrase (case-insensitive).
pub fn contains_wake(text: &str, wake: &str) -> bool {
    let t = text.to_lowercase();
    let w = wake.to_lowercase();
    // Match "кеби" or "кэби" (common spellings) and skip leading filler.
    if t.contains(&w) {
        return true;
    }
    // Common Russian renderings of "Kebi"
    if wake.eq_ignore_ascii_case("kebi") || wake.eq_ignore_ascii_case("кеби") {
        for alt in ["кэби", "кеби", "kebi", "кебби", "кеби,"] {
            if t.contains(alt) {
                return true;
            }
        }
    }
    false
}

/// Resets the wake word detector state.
pub struct WakeWordDetector;

impl WakeWordDetector {
    /// Stub: the actual cloud-based detection runs in the async pipeline.
    pub fn try_new(_model_path: &str, _keywords: &[&str]) -> Result<Self> {
        Ok(Self)
    }

    pub fn is_ready(&self) -> bool {
        true
    }

    pub fn reset(&self) {
        // No-op for cloud detector.
    }
}
