//! App state. Made by KebiLab

use crate::AppState;
use anyhow::Result;
use kebi_core::{AppPaths, Config, Profile};
use kebi_llm::LlmClient;
use kebi_ui::{OverlayApp, SettingsApp, TrayIcon};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct AppState {
    pub config: parking_lot::Mutex<Config>,
    pub profile: parking_lot::Mutex<Profile>,
    pub paths: AppPaths,
    pub llm: Arc<Mutex<LlmClient>>,
    pub quit: Arc<AtomicBool>,
    pub overlay_visible: Arc<AtomicBool>,
    pub overlay_status: parking_lot::Mutex<String>,
    pub overlay_tx: mpsc::UnboundedSender<String>,
    pub settings_tx: mpsc::UnboundedSender<()>,
}

impl AppState {
    pub fn new(config: Config, profile: Profile, paths: AppPaths, llm: Arc<Mutex<LlmClient>>) -> Self {
        let (overlay_tx, _overlay_rx) = mpsc::unbounded_channel();
        let (settings_tx, _settings_rx) = mpsc::unbounded_channel();
        Self {
            config: parking_lot::Mutex::new(config),
            profile: parking_lot::Mutex::new(profile),
            paths,
            llm,
            quit: Arc::new(AtomicBool::new(false)),
            overlay_visible: Arc::new(AtomicBool::new(false)),
            overlay_status: parking_lot::Mutex::new("Готов".into()),
            overlay_tx,
            settings_tx,
        }
    }
    pub fn is_quit(&self) -> bool { self.quit.load(Ordering::SeqCst) }
    pub fn request_quit(&self) { self.quit.store(true, Ordering::SeqCst); }
    pub fn set_overlay_status(&self, s: impl Into<String>) { *self.overlay_status.lock() = s.into(); }
    pub fn config(&self) -> Config { self.config.lock().clone() }
    pub fn profile(&self) -> Profile { self.profile.lock().clone() }
}

pub async fn wire(state: Arc<AppState>) -> Result<()> {
    // Tray
    let _tray = TrayIcon::install(&egui::default_context())?;
    // Overlay window
    std::thread::spawn({
        let state = state.clone();
        move || {
            let cfg = state.config();
            let mut app = OverlayApp::new(cfg.general.wake_word.clone());
            let visible = state.overlay_visible.clone();
            app.visible = visible;
            let native_options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default()
                    .with_inner_size([520.0, 420.0])
                    .with_transparent(true)
                    .with_decorations(false)
                    .with_always_on_top(),
                ..Default::default()
            };
            let _ = eframe::run_native("KebiControl Overlay", native_options, Box::new(|_cc| Box::new(app)));
        }
    });
    // Settings window (open on demand from tray)
    std::thread::spawn({
        let state = state.clone();
        move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(300));
                if state.is_quit() { break; }
            }
        }
    });
    Ok(())
}
