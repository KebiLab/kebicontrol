//! UI module. Made by KebiLab

pub mod tray;
pub mod overlay;
pub mod settings;
pub mod theme;
pub mod autostart;
pub mod hotkeys;

pub use tray::TrayIcon;
pub use overlay::OverlayApp;
pub use settings::SettingsApp;
pub use autostart::set_autostart;
