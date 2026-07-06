//! Minimalist i18n. Made by KebiLab

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
    pub fn code(&self) -> &'static str { match self { Lang::Ru => "ru", Lang::En => "en" } }
    pub fn label(self) -> &'static str { match self { Lang::Ru => "Русский", Lang::En => "English" } }
}

static RU: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
static EN: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn ru() -> &'static HashMap<&'static str, &'static str> {
    RU.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("app.name", "KebiControl");
        m.insert("app.tagline", "Голосовое управление Windows");
        m.insert("app.by", "Made by KebiLab");
        m.insert("home.input_hint", "Скажите или введите команду…");
        m.insert("settings.title", "Настройки");
        m.insert("settings.field.provider", "Провайдер");
        m.insert("settings.field.model", "Модель");
        m.insert("settings.field.apikey", "API-ключ");
        m.insert("settings.save", "Сохранить");
        m.insert("settings.cancel", "Отмена");
        m.insert("theme.dark", "Тёмная");
        m.insert("theme.light", "Светлая");
        m
    })
}
fn en() -> &'static HashMap<&'static str, &'static str> {
    EN.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("app.name", "KebiControl");
        m.insert("app.tagline", "Voice control for Windows");
        m.insert("app.by", "Made by KebiLab");
        m.insert("home.input_hint", "Say or type a command…");
        m.insert("settings.title", "Settings");
        m.insert("settings.field.provider", "Provider");
        m.insert("settings.field.model", "Model");
        m.insert("settings.field.apikey", "API key");
        m.insert("settings.save", "Save");
        m.insert("settings.cancel", "Cancel");
        m.insert("theme.dark", "Dark");
        m.insert("theme.light", "Light");
        m
    })
}

pub fn t(lang: Lang, key: &str) -> String {
    let d = match lang { Lang::Ru => ru(), Lang::En => en() };
    d.get(key).copied().unwrap_or(key).to_string()
}
