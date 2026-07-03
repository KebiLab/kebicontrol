//! Streaming dictation mode. Made by KebiLab

use std::sync::atomic::{AtomicBool, Ordering};

static ACTIVE: AtomicBool = AtomicBool::new(false);

pub async fn set_active(on: bool) -> crate::error::Result<()> {
    ACTIVE.store(on, Ordering::SeqCst);
    Ok(())
}

pub fn is_active() -> bool { ACTIVE.load(Ordering::SeqCst) }
