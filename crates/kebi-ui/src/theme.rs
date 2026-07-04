//! Themes. Three minimalist palettes. Made by KebiLab

use eframe::egui::{Color32, Rounding, Stroke, Style, Vec2, Visuals};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Midnight,
    Dawn,
    Forest,
}

impl Theme {
    pub fn from_code(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dawn" => Theme::Dawn,
            "forest" => Theme::Forest,
            _ => Theme::Midnight,
        }
    }
    pub fn code(&self) -> &'static str {
        match self {
            Theme::Midnight => "midnight",
            Theme::Dawn => "dawn",
            Theme::Forest => "forest",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub bg_deep: Color32,
    pub bg_panel: Color32,
    pub bg_raised: Color32,
    pub line: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_2: Color32,
    pub success: Color32,
    pub warn: Color32,
    pub danger: Color32,
    pub nav_active_bg: Color32,
}

impl Palette {
    pub const fn midnight() -> Self {
        Self {
            bg_deep: Color32::from_rgb(8, 11, 18),
            bg_panel: Color32::from_rgb(15, 19, 30),
            bg_raised: Color32::from_rgb(22, 28, 44),
            line: Color32::from_rgb(36, 44, 66),
            text: Color32::from_rgb(232, 238, 252),
            text_muted: Color32::from_rgb(140, 152, 180),
            accent: Color32::from_rgb(96, 165, 250),
            accent_2: Color32::from_rgb(34, 211, 238),
            success: Color32::from_rgb(74, 222, 128),
            warn: Color32::from_rgb(250, 204, 21),
            danger: Color32::from_rgb(248, 113, 113),
            nav_active_bg: Color32::from_rgb(28, 36, 56),
        }
    }
    pub const fn dawn() -> Self {
        Self {
            bg_deep: Color32::from_rgb(252, 250, 246),
            bg_panel: Color32::from_rgb(255, 255, 255),
            bg_raised: Color32::from_rgb(244, 240, 235),
            line: Color32::from_rgb(220, 215, 210),
            text: Color32::from_rgb(28, 30, 36),
            text_muted: Color32::from_rgb(110, 116, 130),
            accent: Color32::from_rgb(220, 90, 50),
            accent_2: Color32::from_rgb(180, 110, 40),
            success: Color32::from_rgb(60, 160, 90),
            warn: Color32::from_rgb(200, 140, 20),
            danger: Color32::from_rgb(200, 60, 60),
            nav_active_bg: Color32::from_rgb(244, 230, 220),
        }
    }
    pub const fn forest() -> Self {
        Self {
            bg_deep: Color32::from_rgb(8, 16, 12),
            bg_panel: Color32::from_rgb(14, 26, 20),
            bg_raised: Color32::from_rgb(20, 36, 28),
            line: Color32::from_rgb(36, 60, 48),
            text: Color32::from_rgb(230, 240, 232),
            text_muted: Color32::from_rgb(140, 160, 148),
            accent: Color32::from_rgb(120, 220, 160),
            accent_2: Color32::from_rgb(180, 220, 120),
            success: Color32::from_rgb(120, 220, 160),
            warn: Color32::from_rgb(220, 200, 80),
            danger: Color32::from_rgb(220, 100, 100),
            nav_active_bg: Color32::from_rgb(28, 50, 38),
        }
    }

    pub fn get(t: Theme) -> Self {
        match t {
            Theme::Midnight => Self::midnight(),
            Theme::Dawn => Self::dawn(),
            Theme::Forest => Self::forest(),
        }
    }
}

pub fn install(ctx: &eframe::egui::Context, theme: Theme) {
    let p = Palette::get(theme);
    let mut style = Style::default();
    let mut visuals = if p.bg_deep.r() < 64 { Visuals::dark() } else { Visuals::light() };
    visuals.override_text_color = Some(p.text);
    visuals.window_fill = p.bg_panel;
    visuals.panel_fill = p.bg_panel;
    visuals.faint_bg_color = p.bg_raised;
    visuals.extreme_bg_color = p.bg_deep;
    visuals.hyperlink_color = p.accent;
    let r10 = Rounding::same(10.0);
    visuals.widgets.noninteractive.bg_fill = p.bg_raised;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.line);
    visuals.widgets.noninteractive.rounding = r10;
    visuals.widgets.inactive.bg_fill = p.bg_raised;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, p.line);
    visuals.widgets.inactive.rounding = r10;
    visuals.widgets.hovered.bg_fill = p.nav_active_bg;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, p.accent);
    visuals.widgets.hovered.rounding = r10;
    visuals.widgets.active.bg_fill = p.nav_active_bg;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, p.accent_2);
    visuals.widgets.active.rounding = r10;
    visuals.window_rounding = Rounding::same(14.0);
    visuals.menu_rounding = r10;
    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(10.0, 8.0);
    ctx.set_style(style);
}
