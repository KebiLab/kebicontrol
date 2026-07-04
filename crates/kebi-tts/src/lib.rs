//! TTS stub. Real SAPI integration via COM is in TODO; for now TTS
//! is logged and (optionally) sent to SAPI via PowerShell.
//! Made by KebiLab

use anyhow::Result;
use std::process::Command;

pub struct Tts {
    rate: i32,
    volume: u8,
}

impl Tts {
    pub fn new() -> Result<Self> { Ok(Self { rate: 0, volume: 100 }) }
    pub fn set_rate(&mut self, rate: i32) -> Result<()> { self.rate = rate; Ok(()) }
    pub fn set_volume(&mut self, vol: u8) -> Result<()> { self.volume = vol.min(100); Ok(()) }

    pub fn speak_async(&self, text: &str) -> Result<()> {
        let escaped = text.replace('\'', "''");
        let script = format!(
            "Add-Type -AssemblyName System.Speech; \
             $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
             $s.Rate = {rate}; $s.Volume = {vol}; \
             $s.SpeakAsync('{escaped}') | Out-Null",
            rate = self.rate,
            vol = self.volume,
            escaped = escaped,
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .spawn();
        Ok(())
    }

    pub fn speak_blocking(&self, text: &str) -> Result<()> {
        let escaped = text.replace('\'', "''");
        let script = format!(
            "Add-Type -AssemblyName System.Speech; \
             $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
             $s.Rate = {rate}; $s.Volume = {vol}; \
             $s.Speak('{escaped}')",
            rate = self.rate,
            vol = self.volume,
            escaped = escaped,
        );
        Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .output()?;
        Ok(())
    }
}
