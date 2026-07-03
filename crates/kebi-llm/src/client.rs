//! LLM HTTP client. Made by KebiLab

use super::{ChatMessage, ChatResponse};
use anyhow::{Context, Result};
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    temperature: f32,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize)]
struct ResponseFormat { #[serde(rename = "type")] ty: String }

pub struct LlmClient {
    base_url: String,
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl LlmClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>, model: impl Into<String>, timeout_secs: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs.max(5)))
            .build()
            .unwrap_or_default();
        Self { base_url: base_url.into(), api_key: api_key.into(), model: model.into(), client }
    }

    pub fn model(&self) -> &str { &self.model }
    pub fn set_model(&mut self, m: impl Into<String>) { self.model = m.into(); }
    pub fn set_base_url(&mut self, u: impl Into<String>) { self.base_url = u.into(); }

    pub async fn chat_json(&self, messages: Vec<ChatMessage>) -> Result<String> {
        self.chat(messages, true).await
    }

    pub async fn chat_text(&self, messages: Vec<ChatMessage>) -> Result<String> {
        self.chat(messages, false).await
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>, json_mode: bool) -> Result<String> {
        let req = ChatRequest {
            model: &self.model,
            messages: &messages,
            temperature: 0.0,
            max_tokens: 600,
            response_format: if json_mode { Some(ResponseFormat { ty: "json_object".into() }) } else { None },
        };
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let mut rb = self.client.post(url).json(&req);
        if !self.api_key.is_empty() {
            rb = rb.bearer_auth(&self.api_key);
        }
        let resp = rb.send().await.context("LLM http")?;
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            anyhow::bail!("LLM status {status}: {body}");
        }
        let parsed: ChatResponse = serde_json::from_str(&body).context("LLM parse")?;
        let content = parsed.choices.first().map(|c| c.message.content.clone()).unwrap_or_default();
        Ok(content)
    }
}
