//! LLM fallback parser. Calls OpenAI-compatible /chat/completions.
//! Made by KebiLab

use crate::command::{Command, Confidence, ParseSource, ParsedCommand};
use crate::i18n::Lang;
use crate::parser::ParserContext;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
    stream: bool,
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    ty: String,
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

pub fn system_prompt(lang: Lang) -> String {
    let lang_str = match lang {
        Lang::Ru => "Russian",
        Lang::En => "English",
    };
    format!(
        "You are KebiControl, a Windows voice assistant. The user speaks in {lang_str}.\n\
         Convert the user's command into a JSON object matching the schema:\n\
         {{\"action\": \"...\", \"args\": {{...}}}}\n\
         Allowed actions: open, close, focus, web_search, web_open, volume, brightness, window,\n\
         input, media, screenshot, power, remind, say, chat, toggle_tts, help, dictation.\n\
         If the user just wants to chat, return {{\"action\":\"chat\",\"args\":{{\"text\":\"<reply>\"}}}}.\n\
         Reply ONLY with valid JSON, no prose."
    )
}

pub fn user_context(ctx: &ParserContext, text: &str) -> String {
    let aliases = ctx
        .profile
        .aliases
        .0
        .iter()
        .map(|(k, v)| format!("  {k} -> {v}"))
        .collect::<Vec<_>>()
        .join("\n");
    let apps = ctx
        .profile
        .apps
        .items
        .iter()
        .map(|(k, v)| format!("  {k} -> {v}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("Command: {text}\n\nAliases:\n{aliases}\n\nApps:\n{apps}")
}

pub async fn parse_with_llm(
    ctx: &ParserContext,
    text: &str,
    base_url: &str,
    api_key: &str,
    model: &str,
) -> anyhow::Result<ParsedCommand> {
    use anyhow::Context;
    let lang = Lang::from_code(ctx.language);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let body = ChatRequest {
        model,
        messages: vec![
            ChatMessage { role: "system".into(), content: system_prompt(lang) },
            ChatMessage { role: "user".into(), content: user_context(ctx, text) },
        ],
        temperature: 0.0,
        max_tokens: 400,
        stream: false,
        response_format: Some(ResponseFormat { ty: "json_object".into() }),
    };

    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let resp = client
        .post(url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .context("LLM HTTP request failed")?;

    let status = resp.status();
    let body_text = resp.text().await.unwrap_or_default();
    if !status.is_success() {
        anyhow::bail!("LLM status {status}: {body_text}");
    }
    let parsed: ChatResponse = serde_json::from_str(&body_text)
        .context("LLM response parse failed")?;
    let content = parsed
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    let value: serde_json::Value = serde_json::from_str(&content)
        .context("LLM returned non-JSON")?;
    let cmd: Command = serde_json::from_value(value)
        .context("LLM JSON did not match Command schema")?;

    Ok(ParsedCommand {
        command: cmd,
        confidence: Confidence::MEDIUM,
        source: ParseSource::Llm,
    })
}
