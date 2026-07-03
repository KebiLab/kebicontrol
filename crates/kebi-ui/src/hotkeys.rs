//! Global hotkey registration. Made by KebiLab

use anyhow::Result;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::GlobalHotKeyManager;

pub fn parse_hotkey(s: &str) -> Option<HotKey> {
    let mut mods = Modifiers::empty();
    let mut key: Option<Code> = None;
    for tok in s.split('+') {
        let t = tok.trim();
        match t.to_lowercase().as_str() {
            "ctrl" | "control" => mods |= Modifiers::CONTROL,
            "shift" => mods |= Modifiers::SHIFT,
            "alt" => mods |= Modifiers::ALT,
            "win" | "meta" | "super" => mods |= Modifiers::META,
            "space" => key = Some(Code::Space),
            "esc" | "escape" => key = Some(Code::Escape),
            "k" => key = Some(Code::KeyK),
            "m" => key = Some(Code::KeyM),
            "d" => key = Some(Code::KeyD),
            "p" => key = Some(Code::KeyP),
            "k" => key = Some(Code::KeyK),
            other => {
                if other.len() == 1 {
                    let c = other.chars().next().unwrap().to_ascii_uppercase();
                    key = Some(match c {
                        'A' => Code::KeyA, 'B' => Code::KeyB, 'C' => Code::KeyC, 'D' => Code::KeyD,
                        'E' => Code::KeyE, 'F' => Code::KeyF, 'G' => Code::KeyG, 'H' => Code::KeyH,
                        'I' => Code::KeyI, 'J' => Code::KeyJ, 'K' => Code::KeyK, 'L' => Code::KeyL,
                        'M' => Code::KeyM, 'N' => Code::KeyN, 'O' => Code::KeyO, 'P' => Code::KeyP,
                        'Q' => Code::KeyQ, 'R' => Code::KeyR, 'S' => Code::KeyS, 'T' => Code::KeyT,
                        'U' => Code::KeyU, 'V' => Code::KeyV, 'W' => Code::KeyW, 'X' => Code::KeyX,
                        'Y' => Code::KeyY, 'Z' => Code::KeyZ,
                        _ => return None,
                    });
                }
            }
        }
    }
    Some(HotKey::new(mods, key?))
}

pub fn build_manager() -> Result<GlobalHotKeyManager> {
    Ok(GlobalHotKeyManager::new()?)
}
