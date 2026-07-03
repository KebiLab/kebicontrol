//! System actions: power, volume, brightness, screenshot, reminders, TTS toggle.
//! Made by KebiLab

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
    // Real implementation: invoke kebi-tts. Here we emit a tracing event so
    // the core crate has no Windows-only dependencies.
    tracing::info!(text = _text, "tts: speak");
    Ok(())
}

pub async fn power(op: PowerOp) -> Result<Option<String>> {
    let (verb, exe, args) = match op {
        PowerOp::Shutdown => ("Выключаю компьютер", "shutdown", ["/s", "/t", "0"]),
        PowerOp::Restart => ("Перезагружаю", "shutdown", ["/r", "/t", "0"]),
        PowerOp::Sleep => ("Усыпляю", "rundll32", ["powrprof.dll,SetSuspendState", "0", "1", "0"]),
        PowerOp::Hibernate => ("Гибернация", "shutdown", ["/h"]),
        PowerOp::Lock => ("Блокирую", "rundll32", ["user32.dll,LockWorkStation"]),
        PowerOp::SignOut => ("Выхожу", "shutdown", ["/l"]),
    };
    StdCommand::new(exe)
        .args(args)
        .spawn()
        .map_err(|e| KebiError::Action(format!("{verb}: {e}")))?;
    Ok(Some(verb.into()))
}

pub async fn volume(op: VolumeOp, value: Option<u8>) -> Result<Option<String>> {
    use windows::Win32::Media::Audio::{
        eConsole, Endpoints::IAudioEndpointVolume, IMMDeviceEnumerator, MMDeviceEnumerator,
    };
    use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, COINIT_APARTMENTTHREADED};

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, windows::Win32::System::Com::CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eConsole, 0)?;
        let endpoint: IAudioEndpointVolume = device.Activate(
            windows::Win32::System::Com::CLSCTX_ALL,
            None,
        )?;
        let msg = match op {
            VolumeOp::Up => {
                endpoint.StepUp()?;
                "Громче"
            }
            VolumeOp::Down => {
                endpoint.StepDown()?;
                "Тише"
            }
            VolumeOp::Mute => {
                endpoint.SetMute(true, std::ptr::null())?;
                "Звук выключен"
            }
            VolumeOp::Unmute => {
                endpoint.SetMute(false, std::ptr::null())?;
                "Звук включён"
            }
            VolumeOp::Toggle => {
                let m = endpoint.GetMute()?;
                endpoint.SetMute(!m.as_bool(), std::ptr::null())?;
                if m.as_bool() { "Звук включён" } else { "Звук выключен" }
            }
            VolumeOp::Set => {
                if let Some(v) = value {
                    let scalar = v as f32 / 100.0;
                    endpoint.SetMasterVolumeLevelScalar(scalar, std::ptr::null())?;
                    "Громкость установлена"
                } else {
                    "Не указано значение громкости"
                }
            }
        };
        Ok(Some(msg.into()))
    }
}

pub async fn brightness(value: u8) -> Result<Option<String>> {
    // Without WMI / monitor APIs, use PowerShell to set WMI brightness.
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
    let args: &[&str] = match mode {
        ScreenshotMode::Full => &[],
        ScreenshotMode::Window => &[],
        ScreenshotMode::Selection => return Ok(Some("Режим выделения: реализуется через Win+Shift+S".into())),
    };
    let _ = args;
    // Trigger PrintScreen via SendInput (full screen copy to clipboard).
    input_send_print_screen()?;
    Ok(Some("Скриншот сделан".into()))
}

fn input_send_print_screen() -> Result<()> {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_SNAPSHOT,
    };
    unsafe {
        let mut input = INPUT::default();
        input.r#type = INPUT_KEYBOARD;
        input.Anonymous.ki.wVk = VK_SNAPSHOT;
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        let mut up = input;
        up.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0);
        SendInput(&[up], std::mem::size_of::<INPUT>() as i32);
    }
    Ok(())
}

pub async fn remind(kind: RemindKind, value: &str, text: Option<&str>) -> Result<Option<String>> {
    match kind {
        RemindKind::Timer => {
            // Parse "5 минут" / "1h" / "30 секунд"
            let (val, unit) = parse_duration(value);
            if val <= 0 {
                return Err(KebiError::Action("Не понял длительность".into()));
            }
            let secs = match unit {
                "s" | "сек" | "секунд" | "second" | "seconds" => val,
                "m" | "мин" | "минут" | "minute" | "minutes" => val * 60,
                "h" | "час" | "часа" | "часов" | "hour" | "hours" => val * 3600,
                _ => val * 60,
            };
            let msg = text.unwrap_or("Время вышло").to_string();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(secs as u64)).await;
                let _ = speak(&msg).await;
                let _ = StdCommand::new("powershell")
                    .args([
                        "-NoProfile",
                        "-Command",
                        "[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); \
                         [System.Windows.Forms.MessageBox]::Show('Время вышло!')",
                    ])
                    .spawn();
            });
            Ok(Some(format!("Таймер на {secs} сек")))
        }
        RemindKind::At => {
            // value is HH:MM
            let msg = text.unwrap_or("Напоминание").to_string();
            tokio::spawn(async move {
                loop {
                    let now = chrono::Local::now().format("%H:%M").to_string();
                    if now == value {
                        let _ = speak(&msg).await;
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(20)).await;
                }
            });
            Ok(Some(format!("Напомню в {value}")))
        }
        RemindKind::Stopwatch => {
            Ok(Some("Секундомер: в v1.1".into()))
        }
    }
}

fn parse_duration(s: &str) -> (i64, &'static str) {
    let lower = s.to_lowercase();
    let digits: String = lower.chars().filter(|c| c.is_ascii_digit()).collect();
    let val: i64 = digits.parse().unwrap_or(0);
    if lower.contains("сек") || lower.contains("sec") {
        (val, "s")
    } else if lower.contains("час") || lower.contains("hour") || lower.contains('h') {
        (val, "h")
    } else if lower.contains("мин") || lower.contains("min") || lower.contains('m') {
        (val, "m")
    } else if val > 0 {
        (val, "s")
    } else {
        (0, "m")
    }
}
