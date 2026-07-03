//! Minimalist "midnight" theme. Made by KebiLab

use eframe::egui::{self, Color32, CornerRadius, Stroke, Style, Vec2, Visuals};

pub const BG_DEEP: Color32 = Color32::from_rgb(8, 11, 18);          // #080B12
pub const BG_PANEL: Color32 = Color32::from_rgb(15, 19, 30);        // #0F131E
pub const BG_RAISED: Color32 = Color32::from_rgb(20, 26, 42);       // #141A2A
pub const LINE: Color32 = Color32::from_rgb(36, 44, 66);            // #242C42
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(232, 238, 252);// #E8EEFC
pub const TEXT_MUTED: Color32 = Color32::from_rgb(140, 152, 180);  // #8C98B4
pub const ACCENT: Color32 = Color32::from_rgb(96, 165, 250);        // #60A5FA
pub const ACCENT_2: Color32 = Color32::from_rgb(34, 211, 238);      // #22D3EE
pub const SUCCESS: Color32 = Color32::from_rgb(74, 222, 128);
pub const WARN: Color32 = Color32::from_rgb(250, 204, 21);
pub const DANGER: Color32 = Color32::from_rgb(248, 113, 113);

pub fn install(ctx: &egui::Context) {
    let mut style = Style::default();
    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(TEXT_PRIMARY);
    visuals.window_fill = BG_PANEL;
    visuals.panel_fill = BG_PANEL;
    visuals.faint_bg_color = BG_RAISED;
    visuals.extreme_bg_color = BG_DEEP;
    visuals.hyperlink_color = ACCENT;
    visuals.widgets.noninteractive.bg_fill = BG_RAISED;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, LINE);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(10);
    visuals.widgets.inactive.bg_fill = BG_RAISED;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, LINE);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(10);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(28, 36, 56);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(10);
    visuals.widgets.active.bg_fill = Color32::from_rgb(38, 50, 78);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT_2);
    visuals.widgets.active.corner_radius = CornerRadius::same(10);
    visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(96, 165, 250, 50);
    visuals.selection.stroke = Stroke::new(1.0, ACCENT);

    visuals.window_corner_radius = CornerRadius::same(14);
    visuals.window_shadow = egui::Shadow { offset: Vec2::new(0.0, 8.0), blur: 24.0, spread: 0.0, color: Color32::from_rgba_unmultiplied(0, 0, 0, 120) };
    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(10.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(16);
    style.text_styles.get_mut(&egui::TextStyle::Body).map(|f| f.size = 14.0);
    style.text_styles.get_mut(&egui::TextStyle::Heading).map(|f| f.size = 20.0);
    style.text_styles.get_mut(&egui::TextStyle::Small).map(|f| f.size = 12.0);
    ctx.set_style(style);
}
