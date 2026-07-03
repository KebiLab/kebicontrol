//! Media keys. Made by KebiLab

use crate::command::MediaOp;
use crate::error::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_MEDIA_NEXT_TRACK,
    VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK, VK_MEDIA_STOP, VIRTUAL_KEY,
};

pub async fn apply(op: MediaOp) -> Result<Option<String>> {
    let vk: VIRTUAL_KEY = match op {
        MediaOp::Play | MediaOp::Toggle => VK_MEDIA_PLAY_PAUSE,
        MediaOp::Pause => VK_MEDIA_PLAY_PAUSE,
        MediaOp::Next => VK_MEDIA_NEXT_TRACK,
        MediaOp::Previous => VK_MEDIA_PREV_TRACK,
        MediaOp::Stop => VK_MEDIA_STOP,
        MediaOp::VolUp | MediaOp::VolDown | MediaOp::Mute => return Ok(None),
    };
    unsafe {
        let mut input = INPUT::default();
        input.r#type = INPUT_KEYBOARD;
        input.Anonymous.ki.wVk = vk;
        let _ = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        let mut up = INPUT::default();
        up.r#type = INPUT_KEYBOARD;
        up.Anonymous.ki.wVk = vk;
        up.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0);
        let _ = SendInput(&[up], std::mem::size_of::<INPUT>() as i32);
    }
    Ok(Some("Медиа".into()))
}
