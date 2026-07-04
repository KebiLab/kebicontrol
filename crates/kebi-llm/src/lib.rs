//! LLM client. OpenAI-compatible. Made by KebiLab

pub mod providers;
pub mod client;
pub mod anthropic;
pub mod prompt;

pub use client::LlmClient;
pub use providers::{LlmProvider, ProviderPresets};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ChoiceMessage,
}

#[derive(Debug, Deserialize)]
pub struct ChoiceMessage {
    pub content: String,
}
