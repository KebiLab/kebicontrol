//! Minimalist theme. Two palettes. Made by KebiLab

use eframe::egui::{Color32, Rounding, Stroke, Style, Vec2, Visuals};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn from_code(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light" => Theme::Light,
            _ => Theme::Dark,
        }
    }
    pub fn code(&self) -> &'static str { match self { Theme::Dark => "dark", Theme::Light => "light" } }
    pub fn label(self) -> &'static str { match self { Theme::Dark => "Тёмная", Theme::Light => "Светлая" } }
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub bg: Color32,
    pub surface: Color32,
    pub surface_2: Color32,
    pub line: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_text: Color32,
    pub success: Color32,
    pub danger: Color32,
}

impl Palette {
    pub fn dark() -> Self {
        Self {
            bg: Color32::from_rgb(10, 13, 22),         // #0A0D16
            surface: Color32::from_rgb(20, 26, 42),    // #141A2A
            surface_2: Color32::from_rgb(28, 36, 58),  // #1C243A
            line: Color32::from_rgb(38, 48, 76),       // #26304C
            text: Color32::from_rgb(245, 247, 252),    // #F5F7FC
            text_muted: Color32::from_rgb(150, 162, 188), // #96A2BC
            accent: Color32::from_rgb(95, 158, 255),   // #5F9EFF
            accent_text: Color32::from_rgb(255, 255, 255),
            success: Color32::from_rgb(80, 200, 140),
            danger: Color32::from_rgb(232, 100, 100),
        }
    }
    pub fn light() -> Self {
        Self {
            bg: Color32::from_rgb(248, 249, 252),      // #F8F9FC
            surface: Color32::from_rgb(255, 255, 255),
            surface_2: Color32::from_rgb(240, 243, 250),
            line: Color32::from_rgb(220, 226, 238),
            text: Color32::from_rgb(20, 26, 38),
            text_muted: Color32::from_rgb(110, 120, 140),
            accent: Color32::from_rgb(70, 130, 240),
            accent_text: Color32::from_rgb(255, 255, 255),
            success: Color32::from_rgb(40, 160, 100),
            danger: Color32::from_rgb(200, 70, 70),
        }
    }
    pub fn get(t: Theme) -> Self { match t { Theme::Dark => Self::dark(), Theme::Light => Self::light() } }
    pub fn is_dark(&self) -> bool { self.bg.r() < 64 }
}

pub fn install(ctx: &eframe::egui::Context, p: &Palette) {
    let mut style = Style::default();
    let mut visuals = if p.is_dark() { Visuals::dark() } else { Visuals::light() };
    visuals.override_text_color = Some(p.text);
    visuals.window_fill = p.bg;
    visuals.panel_fill = p.bg;
    visuals.faint_bg_color = p.surface;
    visuals.extreme_bg_color = p.bg;
    visuals.hyperlink_color = p.accent;

    let r = Rounding::same(10.0);
    visuals.widgets.noninteractive.bg_fill = p.surface;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.line);
    visuals.widgets.noninteractive.rounding = r;
    visuals.widgets.inactive.bg_fill = p.surface;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, p.line);
    visuals.widgets.inactive.rounding = r;
    visuals.widgets.hovered.bg_fill = p.surface_2;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, p.accent);
    visuals.widgets.hovered.rounding = r;
    visuals.widgets.active.bg_fill = p.accent;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, p.accent);
    visuals.widgets.active.rounding = r;
    visuals.window_rounding = Rounding::same(14.0);
    visuals.menu_rounding = r;
    visuals.window_shadow = eframe::egui::epaint::Shadow {
        offset: Vec2::new(0.0, 6.0), blur: 18.0, spread: 0.0,
        color: if p.is_dark() { Color32::from_rgba_unmultiplied(0, 0, 0, 120) }
               else { Color32::from_rgba_unmultiplied(60, 80, 120, 40) },
    };

    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(12.0, 10.0);
    style.spacing.window_margin = eframe::egui::Margin::same(20.0);
    style.spacing.button_padding = Vec2::new(18.0, 10.0);
    if let Some(f) = style.text_styles.get_mut(&eframe::egui::TextStyle::Body) { f.size = 14.0; }
    if let Some(f) = style.text_styles.get_mut(&eframe::egui::TextStyle::Heading) { f.size = 24.0; }
    if let Some(f) = style.text_styles.get_mut(&eframe::egui::TextStyle::Small) { f.size = 12.0; }
    if let Some(f) = style.text_styles.get_mut(&eframe::egui::TextStyle::Button) { f.size = 14.0; }
    if let Some(f) = style.text_styles.get_mut(&eframe::egui::TextStyle::Monospace) { f.size = 13.0; }
    ctx.set_style(style);
}
