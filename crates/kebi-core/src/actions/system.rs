//! System actions. Made by KebiLab

use crate::command::{PowerOp, RemindKind, ScreenshotMode, VolumeOp};
use crate::error::{KebiError, Result};
use std::process::Command as StdCommand;
use std::sync::atomic::{AtomicBool, Ordering};

static TTS_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn tts_enabled() -> bool { TTS_ENABLED.load(Ordering::Relaxed) }
pub fn toggle_tts() -> Result<bool> {
    let prev = TTS_ENABLED.load(Ordering::Relaxed);
    TTS_ENABLED.store(!prev, Ordering::Relaxed);
    Ok(!prev)
}

pub async fn speak(_text: &str) -> Result<()> {
    tracing::info!(text = _text, "tts: speak");
    Ok(())
}

pub async fn power(op: PowerOp) -> Result<Option<String>> {
    let (verb, exe, args): (&str, &str, Vec<&str>) = match op {
        PowerOp::Shutdown => ("Выключаю компьютер", "shutdown", vec!["/s", "/t", "0"]),
        PowerOp::Restart => ("Перезагружаю", "shutdown", vec!["/r", "/t", "0"]),
        PowerOp::Sleep => ("Усыпляю", "rundll32", vec!["powrprof.dll,SetSuspendState", "0", "1", "0"]),
        PowerOp::Hibernate => ("Гибернация", "shutdown", vec!["/h"]),
        PowerOp::Lock => ("Блокирую", "rundll32", vec!["user32.dll,LockWorkStation"]),
        PowerOp::SignOut => ("Выхожу", "shutdown", vec!["/l"]),
    };
    StdCommand::new(exe)
        .args(&args)
        .spawn()
        .map_err(|e| KebiError::Action(format!("{verb}: {e}")))?;
    Ok(Some(verb.into()))
}

pub async fn volume(op: VolumeOp, value: Option<u8>) -> Result<Option<String>> {
    use windows::Win32::Media::Audio::{
        eConsole, Endpoints::IAudioEndpointVolume, EDataFlow, ERole, IMMDeviceEnumerator, MMDeviceEnumerator,
    };
    use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};
    use windows::core::Interface;

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
                .map_err(|e| KebiError::Action(format!("enumerator: {e}")))?;
        let device = enumerator
            .GetDefaultAudioEndpoint(EDataFlow(eConsole.0), ERole(0))
            .map_err(|e| KebiError::Action(format!("endpoint: {e}")))?;
        let endpoint: IAudioEndpointVolume = device
            .cast()
            .map_err(|e| KebiError::Action(format!("cast: {e}")))?;
        let msg: &str = match op {
            VolumeOp::Up => { let _ = endpoint.VolumeStepUp(std::ptr::null()); "Громче" }
            VolumeOp::Down => { let _ = endpoint.VolumeStepDown(std::ptr::null()); "Тише" }
            VolumeOp::Mute => { let _ = endpoint.SetMute(true, std::ptr::null()); "Звук выключен" }
            VolumeOp::Unmute => { let _ = endpoint.SetMute(false, std::ptr::null()); "Звук включён" }
            VolumeOp::Toggle => {
                let cur = endpoint.GetMute().map(|b| b.as_bool()).unwrap_or(false);
                let _ = endpoint.SetMute(!cur, std::ptr::null());
                if cur { "Звук включён" } else { "Звук выключен" }
            }
            VolumeOp::Set => {
                if let Some(v) = value {
                    let scalar = v as f32 / 100.0;
                    let _ = endpoint.SetMasterVolumeLevelScalar(scalar, std::ptr::null());
                    "Громкость установлена"
                } else {
                    "Не указано значение"
                }
            }
        };
        Ok(Some(msg.into()))
    }
}

pub async fn brightness(value: u8) -> Result<Option<String>> {
    let v = value.min(100);
    let script = format!(
        "(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1,{v})"
    );
    StdCommand::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()
        .map_err(|e| KebiError::Action(format!("brightness: {e}")))?;
    Ok(Some(format!("Яркость {v}%")))
}

pub async fn screenshot(mode: ScreenshotMode) -> Result<Option<String>> {
    let _ = mode;
    input_send_print_screen()?;
    Ok(Some("Скриншот сделан".into()))
}

fn input_send_print_screen() -> Result<()> {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_SNAPSHOT,
    };
    unsafe {
        let mut down: INPUT = std::mem::zeroed();
        down.r#type = INPUT_KEYBOARD;
        down.Anonymous.ki.wVk = VK_SNAPSHOT;
        SendInput(&[down], std::mem::size_of::<INPUT>() as i32);
        let mut up: INPUT = std::mem::zeroed();
        up.r#type = INPUT_KEYBOARD;
        up.Anonymous.ki.wVk = VK_SNAPSHOT;
        up.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0);
        SendInput(&[up], std::mem::size_of::<INPUT>() as i32);
    }
    Ok(())
}

pub async fn remind(kind: RemindKind, value: &str, text: Option<&str>) -> Result<Option<String>> {
    let msg = text.unwrap_or("Время вышло").to_string();
    match kind {
        RemindKind::Timer => {
            let (val, unit) = parse_duration(value);
            if val <= 0 {
                return Err(KebiError::Action("Не понял длительность".into()));
            }
            let secs = match unit {
                "s" => val, "m" => val * 60, "h" => val * 3600, _ => val * 60,
            };
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(secs as u64)).await;
                let _ = speak(&msg).await;
            });
            Ok(Some(format!("Таймер на {secs} сек")))
        }
        RemindKind::At => {
            let target = value.to_string();
            tokio::spawn(async move {
                loop {
                    let now = chrono::Local::now().format("%H:%M").to_string();
                    if now == target { let _ = speak(&msg).await; break; }
                    tokio::time::sleep(std::time::Duration::from_secs(20)).await;
                }
            });
            Ok(Some(format!("Напомню в {value}")))
        }
        RemindKind::Stopwatch => Ok(Some("Секундомер: v1.1".into())),
    }
}

fn parse_duration(s: &str) -> (i64, &'static str) {
    let lower = s.to_lowercase();
    let digits: String = lower.chars().filter(|c| c.is_ascii_digit()).collect();
    let val: i64 = digits.parse().unwrap_or(0);
    if lower.contains('с') || lower.contains('s') { (val, "s") }
    else if lower.contains('ч') || lower.contains('h') { (val, "h") }
    else if lower.contains('м') || lower.contains('m') { (val, "m") }
    else if val > 0 { (val, "s") } else { (0, "m") }
}
