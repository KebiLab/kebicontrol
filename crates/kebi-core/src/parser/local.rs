//! Local rule-based parser. Zero network. Made by KebiLab

use crate::command::{
    Command, Confidence, InputOp, MediaOp, ParseSource, ParsedCommand, PowerOp, RemindKind,
    ScreenshotMode, VolumeOp, WebOp, WindowOp,
};
use crate::i18n::Lang;
use crate::parser::ParserContext;
use once_cell::sync::Lazy;
use regex::Regex;

struct Pat {
    re: Regex,
    build: fn(&regex::Captures) -> Option<Command>,
}

fn re(s: &str) -> Regex {
    Regex::new(&format!("(?i)^{s}\\s*[\\.,!\\?]*\\s*$")).expect("valid regex")
}

static PATTERNS_RU: Lazy<Vec<Pat>> = Lazy::new(|| {
    vec![
        // Power
        Pat { re: re(r"выключ(и|и компьютер|и пк)"), build: |_| Some(Command::Power { op: PowerOp::Shutdown }) },
        Pat { re: re(r"(перезагрузи|рестарт)"), build: |_| Some(Command::Power { op: PowerOp::Restart }) },
        Pat { re: re(r"(в сон|спящ(ий|ий режим))"), build: |_| Some(Command::Power { op: PowerOp::Sleep }) },
        Pat { re: re(r"(гибернат|глубокий сон)"), build: |_| Some(Command::Power { op: PowerOp::Hibernate }) },
        Pat { re: re(r"(заблокируй|блокировка)"), build: |_| Some(Command::Power { op: PowerOp::Lock }) },
        Pat { re: re(r"(выйди из уч(ётной записи|ётки)|смени пользователя)"), build: |_| Some(Command::Power { op: PowerOp::SignOut }) },
        // Volume
        Pat { re: re(r"(громче|увелич(ь|ь) громкость)"), build: |_| Some(Command::Volume { op: VolumeOp::Up, value: None }) },
        Pat { re: re(r"(тише|уменьш(и|ь) громкость)"), build: |_| Some(Command::Volume { op: VolumeOp::Down, value: None }) },
        Pat { re: re(r"(выключи звук|без звука|отключи звук|мут)"), build: |_| Some(Command::Volume { op: VolumeOp::Mute, value: None }) },
        Pat { re: re(r"(включи звук|верни звук)"), build: |_| Some(Command::Volume { op: VolumeOp::Unmute, value: None }) },
        Pat { re: re(r"громкость\s+на\s+(\d{1,3})"), build: |c| c.get(1).and_then(|m| m.as_str().parse::<u8>().ok()).map(|v| Command::Volume { op: VolumeOp::Set, value: Some(v.min(100)) }) },
        // Brightness
        Pat { re: re(r"яркость\s+на\s+(\d{1,3})"), build: |c| c.get(1).and_then(|m| m.as_str().parse::<u8>().ok()).map(|v| Command::Brightness { value: v.min(100) }) },
        // Media
        Pat { re: re(r"(пауза|поставь на паузу)"), build: |_| Some(Command::Media { op: MediaOp::Pause }) },
        Pat { re: re(r"(играй|воспроизведи|продолжи)"), build: |_| Some(Command::Media { op: MediaOp::Play }) },
        Pat { re: re(r"(следующ(ий|ая)|дальше)"), build: |_| Some(Command::Media { op: MediaOp::Next }) },
        Pat { re: re(r"(предыдущ(ий|ая)|назад)"), build: |_| Some(Command::Media { op: MediaOp::Previous }) },
        Pat { re: re(r"стоп"), build: |_| Some(Command::Media { op: MediaOp::Stop }) },
        // Screenshot
        Pat { re: re(r"(скриншот|снимок экрана)"), build: |_| Some(Command::Screenshot { mode: ScreenshotMode::Full }) },
        Pat { re: re(r"скриншот окна"), build: |_| Some(Command::Screenshot { mode: ScreenshotMode::Window }) },
        Pat { re: re(r"скриншот выделения"), build: |_| Some(Command::Screenshot { mode: ScreenshotMode::Selection }) },
        // Window
        Pat { re: re(r"(сверни( всё)?|свернуть( всё)?)"), build: |_| Some(Command::Window { op: WindowOp::ShowDesktop, target: None }) },
        Pat { re: re(r"(разверни( всё)?|развернуть( всё)?)"), build: |_| Some(Command::Window { op: WindowOp::Maximize, target: None }) },
        Pat { re: re(r"(восстанови|обычный размер)"), build: |_| Some(Command::Window { op: WindowOp::Restore, target: None }) },
        Pat { re: re(r"(закрой окно|закрой это)"), build: |_| Some(Command::Window { op: WindowOp::Close, target: None }) },
        Pat { re: re(r"окно влево"), build: |_| Some(Command::Window { op: WindowOp::SnapLeft, target: None }) },
        Pat { re: re(r"окно вправо"), build: |_| Some(Command::Window { op: WindowOp::SnapRight, target: None }) },
        Pat { re: re(r"переключи( окно|ся)?"), build: |_| Some(Command::Window { op: WindowOp::AltTab, target: None }) },
        // Dictation
        Pat { re: re(r"(включи|запусти) режим диктовки"), build: |_| Some(Command::Dictation { on: true }) },
        Pat { re: re(r"(выключи|стоп) диктовк(а|у|и)"), build: |_| Some(Command::Dictation { on: false }) },
        // TTS / help
        Pat { re: re(r"(хелп|помощь|что ты умеешь)"), build: |_| Some(Command::Help) },
        Pat { re: re(r"(выключи голос|включи голос|тишина|продолжай)"), build: |_| Some(Command::ToggleTts) },
        // TTS arbitrary phrase: "скажи <text>"
        Pat { re: re(r"скажи\s+(.+)"), build: |c| c.get(1).map(|m| Command::Say { text: m.as_str().trim().to_string() }) },
        // Search / open URL
        Pat { re: re(r"(найди|поищи)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Web { op: WebOp::Search, query: m.as_str().trim().to_string() }) },
        Pat { re: re(r"(открой сайт|открой ссылку)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Web { op: WebOp::Open, query: m.as_str().trim().to_string() }) },
        // Close app
        Pat { re: re(r"закрой\s+(.+)"), build: |c| c.get(1).map(|m| Command::Close { name: m.as_str().trim().to_string(), force: false }) },
        // Focus app
        Pat { re: re(r"(переключись на|перейди к|к окну)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Focus { name: m.as_str().trim().to_string() }) },
        // Input: type / press / click
        Pat { re: re(r"(набери|введи текст)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Input { op: InputOp::Type, text: Some(m.as_str().trim().to_string()) }) },
        Pat { re: re(r"нажми\s+(.+)"), build: |c| c.get(1).map(|m| Command::Input { op: InputOp::Press, text: Some(m.as_str().trim().to_string()) }) },
        Pat { re: re(r"(кликни|щёлкни)( левой| правой| дважды)?"), build: |c| {
            let s = c.get(0).map(|m| m.as_str().to_lowercase()).unwrap_or_default();
            if s.contains("правой") { Some(Command::Input { op: InputOp::RightClick, text: None }) }
            else if s.contains("дважды") { Some(Command::Input { op: InputOp::DoubleClick, text: None }) }
            else { Some(Command::Input { op: InputOp::Click, text: None }) }
        } },
        // Reminders
        Pat { re: re(r"таймер\s+на\s+(.+)"), build: |c| c.get(1).map(|m| Command::Remind { kind: RemindKind::Timer, value: m.as_str().trim().to_string(), text: None }) },
        Pat { re: re(r"напомни\s+в\s+(\d{1,2}[:\.]\d{2})\s+(.+)"), build: |c| {
            let t = c.get(1)?.as_str().replace('.', ":");
            let txt = c.get(2)?.as_str().trim().to_string();
            Some(Command::Remind { kind: RemindKind::At, value: t, text: Some(txt) })
        } },
        // Open app (catch-all)
        Pat { re: re(r"(открой|запусти)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Open { target: m.as_str().trim().to_string() }) },
    ]
});

static PATTERNS_EN: Lazy<Vec<Pat>> = Lazy::new(|| {
    vec![
        Pat { re: re(r"(shutdown|turn off|power off)"), build: |_| Some(Command::Power { op: PowerOp::Shutdown }) },
        Pat { re: re(r"restart"), build: |_| Some(Command::Power { op: PowerOp::Restart }) },
        Pat { re: re(r"sleep"), build: |_| Some(Command::Power { op: PowerOp::Sleep }) },
        Pat { re: re(r"(hibernate|deep sleep)"), build: |_| Some(Command::Power { op: PowerOp::Hibernate }) },
        Pat { re: re(r"lock"), build: |_| Some(Command::Power { op: PowerOp::Lock }) },
        Pat { re: re(r"sign out"), build: |_| Some(Command::Power { op: PowerOp::SignOut }) },
        Pat { re: re(r"(volume up|louder)"), build: |_| Some(Command::Volume { op: VolumeOp::Up, value: None }) },
        Pat { re: re(r"(volume down|quieter)"), build: |_| Some(Command::Volume { op: VolumeOp::Down, value: None }) },
        Pat { re: re(r"mute"), build: |_| Some(Command::Volume { op: VolumeOp::Mute, value: None }) },
        Pat { re: re(r"unmute"), build: |_| Some(Command::Volume { op: VolumeOp::Unmute, value: None }) },
        Pat { re: re(r"volume\s+(\d{1,3})"), build: |c| c.get(1).and_then(|m| m.as_str().parse::<u8>().ok()).map(|v| Command::Volume { op: VolumeOp::Set, value: Some(v.min(100)) }) },
        Pat { re: re(r"brightness\s+(\d{1,3})"), build: |c| c.get(1).and_then(|m| m.as_str().parse::<u8>().ok()).map(|v| Command::Brightness { value: v.min(100) }) },
        Pat { re: re(r"pause"), build: |_| Some(Command::Media { op: MediaOp::Pause }) },
        Pat { re: re(r"(play|resume)"), build: |_| Some(Command::Media { op: MediaOp::Play }) },
        Pat { re: re(r"next"), build: |_| Some(Command::Media { op: MediaOp::Next }) },
        Pat { re: re(r"previous"), build: |_| Some(Command::Media { op: MediaOp::Previous }) },
        Pat { re: re(r"stop"), build: |_| Some(Command::Media { op: MediaOp::Stop }) },
        Pat { re: re(r"(screenshot|screen shot)"), build: |_| Some(Command::Screenshot { mode: ScreenshotMode::Full }) },
        Pat { re: re(r"(minimize|show desktop)"), build: |_| Some(Command::Window { op: WindowOp::ShowDesktop, target: None }) },
        Pat { re: re(r"maximize"), build: |_| Some(Command::Window { op: WindowOp::Maximize, target: None }) },
        Pat { re: re(r"restore"), build: |_| Some(Command::Window { op: WindowOp::Restore, target: None }) },
        Pat { re: re(r"close window"), build: |_| Some(Command::Window { op: WindowOp::Close, target: None }) },
        Pat { re: re(r"snap left"), build: |_| Some(Command::Window { op: WindowOp::SnapLeft, target: None }) },
        Pat { re: re(r"snap right"), build: |_| Some(Command::Window { op: WindowOp::SnapRight, target: None }) },
        Pat { re: re(r"(switch window|alt tab)"), build: |_| Some(Command::Window { op: WindowOp::AltTab, target: None }) },
        Pat { re: re(r"(start dictation|enable dictation)"), build: |_| Some(Command::Dictation { on: true }) },
        Pat { re: re(r"(stop dictation|disable dictation)"), build: |_| Some(Command::Dictation { on: false }) },
        Pat { re: re(r"(help|what can you do)"), build: |_| Some(Command::Help) },
        Pat { re: re(r"(be quiet|enable voice|toggle voice)"), build: |_| Some(Command::ToggleTts) },
        Pat { re: re(r"say\s+(.+)"), build: |c| c.get(1).map(|m| Command::Say { text: m.as_str().trim().to_string() }) },
        Pat { re: re(r"(search|google)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Web { op: WebOp::Search, query: m.as_str().trim().to_string() }) },
        Pat { re: re(r"(open url|open site)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Web { op: WebOp::Open, query: m.as_str().trim().to_string() }) },
        Pat { re: re(r"close\s+(.+)"), build: |c| c.get(1).map(|m| Command::Close { name: m.as_str().trim().to_string(), force: false }) },
        Pat { re: re(r"(switch to|focus)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Focus { name: m.as_str().trim().to_string() }) },
        Pat { re: re(r"(type|write)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Input { op: InputOp::Type, text: Some(m.as_str().trim().to_string()) }) },
        Pat { re: re(r"press\s+(.+)"), build: |c| c.get(1).map(|m| Command::Input { op: InputOp::Press, text: Some(m.as_str().trim().to_string()) }) },
        Pat { re: re(r"click(\s+(right|double))?"), build: |c| {
            let s = c.get(0).map(|m| m.as_str().to_lowercase()).unwrap_or_default();
            if s.contains("right") { Some(Command::Input { op: InputOp::RightClick, text: None }) }
            else if s.contains("double") { Some(Command::Input { op: InputOp::DoubleClick, text: None }) }
            else { Some(Command::Input { op: InputOp::Click, text: None }) }
        } },
        Pat { re: re(r"timer\s+(.+)"), build: |c| c.get(1).map(|m| Command::Remind { kind: RemindKind::Timer, value: m.as_str().trim().to_string(), text: None }) },
        Pat { re: re(r"remind\s+at\s+(\d{1,2}[:\.]\d{2})\s+(.+)"), build: |c| {
            let t = c.get(1)?.as_str().replace('.', ":");
            let txt = c.get(2)?.as_str().trim().to_string();
            Some(Command::Remind { kind: RemindKind::At, value: t, text: Some(txt) })
        } },
        Pat { re: re(r"(open|launch)\s+(.+)"), build: |c| c.get(2).map(|m| Command::Open { target: m.as_str().trim().to_string() }) },
    ]
});

pub fn try_match(ctx: &ParserContext, text: &str) -> Option<ParsedCommand> {
    let lang = Lang::from_code(ctx.language);
    let patterns = match lang {
        Lang::Ru => &*PATTERNS_RU,
        Lang::En => &*PATTERNS_EN,
    };

    for pat in patterns {
        if let Some(caps) = pat.re.captures(text.trim()) {
            if let Some(cmd) = (pat.build)(&caps) {
                return Some(ParsedCommand {
                    command: cmd,
                    confidence: Confidence::HIGH,
                    source: ParseSource::LocalRule,
                });
            }
        }
    }

    // Aliases like "ютуб" -> "https://youtube.com"
    let resolved = ctx.profile.aliases.resolve(text);
    if resolved != text && (resolved.starts_with("http://") || resolved.starts_with("https://")) {
        return Some(ParsedCommand {
            command: Command::Web { op: WebOp::Open, query: resolved },
            confidence: Confidence::HIGH,
            source: ParseSource::LocalRule,
        });
    }

    // Custom phrase overrides
    let key = text.trim().to_lowercase();
    if let Some(kind) = ctx.profile.custom_phrases.get(&key) {
        let cmd = match kind.as_str() {
            "cancel" => Command::Help,
            "stop_tts" => Command::ToggleTts,
            other => Command::Unknown { reason: format!("custom:{other}") },
        };
        return Some(ParsedCommand {
            command: cmd,
            confidence: Confidence::HIGH,
            source: ParseSource::LocalRule,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::Profile;

    #[test]
    fn ru_volume_set() {
        let p = Profile::default();
        let ctx = ParserContext { profile: &p, wake_word: "кеби", language: "ru" };
        let out = try_match(&ctx, "громкость на 40");
        assert!(matches!(out, Some(ref pc) if matches!(pc.command, Command::Volume { op: VolumeOp::Set, value: Some(40) })));
    }

    #[test]
    fn ru_open_youtube_alias() {
        let p = Profile::default();
        let ctx = ParserContext { profile: &p, wake_word: "кеби", language: "ru" };
        let out = try_match(&ctx, "ютуб");
        assert!(matches!(out, Some(ref pc) if matches!(pc.command, Command::Web { op: WebOp::Open, .. })));
    }

    #[test]
    fn en_shutdown() {
        let p = Profile::default();
        let ctx = ParserContext { profile: &p, wake_word: "kebi", language: "en" };
        let out = try_match(&ctx, "shutdown");
        assert!(matches!(out, Some(ref pc) if matches!(pc.command, Command::Power { op: PowerOp::Shutdown })));
    }
}
