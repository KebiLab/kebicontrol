//! Provider presets. Made by KebiLab

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LlmProvider {
    OpenCode,
    OpenAI,
    Anthropic,
    Google,
    Mistral,
    Groq,
    DeepSeek,
    XAI,
    Custom,
}

impl LlmProvider {
    pub fn from_code(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "openai" => LlmProvider::OpenAI,
            "anthropic" => LlmProvider::Anthropic,
            "google" | "gemini" => LlmProvider::Google,
            "mistral" => LlmProvider::Mistral,
            "groq" => LlmProvider::Groq,
            "deepseek" => LlmProvider::DeepSeek,
            "xai" => LlmProvider::XAI,
            "custom" => LlmProvider::Custom,
            _ => LlmProvider::OpenCode,
        }
    }
    pub fn code(&self) -> &'static str {
        match self {
            LlmProvider::OpenCode => "opencode",
            LlmProvider::OpenAI => "openai",
            LlmProvider::Anthropic => "anthropic",
            LlmProvider::Google => "google",
            LlmProvider::Mistral => "mistral",
            LlmProvider::Groq => "groq",
            LlmProvider::DeepSeek => "deepseek",
            LlmProvider::XAI => "xai",
            LlmProvider::Custom => "custom",
        }
    }
    pub fn all() -> &'static [LlmProvider] {
        &[
            LlmProvider::OpenCode,
            LlmProvider::OpenAI,
            LlmProvider::Anthropic,
            LlmProvider::Google,
            LlmProvider::Mistral,
            LlmProvider::Groq,
            LlmProvider::DeepSeek,
            LlmProvider::XAI,
            LlmProvider::Custom,
        ]
    }
    pub fn default_base_url(&self) -> &'static str {
        ProviderPresets::default_base_url(*self)
    }
    pub fn default_model(&self) -> &'static str {
        ProviderPresets::default_model(*self)
    }
    pub fn default_models(&self) -> &'static [&'static str] {
        ProviderPresets::default_models(*self)
    }
}

pub struct ProviderPresets;

impl ProviderPresets {
    pub fn default_base_url(p: LlmProvider) -> &'static str {
        match p {
            LlmProvider::OpenCode => "https://api.opencode.ai/v1",
            LlmProvider::OpenAI => "https://api.openai.com/v1",
            LlmProvider::Anthropic => "https://api.anthropic.com/v1",
            LlmProvider::Google => "https://generativelanguage.googleapis.com/v1beta/openai",
            LlmProvider::Mistral => "https://api.mistral.ai/v1",
            LlmProvider::Groq => "https://api.groq.com/openai/v1",
            LlmProvider::DeepSeek => "https://api.deepseek.com/v1",
            LlmProvider::XAI => "https://api.x.ai/v1",
            LlmProvider::Custom => "https://api.example.com/v1",
        }
    }
    pub fn default_model(p: LlmProvider) -> &'static str {
        match p {
            LlmProvider::OpenCode => "deepseek-v4-flash",
            LlmProvider::OpenAI => "gpt-4o-mini",
            LlmProvider::Anthropic => "claude-3-5-sonnet-latest",
            LlmProvider::Google => "gemini-2.0-flash",
            LlmProvider::Mistral => "mistral-large-latest",
            LlmProvider::Groq => "llama-3.1-70b-versatile",
            LlmProvider::DeepSeek => "deepseek-chat",
            LlmProvider::XAI => "grok-2-latest",
            LlmProvider::Custom => "",
        }
    }
    pub fn default_models(p: LlmProvider) -> &'static [&'static str] {
        match p {
            LlmProvider::OpenCode => &["deepseek-v4-flash", "minimax-m3", "mimo-v2.5", "nvidia/nemotron"],
            LlmProvider::OpenAI => &["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "o1", "o1-mini", "o3-mini"],
            LlmProvider::Anthropic => &["claude-3-5-sonnet-latest", "claude-3-5-haiku-latest", "claude-3-opus-latest"],
            LlmProvider::Google => &["gemini-2.0-flash", "gemini-1.5-pro", "gemini-1.5-flash"],
            LlmProvider::Mistral => &["mistral-large-latest", "mistral-small-latest", "codestral-latest"],
            LlmProvider::Groq => &["llama-3.1-70b-versatile", "llama-3.1-8b-instant", "mixtral-8x7b-32768"],
            LlmProvider::DeepSeek => &["deepseek-chat", "deepseek-reasoner"],
            LlmProvider::XAI => &["grok-2-latest", "grok-2-mini", "grok-beta"],
            LlmProvider::Custom => &[],
        }
    }
    /// Whether the provider needs a special client (Anthropic uses /v1/messages).
    pub fn is_native(p: LlmProvider) -> bool {
        matches!(p, LlmProvider::Anthropic)
    }
}

// Back-compat helpers used by the old code path
pub fn base_url(p: LlmProvider) -> &'static str { ProviderPresets::default_base_url(p) }
pub fn default_model(p: LlmProvider) -> &'static str { ProviderPresets::default_model(p) }
