//! Anthropic Messages API client. Made by KebiLab

use super::{ChatMessage, ChatResponse};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize)]
struct AnthropicRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    system: Option<String>,
    messages: Vec<AnthropicMessage>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResp {
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    ty: String,
    text: Option<String>,
}

pub struct AnthropicClient {
    api_key: String,
    model: String,
    base_url: String,
    version: String,
    client: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>, model: impl Into<String>, timeout_secs: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs.max(5)))
            .build()
            .unwrap_or_default();
        Self {
            api_key: api_key.into(),
            model: model.into(),
            base_url: base_url.into(),
            version: "2023-06-01".into(),
            client,
        }
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>, json_mode: bool) -> Result<String> {
        let mut sys: Option<String> = None;
        let mut msgs: Vec<AnthropicMessage> = Vec::new();
        for m in messages {
            match m.role.as_str() {
                "system" => sys = Some(m.content),
                "user" => msgs.push(AnthropicMessage { role: "user".into(), content: m.content }),
                "assistant" => msgs.push(AnthropicMessage { role: "assistant".into(), content: m.content }),
                _ => {}
            }
        }
        // If json_mode and a system prompt exists, hint it; Anthropic has no json_object mode.
        if json_mode {
            let hint = "\n\nReply ONLY with valid JSON.".to_string();
            sys = Some(match sys {
                Some(s) => s + &hint,
                None => hint.trim_start().to_string(),
            });
        }

        let req = AnthropicRequest {
            model: &self.model,
            max_tokens: 1024,
            system: sys,
            messages: msgs,
            temperature: 0.0,
        };

        let url = format!("{}/messages", self.base_url.trim_end_matches('/'));
        let resp = self.client.post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.version)
            .header("content-type", "application/json")
            .json(&req)
            .send()
            .await
            .context("anthropic http")?;

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            anyhow::bail!("anthropic status {status}: {body}");
        }
        let parsed: AnthropicResp = serde_json::from_str(&body).context("anthropic parse")?;
        let text = parsed.content.iter()
            .filter_map(|c| c.text.clone())
            .collect::<Vec<_>>()
            .join("\n");
        // Return in unified shape so caller can deserialize to ChatResponse if needed.
        let unified = ChatResponse {
            choices: vec![super::Choice {
                message: super::ChoiceMessage { content: text },
            }],
        };
        Ok(unified.choices.into_iter().next().unwrap().message.content)
    }
}
