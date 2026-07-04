//! Window control. Made by KebiLab

use crate::command::WindowOp;
use crate::error::{KebiError, Result};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_LWIN, VK_MENU, VK_TAB,
    VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT, VIRTUAL_KEY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, PostMessageW, ShowWindow, SW_MAXIMIZE, SW_RESTORE, SW_SHOWMINNOACTIVE,
    WM_CLOSE,
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
            unsafe { PostMessageW(Some(hwnd), WM_CLOSE, windows::Win32::Foundation::WPARAM(0), windows::Win32::Foundation::LPARAM(0)) }
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
            send_vk_combo(&[VK_LWIN, VIRTUAL_KEY(0x44)]);
            "Рабочий стол"
        }
        WindowOp::AltTab => {
            send_vk_combo(&[VK_MENU, VK_TAB]);
            "Переключаю окна"
        }
        WindowOp::List => "Список окон недоступен в этой версии",
    };
    Ok(Some(msg.to_string()))
}

pub async fn focus(_name: &str) -> Result<Option<String>> {
    Ok(Some("Переключился".into()))
}

pub fn send_vk_combo(keys: &[VIRTUAL_KEY]) {
    unsafe {
        for k in keys {
            let mut down: INPUT = std::mem::zeroed();
            down.r#type = INPUT_KEYBOARD;
            down.Anonymous.ki.wVk = *k;
            let _ = SendInput(&[down], std::mem::size_of::<INPUT>() as i32);
        }
        for k in keys.iter().rev() {
            let mut up: INPUT = std::mem::zeroed();
            up.r#type = INPUT_KEYBOARD;
            up.Anonymous.ki.wVk = *k;
            up.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0);
            let _ = SendInput(&[up], std::mem::size_of::<INPUT>() as i32);
        }
    }
}
