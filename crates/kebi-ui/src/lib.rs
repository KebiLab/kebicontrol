//! UI module. Made by KebiLab

pub mod app;
pub mod settings_view;
pub mod icons;
pub mod theme;
pub mod i18n;
pub mod hotkeys;

pub use app::{MainApp, Page, HistoryEntry, Status};
pub use theme::{Theme, Palette};
pub use i18n::Lang;
