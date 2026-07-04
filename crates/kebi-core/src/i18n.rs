//! I18n. Made by KebiLab

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang { Ru, En }

impl Lang {
    pub fn from_code(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "en" => Lang::En,
            _ => Lang::Ru,
        }
    }
}

pub fn dict(lang: Lang) -> &'static HashMap<&'static str, &'static str> {
    match lang { Lang::Ru => ru(), Lang::En => en() }
}

pub fn t(lang: Lang, key: &str) -> String {
    dict(lang).get(key).copied().unwrap_or(key).to_string()
}

static RU: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
static EN: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn ru() -> &'static HashMap<&'static str, &'static str> {
    RU.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("app.name", "KebiControl");
        m.insert("app.by", "Made by KebiLab");
        m.insert("status.listening", "Слушаю");
        m.insert("status.idle", "Готов");
        m.insert("status.thinking", "Думаю");
        m.insert("status.error", "Ошибка");
        m.insert("cmd.unknown", "Не понял команду");
        m.insert("cmd.ok", "Готово");
        m.insert("help.title", "Горячие клавиши");
        m.insert("llm.missing_key", "API-ключ не задан. Откройте Настройки.");
        m.insert("llm.error", "Ошибка LLM");
        m.insert("overlay.search_hint", "Скажите команду или введите её");
        m
    })
}

fn en() -> &'static HashMap<&'static str, &'static str> {
    EN.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("app.name", "KebiControl");
        m.insert("app.by", "Made by KebiLab");
        m.insert("status.listening", "Listening");
        m.insert("status.idle", "Ready");
        m.insert("status.thinking", "Thinking");
        m.insert("status.error", "Error");
        m.insert("cmd.unknown", "Unknown command");
        m.insert("cmd.ok", "Done");
        m.insert("help.title", "Hotkeys");
        m.insert("llm.missing_key", "API key is not set. Open Settings.");
        m.insert("llm.error", "LLM error");
        m.insert("overlay.search_hint", "Say a command or type it");
        m
    })
}
