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

/// Load the application icon from the bundled .ico file.
pub fn load_icon() -> Option<eframe::egui::IconData> {
    let bytes = include_bytes!("../../../assets/icon/kebicontrol.ico");
    let dir = ico::IconDir::read(std::io::Cursor::new(&bytes)).ok()?;
    let entry = dir.entries().first()?.clone();
    let img = entry.decode().ok()?;
    let rgba = img.rgba_data().to_vec();
    let (w, h) = (img.width() as u32, img.height() as u32);
    Some(eframe::egui::IconData { rgba, width: w, height: h })
}
