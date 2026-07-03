//! Text-to-speech via Windows SAPI. Made by KebiLab

use anyhow::Result;
use windows::core::Interface;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Accessibility::{ISpeechVoice, SpVoice};

/// Minimal SAPI wrapper.
pub struct Tts {
    voice: ISpeechVoice,
}

impl Tts {
    pub fn new() -> Result<Self> {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            let voice: ISpeechVoice = CoCreateInstance(&SpVoice, None, CLSCTX_ALL)?;
            Ok(Self { voice })
        }
    }

    pub fn speak_blocking(&self, text: &str) -> Result<()> {
        unsafe {
            let bstr = windows::core::BSTR::from(text);
            // Use SyncSpeak (SVSFDefault) — flags = 0
            self.voice.Speak(&bstr, 0.into(), None)?;
        }
        Ok(())
    }

    pub fn speak_async(&self, text: &str) -> Result<()> {
        unsafe {
            let bstr = windows::core::BSTR::from(text);
            // Async flag = 1
            self.voice.Speak(&bstr, 1.into(), None)?;
        }
        Ok(())
    }

    pub fn set_rate(&self, rate: i32) -> Result<()> {
        unsafe { self.voice.Set_Rate(rate)?; Ok(()) }
    }

    pub fn set_volume(&self, vol: u8) -> Result<()> {
        unsafe { self.voice.Set_Volume(vol as i32)?; Ok(()) }
    }
}
