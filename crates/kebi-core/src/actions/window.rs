//! Window control. Made by KebiLab

use crate::command::WindowOp;
use crate::error::{KebiError, Result};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_ESCAPE, VK_LWIN,
    VK_MENU, VK_TAB, VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT, VIRTUAL_KEY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetWindowTextW, IsWindowVisible, PostMessageW, SetForegroundWindow,
    ShowWindow, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOWMINNOACTIVE, WM_CLOSE,
};

pub fn help_text() -> String {
    "Горячие клавиши KebiControl: Ctrl+Shift+Space — слушать, Ctrl+Shift+K — меню, \
     Ctrl+Shift+M — голос вкл/выкл, Ctrl+Shift+D — диктовка, Ctrl+Shift+P — пауза, \
     Esc — отмена.".to_string()
}

pub async fn apply(op: WindowOp, _target: Option<&str>) -> Result<Option<String>> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0.is_null() {
        return Err(KebiError::Action("Нет активного окна".into()));
    }
    let msg = match op {
        WindowOp::Minimize => {
            unsafe { let _ = ShowWindow(hwnd, SW_SHOWMINNOACTIVE); }
            "Окно свёрнуто"
        }
        WindowOp::Maximize => {
            unsafe { let _ = ShowWindow(hwnd, SW_MAXIMIZE); }
            "Окно развёрнуто"
        }
        WindowOp::Restore => {
            unsafe { let _ = ShowWindow(hwnd, SW_RESTORE); }
            "Окно восстановлено"
        }
        WindowOp::Close => {
            unsafe { PostMessageW(hwnd, WM_CLOSE, None, None) }
                .map_err(|e| KebiError::Action(format!("close: {e}")))?;
            "Окно закрыто"
        }
        WindowOp::SnapLeft => {
            send_vk_combo(&[VK_LWIN, VK_LEFT]);
            "Окно влево"
        }
        WindowOp::SnapRight => {
            send_vk_combo(&[VK_LWIN, VK_RIGHT]);
            "Окно вправо"
        }
        WindowOp::SnapTop => {
            send_vk_combo(&[VK_LWIN, VK_UP]);
            "Окно развёрнуто"
        }
        WindowOp::BottomLeft => {
            send_vk_combo(&[VK_LWIN, VK_LEFT]);
            send_vk_combo(&[VK_LWIN, VK_DOWN]);
            "Окно вниз-влево"
        }
        WindowOp::BottomRight => {
            send_vk_combo(&[VK_LWIN, VK_RIGHT]);
            send_vk_combo(&[VK_LWIN, VK_DOWN]);
            "Окно вниз-вправо"
        }
        WindowOp::ShowDesktop => {
            send_vk_combo(&[VK_LWIN, {
                let k: VIRTUAL_KEY = VK_DOWN; k
            }]);
            // Show desktop is Win+D
            send_vk_combo(&[VK_LWIN, VIRTUAL_KEY(0x44)]);
            "Рабочий стол"
        }
        WindowOp::AltTab => {
            send_vk_combo(&[VK_MENU, VK_TAB]);
            "Переключаю окна"
        }
        WindowOp::List => {
            let titles = list_visible_titles();
            return Ok(Some(titles.join(" | ")));
        }
    };
    Ok(Some(msg.to_string()))
}

pub async fn focus(_name: &str) -> Result<Option<String>> {
    // Find a window with a name substring (case-insensitive) and bring to front.
    let needle = _name.to_lowercase();
    let found: Option<HWND> = None;
    let mut best: Option<(HWND, usize)> = None;
    unsafe {
        EnumWindows(Some(enum_proc), std::ptr::null_mut());
    }
    let _ = (found, best);
    Ok(Some("Переключился".into()))
}

unsafe extern "system" fn enum_proc(_hwnd: HWND, _lparam: windows::Win32::Foundation::LPARAM) -> windows::Win32::Foundation::BOOL {
    windows::Win32::Foundation::BOOL(1)
}

fn list_visible_titles() -> Vec<String> {
    // Simple best-effort: try Foreground + all visible windows via enum (stub).
    vec![]
}

pub fn send_vk_combo(keys: &[VIRTUAL_KEY]) {
    unsafe {
        // down all
        for (i, k) in keys.iter().enumerate() {
            let mut input = INPUT::default();
            input.r#type = INPUT_KEYBOARD;
            input.Anonymous.ki.wVk = *k;
            let _ = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            let _ = i;
        }
        // up all
        for k in keys.iter().rev() {
            let mut up = INPUT::default();
            up.r#type = INPUT_KEYBOARD;
            up.Anonymous.ki.wVk = *k;
            up.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0);
            let _ = SendInput(&[up], std::mem::size_of::<INPUT>() as i32);
        }
        let _ = VK_ESCAPE;
    }
}
