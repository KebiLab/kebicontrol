//! Theme: minimalist dark. Made by KebiLab

use eframe::egui::{self, Color32, Rounding, Stroke, Style, Vec2, Visuals};

pub const BG_DEEP: Color32 = Color32::from_rgb(8, 11, 18);
pub const BG_PANEL: Color32 = Color32::from_rgb(15, 19, 30);
pub const BG_RAISED: Color32 = Color32::from_rgb(20, 26, 42);
pub const LINE: Color32 = Color32::from_rgb(36, 44, 66);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(232, 238, 252);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(140, 152, 180);
pub const ACCENT: Color32 = Color32::from_rgb(96, 165, 250);
pub const ACCENT_2: Color32 = Color32::from_rgb(34, 211, 238);

pub fn install(ctx: &egui::Context) {
    let mut style = Style::default();
    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(TEXT_PRIMARY);
    visuals.window_fill = BG_PANEL;
    visuals.panel_fill = BG_PANEL;
    visuals.faint_bg_color = BG_RAISED;
    visuals.extreme_bg_color = BG_DEEP;
    let r10 = Rounding::same(10.0);
    let r14 = Rounding::same(14.0);
    visuals.widgets.noninteractive.bg_fill = BG_RAISED;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, LINE);
    visuals.widgets.noninteractive.rounding = r10;
    visuals.widgets.inactive.bg_fill = BG_RAISED;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, LINE);
    visuals.widgets.inactive.rounding = r10;
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(28, 36, 56);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.hovered.rounding = r10;
    visuals.widgets.active.bg_fill = Color32::from_rgb(38, 50, 78);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT_2);
    visuals.widgets.active.rounding = r10;
    visuals.window_rounding = r14;
    visuals.menu_rounding = r10;
    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(10.0, 8.0);
    ctx.set_style(style);
}
