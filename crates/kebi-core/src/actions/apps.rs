//! App launching / closing. Made by KebiLab

use crate::error::{KebiError, Result};
use std::process::Command as StdCommand;
use sysinfo::{ProcessExt, System, SystemExt};

pub async fn open(target: &str) -> Result<Option<String>> {
    let t = target.trim();
    let lower = t.to_lowercase();
    // If it's a URL or scheme (http://, https://, ms-settings:, etc.)
    if t.contains("://") || t.starts_with("ms-") || t.starts_with("shell:") {
        StdCommand::new("cmd")
            .args(["/C", "start", "", t])
            .spawn()
            .map_err(|e| KebiError::Action(format!("open url: {e}")))?;
        return Ok(Some(format!("Открываю {t}")));
    }

    // Common shortcuts
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

pub async fn close(name: &str, force: bool) -> Result<Option<String>> {
    let mut sys = System::new_all();
    sys.refresh_processes();
    let needle = name.to_lowercase();
    let mut killed = 0usize;
    for p in sys.processes_by_name(&name.replace(' ', "")) {
        if p.name().to_lowercase().contains(&needle) {
            if !p.kill() {
                return Err(KebiError::Action(format!("Не удалось закрыть {name}")));
            }
            killed += 1;
        }
    }
    if killed == 0 && !force {
        // Fallback: taskkill
        let _ = StdCommand::new("taskkill")
            .args(["/IM", name, "/F"])
            .output();
    }
    Ok(Some(format!("Закрыто: {name}")))
}
