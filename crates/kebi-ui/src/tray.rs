//! System tray. Made by KebiLab

use anyhow::Result;
use eframe::egui::Context;
use std::sync::{Arc, Mutex};
use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIconBuilder};

pub struct TrayIcon {
    inner: tray_icon::TrayIcon,
    pub show_overlay: Arc<Mutex<bool>>,
    pub show_settings: Arc<Mutex<bool>>,
    pub quit: Arc<Mutex<bool>>,
    pub listening: Arc<Mutex<bool>>,
}

impl TrayIcon {
    pub fn install(ctx: &Context) -> Result<Arc<Mutex<Self>>> {
        let show_overlay = Arc::new(Mutex::new(false));
        let show_settings = Arc::new(Mutex::new(false));
        let quit = Arc::new(Mutex::new(false));
        let listening = Arc::new(Mutex::new(false));

        let show_overlay_m = show_overlay.clone();
        let show_settings_m = show_settings.clone();
        let quit_m = quit.clone();

        let menu = Menu::new();
        let mi_overlay = MenuItem::with_id("overlay", "Open overlay (Ctrl+Shift+K)", true, None);
        let mi_settings = MenuItem::with_id("settings", "Settings…", true, None);
        let mi_quit = MenuItem::with_id("quit", "Quit", true, None);
        menu.append_items(&[
            &PredefinedMenuItem::separator(),
            &mi_overlay,
            &mi_settings,
            &PredefinedMenuItem::separator(),
            &mi_quit,
        ])?;
        let menu_channel = tray_icon::menu::MenuEvent::receiver().to_owned();
        let _ = menu_channel;

        let icon = load_default_icon()?;

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("KebiControl — voice control. Made by KebiLab")
            .with_icon(icon)
            .build()?;

        // Menu event handling
        let show_overlay_e = show_overlay_m.clone();
        let show_settings_e = show_settings_m.clone();
        let quit_e = quit_m.clone();
        std::thread::spawn(move || {
            let receiver = tray_icon::menu::MenuEvent::receiver();
            for ev in receiver.iter() {
                match ev.id.as_ref() {
                    "overlay" => { *show_overlay_e.lock().unwrap() = true; }
                    "settings" => { *show_settings_e.lock().unwrap() = true; }
                    "quit" => { *quit_e.lock().unwrap() = true; }
                    _ => {}
                }
            }
        });

        let me = Self { inner: tray, show_overlay, show_settings, quit, listening };
        Ok(Arc::new(Mutex::new(me)))
    }

    pub fn set_listening(&mut self, on: bool) {
        *self.listening.lock().unwrap() = on;
    }
}

fn load_default_icon() -> Result<Icon> {
    use std::io::Cursor;
    // Embed the ICO at build time via include_bytes
    let bytes = include_bytes!("../../assets/icon/kebicontrol.ico");
    let dir = ico::IconDir::read(Cursor::new(&bytes))?;
    let entry = dir.entries().first().context("no ico entry")?;
    let img = entry.decode()?;
    let rgba = img.rgba_data().to_vec();
    let (w, h) = (img.width(), img.height());
    let icon = Icon::from_rgba(rgba, w as u32, h as u32)?;
    Ok(icon)
}

use anyhow::Context;
