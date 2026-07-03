//! Keyboard / mouse input actions. Made by KebiLab

use crate::command::InputOp;
use crate::error::{KebiError, Result};
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings, Mouse};

pub async fn apply(op: InputOp, text: Option<&str>) -> Result<Option<String>> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| KebiError::Action(format!("enigo: {e}")))?;
    let msg = match op {
        InputOp::Type => {
            let t = text.unwrap_or("");
            // Set clipboard, then Ctrl+V to paste (fast and works in most apps).
            set_clipboard(t)?;
            enigo.key(Key::Control, Click).ok();
            enigo.key(Key::Unicode('v'), Click).ok();
            "Ввёл текст"
        }
        InputOp::Press => {
            let key = parse_key(text.unwrap_or(""));
            enigo.key(key, Click).ok();
            "Нажал клавишу"
        }
        InputOp::Click => {
            enigo.button(enigo::Button::Left, Click).ok();
            "Клик"
        }
        InputOp::RightClick => {
            enigo.button(enigo::Button::Right, Click).ok();
            "Правый клик"
        }
        InputOp::DoubleClick => {
            enigo.button(enigo::Button::Left, Click).ok();
            enigo.button(enigo::Button::Left, Click).ok();
            "Двойной клик"
        }
        InputOp::Scroll => {
            let amount: i32 = text
                .and_then(|s| s.parse().ok())
                .unwrap_or(3);
            enigo.scroll(amount, enigo::Axis::Vertical).ok();
            "Прокрутил"
        }
    };
    Ok(Some(msg.into()))
}

fn parse_key(s: &str) -> Key {
    let k = s.trim().to_lowercase();
    match k.as_str() {
        "enter" | "ввод" | "энтер" => Key::Return,
        "esc" | "escape" | "эскейп" => Key::Escape,
        "tab" | "таб" => Key::Tab,
        "space" | "пробел" => Key::Space,
        "backspace" | "бэкспейс" => Key::Backspace,
        "delete" | "дел" | "удалить" => Key::Delete,
        "up" | "вверх" => Key::UpArrow,
        "down" | "вниз" => Key::DownArrow,
        "left" | "влево" => Key::LeftArrow,
        "right" | "вправо" => Key::RightArrow,
        "home" => Key::Home,
        "end" => Key::End,
        "pageup" => Key::PageUp,
        "pagedown" => Key::PageDown,
        "ctrl" | "control" => Key::Control,
        "alt" => Key::Alt,
        "shift" => Key::Shift,
        "win" | "windows" => Key::Meta,
        "f1" => Key::F1,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "f10" => Key::F10,
        "f11" => Key::F11,
        "f12" => Key::F12,
        s if s.chars().count() == 1 => Key::Unicode(s.chars().next().unwrap()),
        _ => Key::Space,
    }
}

#[cfg(windows)]
fn set_clipboard(text: &str) -> Result<()> {
    use windows::Win32::System::DataExchange::{OpenClipboard, SetClipboardData, CloseClipboard, EmptyClipboard};
    use windows::Win32::Foundation::HGLOBAL;
    use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
    unsafe {
        let _ = OpenClipboard(None);
        let _ = EmptyClipboard();
        let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let bytes = wide.len() * std::mem::size_of::<u16>();
        let h = GlobalAlloc(GMEM_MOVEABLE, bytes);
        if h.is_err() {
            let _ = CloseClipboard();
            return Err(KebiError::Action("clipboard alloc failed".into()));
        }
        let h = HGLOBAL(h.unwrap());
        let ptr = GlobalLock(h);
        std::ptr::copy_nonoverlapping(wide.as_ptr() as *const _, ptr as *mut _, wide.len());
        let _ = GlobalUnlock(h);
        let _ = SetClipboardData(13 /* CF_UNICODETEXT */, Some(windows::Win32::Foundation::HANDLE(h.0)));
        let _ = CloseClipboard();
    }
    Ok(())
}

#[cfg(not(windows))]
fn set_clipboard(_text: &str) -> Result<()> { Ok(()) }
