//! Action executors. Made by KebiLab

pub mod system;
pub mod apps;
pub mod window;
pub mod input;
pub mod media;
pub mod web;
pub mod dictation;

use crate::command::Command;
use crate::error::{KebiError, Result};

/// Execute a parsed command. Returns optional text to speak back to the user.
pub async fn execute(cmd: &Command) -> Result<Option<String>> {
    use Command::*;
    let reply = match cmd {
        Say { text } => {
            system::speak(text).await?;
            Some(text.clone())
        }
        Open { target } => apps::open(target).await?,
        Close { name, force } => apps::close(name, *force).await?,
        Focus { name } => window::focus(name).await?,
        Volume { op, value } => system::volume(*op, *value).await?,
        Brightness { value } => system::brightness(*value).await?,
        Window { op, target } => window::apply(*op, target.as_deref()).await?,
        Input { op, text } => input::apply(*op, text.as_deref()).await?,
        Media { op } => media::apply(*op).await?,
        Screenshot { mode } => system::screenshot(*mode).await?,
        Web { op, query } => web::apply(*op, query).await?,
        Remind { kind, value, text } => system::remind(*kind, value, text.as_deref()).await?,
        Power { op } => system::power(*op).await?,
        ToggleTts => {
            system::toggle_tts()?;
            Some(if system::tts_enabled() { "Голос включён" } else { "Голос выключен" }.to_string())
        }
        Help => Some(window::help_text()),
        Dictation { on } => {
            dictation::set_active(*on).await?;
            Some(if *on { "Режим диктовки включён" } else { "Режим диктовки выключен" }.to_string())
        }
        Chat { text } => Some(text.clone()),
        Unknown { reason } => Some(format!("Не понял: {reason}")),
    };
    Ok(reply)
}

pub fn err(s: impl Into<String>) -> KebiError {
    KebiError::Action(s.into())
}
