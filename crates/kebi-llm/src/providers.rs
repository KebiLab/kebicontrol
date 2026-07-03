//! Provider presets. Made by KebiLab

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LlmProvider {
    OpenCode,
    DeepSeek,
    MiMo,
    Nvidia,
    Custom,
}

impl LlmProvider {
    pub fn from_code(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "deepseek" => LlmProvider::DeepSeek,
            "mimo" => LlmProvider::MiMo,
            "nvidia" => LlmProvider::Nvidia,
            "custom" => LlmProvider::Custom,
            _ => LlmProvider::OpenCode,
        }
    }
    pub fn code(&self) -> &'static str {
        match self {
            LlmProvider::OpenCode => "opencode",
            LlmProvider::DeepSeek => "deepseek",
            LlmProvider::MiMo => "mimo",
            LlmProvider::Nvidia => "nvidia",
            LlmProvider::Custom => "custom",
        }
    }
}

pub struct ProviderPresets;

impl ProviderPresets {
    pub fn base_url(p: LlmProvider) -> &'static str {
        match p {
            LlmProvider::OpenCode => "https://api.opencode.ai/v1",
            LlmProvider::DeepSeek => "https://api.deepseek.com/v1",
            LlmProvider::MiMo => "https://api.xiaomimimo.com/v1",
            LlmProvider::Nvidia => "https://integrate.api.nvidia.com/v1",
            LlmProvider::Custom => "https://api.opencode.ai/v1",
        }
    }
    pub fn default_model(p: LlmProvider) -> &'static str {
        match p {
            LlmProvider::OpenCode => "deepseek-v4-flash",
            LlmProvider::DeepSeek => "deepseek-v4-flash",
            LlmProvider::MiMo => "mimo-v2.5",
            LlmProvider::Nvidia => "nvidia/nemotron",
            LlmProvider::Custom => "deepseek-v4-flash",
        }
    }
}
