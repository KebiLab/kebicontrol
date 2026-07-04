//! App launching / closing. Made by KebiLab

use crate::error::{KebiError, Result};
use std::process::Command as StdCommand;

pub async fn open(target: &str) -> Result<Option<String>> {
    let t = target.trim();
    let lower = t.to_lowercase();
    let candidate: Option<&str> = match lower.as_str() {
        "браузер" | "browser" | "интернет" => Some("msedge"),
        "проводник" | "explorer" | "files" => Some("explorer"),
        "терминал" | "terminal" | "консоль" => Some("wt"),
        "блокнот" | "notepad" => Some("notepad"),
        "калькулятор" | "calculator" | "calc" => Some("calc"),
        "настройки" | "settings" => Some("ms-settings:"),
        "дискорд" | "discord" => Some("Discord"),
        "стим" | "steam" => Some("steam"),
        "телеграм" | "telegram" => Some("telegram"),
        "ютуб" | "youtube" => Some("https://youtube.com"),
        "гугл" | "google" => Some("https://google.com"),
        _ => None,
    };
    let resolved = candidate.unwrap_or(t);
    if resolved.contains("://") {
        StdCommand::new("cmd")
            .args(["/C", "start", "", resolved])
            .spawn()
            .map_err(|e| KebiError::Action(format!("open url: {e}")))?;
        return Ok(Some(format!("Открываю {resolved}")));
    }
    StdCommand::new("cmd")
        .args(["/C", "start", "", resolved])
        .spawn()
        .map_err(|e| KebiError::Action(format!("open {t}: {e}")))?;
    Ok(Some(format!("Открываю {t}")))
}

pub async fn close(name: &str, _force: bool) -> Result<Option<String>> {
    let _ = StdCommand::new("taskkill")
        .args(["/IM", name, "/F"])
        .output();
    Ok(Some(format!("Закрыто: {name}")))
}
