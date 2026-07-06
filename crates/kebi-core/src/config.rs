//! Configuration loading/saving. Made by KebiLab

use crate::app_paths::AppPaths;
use crate::error::Result;
use crate::secrets;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub hotkeys: HotkeyConfig,
    #[serde(default)]
    pub audio: AudioConfig,
    #[serde(default)]
    pub stt: SttConfig,
    #[serde(default)]
    pub llm: LlmConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub tts: TtsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            hotkeys: HotkeyConfig::default(),
            audio: AudioConfig::default(),
            stt: SttConfig::default(),
            llm: LlmConfig::default(),
            ui: UiConfig::default(),
            tts: TtsConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub language: String,
    pub wake_word: String,
    pub wake_word_enabled: bool,
    pub autostart: bool,
    pub tts_enabled: bool,
    pub log_level: String,
    pub active_profile: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            language: "ru".into(),
            wake_word: "кеби".into(),
            wake_word_enabled: false,
            autostart: false,
            tts_enabled: true,
            log_level: "info".into(),
            active_profile: "default".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub push_to_listen: String,
    pub overlay: String,
    pub cancel: String,
    pub toggle_tts: String,
    pub dictation: String,
    pub pause: String,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            push_to_listen: "Ctrl+Shift+Space".into(),
            overlay: "Ctrl+Shift+K".into(),
            cancel: "Escape".into(),
            toggle_tts: "Ctrl+Shift+M".into(),
            dictation: "Ctrl+Shift+D".into(),
            pause: "Ctrl+Shift+P".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub device_name: String,
    pub sample_rate: u32,
    pub vad_threshold: f32,
    pub silence_ms: u64,
    pub max_phrase_ms: u64,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            device_name: "default".into(),
            sample_rate: 16000,
            vad_threshold: 0.012,
            silence_ms: 700,
            max_phrase_ms: 8000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttConfig {
    pub engine: String,           // "whisper" (only whisper is used in current build)
    pub whisper_endpoint: String,
    pub whisper_model: String,
    pub whisper_language: String,
    pub api_key_enc: String,      // DPAPI-encrypted
    pub auto_download_model: bool,
}

impl Default for SttConfig {
    fn default() -> Self {
        Self {
            engine: "whisper".into(),
            whisper_endpoint: "https://api.openai.com/v1/audio/transcriptions".into(),
            whisper_model: "whisper-1".into(),
            whisper_language: "ru".into(),
            api_key_enc: String::new(),
            auto_download_model: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,     // "opencode" | "deepseek" | "mimo" | "nvidia" | "custom"
    pub base_url: String,
    pub model: String,
    /// DPAPI-encrypted key, base64.
    pub api_key_enc: String,
    pub timeout_secs: u64,
    pub max_context_turns: usize,
    pub custom: Option<CustomLlm>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "opencode".into(),
            base_url: "https://api.opencode.ai/v1".into(),
            model: "deepseek-v4-flash".into(),
            api_key_enc: String::new(),
            timeout_secs: 30,
            max_context_turns: 6,
            custom: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomLlm {
    pub base_url: String,
    pub model: String,
    pub api_key_enc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub opacity: f32,
    pub show_overlay_on_start: bool,
    pub minimize_to_tray: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "midnight".into(),
            opacity: 0.92,
            show_overlay_on_start: false,
            minimize_to_tray: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsConfig {
    pub voice: String,
    pub rate: i32,
    pub volume: u8,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            voice: "ru-RU, Irina".into(),
            rate: 0,
            volume: 100,
        }
    }
}

impl Config {
    pub fn load(paths: &AppPaths) -> Result<Self> {
        let path = &paths.config;
        if !path.exists() {
            // Seed from bundled example or defaults.
            let cfg = Config::default();
            cfg.save(paths)?;
            return Ok(cfg);
        }
        let s = std::fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&s)
            .map_err(|e| crate::error::KebiError::Config(e.to_string()))?;
        Ok(cfg)
    }

    pub fn save(&self, paths: &AppPaths) -> Result<()> {
        let s = toml::to_string_pretty(self)?;
        std::fs::write(&paths.config, s)?;
        Ok(())
    }

    pub fn get_api_key(&self) -> Option<String> {
        let enc = if self.llm.provider == "custom" {
            self.llm.custom.as_ref().map(|c| c.api_key_enc.clone()).unwrap_or_default()
        } else {
            self.llm.api_key_enc.clone()
        };
        if enc.is_empty() {
            return None;
        }
        secrets::unprotect(&enc).ok()
    }

    pub fn set_api_key(&mut self, key: &str) -> Result<()> {
        let enc = secrets::protect(key)
            .map_err(|e| crate::error::KebiError::Config(e.to_string()))?;
        if self.llm.provider == "custom" {
            if let Some(c) = self.llm.custom.as_mut() {
                c.api_key_enc = enc;
            }
        } else {
            self.llm.api_key_enc = enc;
        }
        Ok(())
    }

    pub fn get_stt_api_key(&self) -> Option<String> {
        if self.stt.api_key_enc.is_empty() { return None; }
        secrets::unprotect(&self.stt.api_key_enc).ok()
    }

    pub fn set_stt_api_key(&mut self, key: &str) -> Result<()> {
        let enc = secrets::protect(key)
            .map_err(|e| crate::error::KebiError::Config(e.to_string()))?;
        self.stt.api_key_enc = enc;
        Ok(())
    }
}

/// Read config from a specific path, useful for tests.
pub fn read_from(path: &Path) -> Result<Config> {
    let s = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&s)?)
}
