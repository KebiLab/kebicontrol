//! Error types. Made by KebiLab

use thiserror::Error;

#[derive(Debug, Error)]
pub enum KebiError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Serialization error: {0}")]
    Serde(#[from] toml::de::Error),

    #[error("Serialization error: {0}")]
    SerdeSer(#[from] toml::ser::Error),

    #[error("Audio error: {0}")]
    Audio(String),

    #[error("STT error: {0}")]
    Stt(String),

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("Action error: {0}")]
    Action(String),

    #[error("Windows API error: {0}")]
    WinApi(String),

    #[error("Profile error: {0}")]
    Profile(String),

    #[error("Other: {0}")]
    Other(String),
}

impl From<anyhow::Error> for KebiError {
    fn from(e: anyhow::Error) -> Self {
        KebiError::Other(format!("{e:#}"))
    }
}

impl From<&str> for KebiError {
    fn from(s: &str) -> Self {
        KebiError::Other(s.to_string())
    }
}

impl From<String> for KebiError {
    fn from(s: String) -> Self {
        KebiError::Other(s)
    }
}

pub type Result<T> = std::result::Result<T, KebiError>;
