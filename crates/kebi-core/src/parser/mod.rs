//! Command parsing: local rules first, LLM fallback.
//! Made by KebiLab

pub mod local;
pub mod llm;

use crate::command::{Command, ParseSource, ParsedCommand, Confidence};
use crate::profile::Profile;

pub struct ParserContext<'a> {
    pub profile: &'a Profile,
    pub wake_word: &'a str,
    pub language: &'a str,
}

/// Result of trying to understand a phrase.
pub enum ParseOutcome {
    /// Local rule matched.
    Local(ParsedCommand),
    /// Need to ask LLM.
    NeedLlm(String),
    /// Could not parse at all.
    Failed(String),
}

pub fn strip_wake<'a>(input: &'a str, wake: &str) -> &'a str {
    let trimmed = input.trim();
    let lower = trimmed.to_lowercase();
    let wlower = wake.to_lowercase();
    if let Some(rest) = lower.strip_prefix(&wlower) {
        let cut = (trimmed.len() - rest.len()).min(trimmed.len());
        trimmed[cut..].trim_start_matches(|c: char| c == ',' || c == ' ' || c == ':')
    } else {
        trimmed
    }
}

/// Try to parse the input. If local rules return Unknown, return NeedLlm.
pub fn parse(ctx: &ParserContext, input: &str) -> ParseOutcome {
    let text = strip_wake(input, ctx.wake_word);
    if text.is_empty() {
        return ParseOutcome::Local(ParsedCommand {
            command: Command::Unknown { reason: "empty".into() },
            confidence: Confidence::LOW,
            source: ParseSource::LocalRule,
        });
    }

    if let Some(cmd) = local::try_match(ctx, text) {
        return ParseOutcome::Local(cmd);
    }

    ParseOutcome::NeedLlm(text.to_string())
}
