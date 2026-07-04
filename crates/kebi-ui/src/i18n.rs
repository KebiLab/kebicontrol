//! UI strings. Made by KebiLab

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
    pub fn code(&self) -> &'static str {
        match self { Lang::Ru => "ru", Lang::En => "en" }
    }
    pub fn label(&self) -> &'static str {
        match self { Lang::Ru => "Русский", Lang::En => "English" }
    }
}

static RU: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
static EN: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn ru() -> &'static HashMap<&'static str, &'static str> {
    RU.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("app.name", "KebiControl");
        m.insert("app.tagline", "Голосовое управление компьютером");
        m.insert("app.by", "Made by KebiLab");
        m.insert("nav.home", "Главная");
        m.insert("nav.history", "История");
        m.insert("nav.settings", "Настройки");
        m.insert("nav.hotkeys", "Горячие клавиши");
        m.insert("nav.about", "О программе");
        m.insert("status.idle", "Готов");
        m.insert("status.listening", "Слушаю");
        m.insert("status.thinking", "Думаю");
        m.insert("status.error", "Ошибка");
        m.insert("status.off", "Голос выключен");
        m.insert("home.greeting", "Привет");
        m.insert("home.subtitle", "Скажите «кеби» или введите команду ниже");
        m.insert("home.input_hint", "Например: громкость на 50, открой ютуб, скриншот");
        m.insert("home.run", "Выполнить");
        m.insert("home.quick", "Быстрые действия");
        m.insert("home.recent", "Последние команды");
        m.insert("home.empty", "Пока ничего. Попробуйте: кеби хелп");
        m.insert("history.title", "История команд");
        m.insert("history.empty", "История пуста");
        m.insert("history.clear", "Очистить");
        m.insert("settings.title", "Настройки");
        m.insert("settings.section.general", "Основные");
        m.insert("settings.section.llm", "Нейросеть");
        m.insert("settings.section.audio", "Аудио");
        m.insert("settings.section.appearance", "Внешний вид");
        m.insert("settings.field.wake", "Wake word");
        m.insert("settings.field.lang", "Язык интерфейса");
        m.insert("settings.field.tts", "Голосовые ответы");
        m.insert("settings.field.autostart", "Запускать с Windows");
        m.insert("settings.field.theme", "Тема");
        m.insert("settings.field.provider", "Провайдер");
        m.insert("settings.field.baseurl", "Base URL");
        m.insert("settings.field.model", "Модель");
        m.insert("settings.field.apikey", "API-ключ");
        m.insert("settings.field.mic", "Микрофон");
        m.insert("settings.save", "Сохранить");
        m.insert("settings.saved", "Сохранено");
        m.insert("settings.error", "Не удалось сохранить");
        m.insert("theme.midnight", "Полночь");
        m.insert("theme.dawn", "Рассвет");
        m.insert("theme.forest", "Лес");
        m.insert("hotkeys.title", "Горячие клавиши");
        m.insert("hotkeys.listen", "Слушать");
        m.insert("hotkeys.cancel", "Отмена");
        m.insert("hotkeys.tts", "Голос");
        m.insert("hotkeys.dictation", "Диктовка");
        m.insert("hotkeys.pause", "Пауза");
        m.insert("about.title", "О программе");
        m.insert("about.version", "Версия");
        m.insert("about.desc", "Голосовой ассистент для Windows. Распознаёт команды локально, нестандартные отправляет в LLM.");
        m.insert("cmd.unknown", "Не понял");
        m.insert("cmd.ok", "Готово");
        m
    })
}

fn en() -> &'static HashMap<&'static str, &'static str> {
    EN.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("app.name", "KebiControl");
        m.insert("app.tagline", "Voice control for your computer");
        m.insert("app.by", "Made by KebiLab");
        m.insert("nav.home", "Home");
        m.insert("nav.history", "History");
        m.insert("nav.settings", "Settings");
        m.insert("nav.hotkeys", "Hotkeys");
        m.insert("nav.about", "About");
        m.insert("status.idle", "Ready");
        m.insert("status.listening", "Listening");
        m.insert("status.thinking", "Thinking");
        m.insert("status.error", "Error");
        m.insert("status.off", "Voice off");
        m.insert("home.greeting", "Hello");
        m.insert("home.subtitle", "Say «kebi» or type a command below");
        m.insert("home.input_hint", "e.g. volume 50, open youtube, screenshot");
        m.insert("home.run", "Run");
        m.insert("home.quick", "Quick actions");
        m.insert("home.recent", "Recent commands");
        m.insert("home.empty", "Nothing yet. Try: kebi help");
        m.insert("history.title", "Command history");
        m.insert("history.empty", "History is empty");
        m.insert("history.clear", "Clear");
        m.insert("settings.title", "Settings");
        m.insert("settings.section.general", "General");
        m.insert("settings.section.llm", "Language model");
        m.insert("settings.section.audio", "Audio");
        m.insert("settings.section.appearance", "Appearance");
        m.insert("settings.field.wake", "Wake word");
        m.insert("settings.field.lang", "Interface language");
        m.insert("settings.field.tts", "Voice replies");
        m.insert("settings.field.autostart", "Launch with Windows");
        m.insert("settings.field.theme", "Theme");
        m.insert("settings.field.provider", "Provider");
        m.insert("settings.field.baseurl", "Base URL");
        m.insert("settings.field.model", "Model");
        m.insert("settings.field.apikey", "API key");
        m.insert("settings.field.mic", "Microphone");
        m.insert("settings.save", "Save");
        m.insert("settings.saved", "Saved");
        m.insert("settings.error", "Save failed");
        m.insert("theme.midnight", "Midnight");
        m.insert("theme.dawn", "Dawn");
        m.insert("theme.forest", "Forest");
        m.insert("hotkeys.title", "Hotkeys");
        m.insert("hotkeys.listen", "Listen");
        m.insert("hotkeys.cancel", "Cancel");
        m.insert("hotkeys.tts", "Voice");
        m.insert("hotkeys.dictation", "Dictation");
        m.insert("hotkeys.pause", "Pause");
        m.insert("about.title", "About");
        m.insert("about.version", "Version");
        m.insert("about.desc", "Voice assistant for Windows. Recognizes commands locally; unusual ones go to a configured LLM.");
        m.insert("cmd.unknown", "Unknown");
        m.insert("cmd.ok", "Done");
        m
    })
}

pub fn dict(lang: Lang) -> &'static HashMap<&'static str, &'static str> {
    match lang { Lang::Ru => ru(), Lang::En => en() }
}

pub fn t(lang: Lang, key: &str) -> String {
    dict(lang).get(key).copied().unwrap_or(key).to_string()
}
