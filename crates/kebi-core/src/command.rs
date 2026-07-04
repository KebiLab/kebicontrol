//! Command domain types. Made by KebiLab

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A normalized command produced by the parser and consumed by actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Command {
    /// Echo / speak a phrase.
    Say { text: String },
    /// Open an application or URL.
    Open { target: String },
    /// Close a process by name.
    Close { name: String, force: bool },
    /// Switch to a running application.
    Focus { name: String },
    /// Volume control.
    Volume { op: VolumeOp, value: Option<u8> },
    /// Brightness control.
    Brightness { value: u8 },
    /// Window control.
    Window { op: WindowOp, target: Option<String> },
    /// Mouse / keyboard input.
    Input { op: InputOp, text: Option<String> },
    /// Media keys.
    Media { op: MediaOp },
    /// Screenshot.
    Screenshot { mode: ScreenshotMode },
    /// Web search / open URL.
    Web { op: WebOp, query: String },
    /// Timers / reminders.
    Remind { kind: RemindKind, value: String, text: Option<String> },
    /// System power.
    Power { op: PowerOp },
    /// Toggle TTS.
    ToggleTts,
    /// List hotkeys verbally.
    Help,
    /// Enter / leave dictation mode.
    Dictation { on: bool },
    /// General chat reply (LLM answered in natural language).
    Chat { text: String },
    /// Unknown / not understood.
    Unknown { reason: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VolumeOp {
    Up,
    Down,
    Set,
    Mute,
    Unmute,
    Toggle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WindowOp {
    Minimize,
    Maximize,
    Restore,
    Close,
    SnapLeft,
    SnapRight,
    SnapTop,
    BottomLeft,
    BottomRight,
    ShowDesktop,
    AltTab,
    List,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InputOp {
    Type,
    Press,
    Click,
    RightClick,
    DoubleClick,
    Scroll,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MediaOp {
    Play,
    Pause,
    Toggle,
    Next,
    Previous,
    Stop,
    VolUp,
    VolDown,
    Mute,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ScreenshotMode {
    Full,
    Window,
    Selection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WebOp {
    Search,
    Open,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RemindKind {
    Timer,
    At,
    Stopwatch,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PowerOp {
    Shutdown,
    Restart,
    Sleep,
    Hibernate,
    Lock,
    SignOut,
}

impl Command {
    pub fn kind(&self) -> &'static str {
        match self {
            Command::Say { .. } => "say",
            Command::Open { .. } => "open",
            Command::Close { .. } => "close",
            Command::Focus { .. } => "focus",
            Command::Volume { .. } => "volume",
            Command::Brightness { .. } => "brightness",
            Command::Window { .. } => "window",
            Command::Input { .. } => "input",
            Command::Media { .. } => "media",
            Command::Screenshot { .. } => "screenshot",
            Command::Web { .. } => "web",
            Command::Remind { .. } => "remind",
            Command::Power { .. } => "power",
            Command::ToggleTts => "toggle_tts",
            Command::Help => "help",
            Command::Dictation { .. } => "dictation",
            Command::Chat { .. } => "chat",
            Command::Unknown { .. } => "unknown",
        }
    }
}

/// Parsing confidence 0.0..=1.0
#[derive(Debug, Clone, Copy)]
pub struct Confidence(pub f32);

impl Confidence {
    pub const HIGH: Self = Self(0.95);
    pub const MEDIUM: Self = Self(0.7);
    pub const LOW: Self = Self(0.4);
}

/// Parser output: a command + confidence + source tag.
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub command: Command,
    pub confidence: Confidence,
    pub source: ParseSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseSource {
    LocalRule,
    Llm,
    Manual,
}

/// Application alias map: "ютуб" -> "https://youtube.com"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Aliases(pub HashMap<String, String>);

impl Aliases {
    pub fn resolve(&self, raw: &str) -> String {
        let key = raw.trim().to_lowercase();
        self.0.get(&key).cloned().unwrap_or_else(|| raw.to_string())
    }
}
