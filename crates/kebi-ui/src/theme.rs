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
    pub bg_input: Color32,
    pub line: Color32,
    pub line_strong: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub accent_2: Color32,
    pub success: Color32,
    pub warn: Color32,
    pub danger: Color32,
    pub nav_bg: Color32,
    pub nav_active_bg: Color32,
    pub nav_hover_bg: Color32,
    pub nav_active_bar: Color32,
    pub shadow: Color32,
}

impl Palette {
    pub fn midnight() -> Self {
        Self {
            bg_deep: Color32::from_rgb(11, 15, 26),
            bg_panel: Color32::from_rgb(17, 23, 42),
            bg_raised: Color32::from_rgb(26, 34, 56),
            bg_input: Color32::from_rgb(22, 29, 48),
            line: Color32::from_rgb(37, 46, 74),
            line_strong: Color32::from_rgb(58, 70, 102),
            text: Color32::from_rgb(230, 234, 245),
            text_muted: Color32::from_rgb(138, 149, 181),
            accent: Color32::from_rgb(124, 156, 255),
            accent_soft: Color32::from_rgba_unmultiplied(124, 156, 255, 40),
            accent_2: Color32::from_rgb(94, 234, 212),
            success: Color32::from_rgb(134, 239, 172),
            warn: Color32::from_rgb(252, 211, 77),
            danger: Color32::from_rgb(252, 165, 165),
            nav_bg: Color32::from_rgb(8, 12, 22),
            nav_active_bg: Color32::from_rgba_unmultiplied(124, 156, 255, 30),
            nav_hover_bg: Color32::from_rgba_unmultiplied(255, 255, 255, 8),
            nav_active_bar: Color32::from_rgb(124, 156, 255),
            shadow: Color32::from_rgba_unmultiplied(0, 0, 0, 140),
        }
    }
    pub fn dawn() -> Self {
        Self {
            bg_deep: Color32::from_rgb(251, 250, 247),
            bg_panel: Color32::from_rgb(255, 255, 255),
            bg_raised: Color32::from_rgb(242, 237, 230),
            bg_input: Color32::from_rgb(248, 244, 238),
            line: Color32::from_rgb(226, 220, 211),
            line_strong: Color32::from_rgb(190, 180, 165),
            text: Color32::from_rgb(27, 31, 42),
            text_muted: Color32::from_rgb(91, 100, 112),
            accent: Color32::from_rgb(217, 119, 87),
            accent_soft: Color32::from_rgba_unmultiplied(217, 119, 87, 30),
            accent_2: Color32::from_rgb(154, 123, 79),
            success: Color32::from_rgb(60, 160, 90),
            warn: Color32::from_rgb(200, 140, 20),
            danger: Color32::from_rgb(200, 60, 60),
            nav_bg: Color32::from_rgb(248, 244, 238),
            nav_active_bg: Color32::from_rgba_unmultiplied(217, 119, 87, 25),
            nav_hover_bg: Color32::from_rgba_unmultiplied(0, 0, 0, 6),
            nav_active_bar: Color32::from_rgb(217, 119, 87),
            shadow: Color32::from_rgba_unmultiplied(120, 100, 80, 60),
        }
    }
    pub fn forest() -> Self {
        Self {
            bg_deep: Color32::from_rgb(10, 20, 16),
            bg_panel: Color32::from_rgb(17, 32, 26),
            bg_raised: Color32::from_rgb(23, 42, 34),
            bg_input: Color32::from_rgb(20, 38, 30),
            line: Color32::from_rgb(36, 59, 48),
            line_strong: Color32::from_rgb(60, 90, 75),
            text: Color32::from_rgb(230, 242, 236),
            text_muted: Color32::from_rgb(138, 162, 152),
            accent: Color32::from_rgb(125, 211, 160),
            accent_soft: Color32::from_rgba_unmultiplied(125, 211, 160, 40),
            accent_2: Color32::from_rgb(189, 224, 104),
            success: Color32::from_rgb(125, 211, 160),
            warn: Color32::from_rgb(220, 200, 80),
            danger: Color32::from_rgb(220, 100, 100),
            nav_bg: Color32::from_rgb(6, 14, 10),
            nav_active_bg: Color32::from_rgba_unmultiplied(125, 211, 160, 30),
            nav_hover_bg: Color32::from_rgba_unmultiplied(255, 255, 255, 8),
            nav_active_bar: Color32::from_rgb(125, 211, 160),
            shadow: Color32::from_rgba_unmultiplied(0, 0, 0, 140),
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

    let r_md = Rounding::same(12.0);
    let r_lg = Rounding::same(16.0);

    visuals.widgets.noninteractive.bg_fill = p.bg_raised;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.line);
    visuals.widgets.noninteractive.rounding = r_md;
    visuals.widgets.inactive.bg_fill = p.bg_raised;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, p.line);
    visuals.widgets.inactive.rounding = r_md;
    visuals.widgets.hovered.bg_fill = p.nav_hover_bg;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, p.accent);
    visuals.widgets.hovered.rounding = r_md;
    visuals.widgets.active.bg_fill = p.accent_soft;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, p.accent);
    visuals.widgets.active.rounding = r_md;

    visuals.window_rounding = r_lg;
    visuals.menu_rounding = r_md;
    visuals.window_shadow = eframe::egui::epaint::Shadow {
        offset: Vec2::new(0.0, 8.0),
        blur: 24.0,
        spread: 0.0,
        color: p.shadow,
    };

    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(12.0, 10.0);
    style.spacing.window_margin = eframe::egui::Margin::same(20.0);
    style.spacing.button_padding = Vec2::new(14.0, 8.0);
    if let Some(f) = style.text_styles.get_mut(&eframe::egui::TextStyle::Body) {
        f.size = 14.0;
    }
    if let Some(f) = style.text_styles.get_mut(&eframe::egui::TextStyle::Heading) {
        f.size = 20.0;
    }
    if let Some(f) = style.text_styles.get_mut(&eframe::egui::TextStyle::Small) {
        f.size = 12.0;
    }
    if let Some(f) = style.text_styles.get_mut(&eframe::egui::TextStyle::Button) {
        f.size = 13.0;
    }
    ctx.set_style(style);
}
