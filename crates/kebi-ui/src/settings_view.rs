//! Settings page (inline). Made by KebiLab

use crate::app::MainApp;
use crate::icons::{self, Icon};
use crate::i18n::{self, Lang};
use crate::theme::Palette;
use eframe::egui::{self, RichText, Vec2};

pub fn page_settings(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "settings.title")).size(24.0).strong().color(p.text),
    ));
    ui.add_space(20.0);

    // General
    section(ui, p, &i18n::t(lang, "settings.section.general"));
    field(ui, p, &i18n::t(lang, "settings.field.wake"), |ui| {
        ui.text_edit_singleline(&mut app.config.general.wake_word);
    });
    field(ui, p, &i18n::t(lang, "settings.field.lang"), |ui| {
        egui::ComboBox::from_id_source("set_lang")
            .selected_text(lang.label())
            .width(140.0)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.config.general.language, "ru".into(), "Русский");
                ui.selectable_value(&mut app.config.general.language, "en".into(), "English");
            });
    });
    ui.checkbox(&mut app.config.general.tts_enabled, i18n::t(lang, "settings.field.tts"));
    ui.checkbox(&mut app.config.general.autostart, i18n::t(lang, "settings.field.autostart"));
    ui.add_space(12.0);

    // Appearance
    section(ui, p, &i18n::t(lang, "settings.section.appearance"));
    field(ui, p, &i18n::t(lang, "settings.field.theme"), |ui| {
        egui::ComboBox::from_id_source("set_theme")
            .selected_text(i18n::t(lang, match app.config.ui.theme.as_str() {
                "dawn" => "theme.dawn",
                "forest" => "theme.forest",
                _ => "theme.midnight",
            }))
            .width(140.0)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.config.ui.theme, "midnight".into(), i18n::t(lang, "theme.midnight"));
                ui.selectable_value(&mut app.config.ui.theme, "dawn".into(), i18n::t(lang, "theme.dawn"));
                ui.selectable_value(&mut app.config.ui.theme, "forest".into(), i18n::t(lang, "theme.forest"));
            });
    });
    ui.add_space(12.0);

    // LLM
    section(ui, p, &i18n::t(lang, "settings.section.llm"));
    field(ui, p, &i18n::t(lang, "settings.field.provider"), |ui| {
        egui::ComboBox::from_id_source("set_provider")
            .selected_text(app.config.llm.provider.clone())
            .width(140.0)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.config.llm.provider, "opencode".into(), "OpenCode Go");
                ui.selectable_value(&mut app.config.llm.provider, "deepseek".into(), "DeepSeek");
                ui.selectable_value(&mut app.config.llm.provider, "mimo".into(), "MiMo");
                ui.selectable_value(&mut app.config.llm.provider, "nvidia".into(), "NVIDIA");
                ui.selectable_value(&mut app.config.llm.provider, "custom".into(), "Свой");
            });
    });
    field(ui, p, &i18n::t(lang, "settings.field.baseurl"), |ui| {
        ui.text_edit_singleline(&mut app.config.llm.base_url);
    });
    field(ui, p, &i18n::t(lang, "settings.field.model"), |ui| {
        ui.text_edit_singleline(&mut app.config.llm.model);
    });
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        let btn = egui::Button::new(
            RichText::new(format!("  {}", i18n::t(lang, "settings.save"))).color(p.text).size(13.0),
        )
        .fill(p.accent)
        .rounding(egui::Rounding::same(8.0))
        .min_size(Vec2::new(140.0, 38.0));
        let resp = ui.add(btn).on_hover_cursor(egui::CursorIcon::PointingHand);
        icons::draw(ui, egui::Rect::from_min_size(
            egui::Pos2::new(resp.rect.left() + 18.0, resp.rect.center().y - 7.0),
            Vec2::new(14.0, 14.0),
        ), p.text, 1.3, Icon::Save);
        if resp.clicked() {
            let result = app.config.save(&kebi_core::AppPaths::new());
            app.settings_msg = Some(if result.is_ok() {
                i18n::t(lang, "settings.saved")
            } else {
                i18n::t(lang, "settings.error")
            });
        }
    });
    if let Some(msg) = &app.settings_msg {
        ui.add_space(6.0);
        ui.add(egui::Label::new(
            RichText::new(msg).color(p.accent_2).size(12.0),
        ));
    }
}

fn section(ui: &mut egui::Ui, p: &Palette, title: &str) {
    ui.add(egui::Label::new(
        RichText::new(title).strong().color(p.text).size(13.0),
    ));
    ui.add_space(4.0);
    let _ = egui::Frame::none()
        .fill(p.bg_raised)
        .stroke(egui::Stroke::new(1.0, p.line))
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::Margin::same(14.0))
        .show(ui, |ui| {});
    ui.add_space(10.0);
}

fn field<F: FnOnce(&mut egui::Ui)>(ui: &mut egui::Ui, p: &Palette, label: &str, add: F) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new(
            RichText::new(label).color(p.text_muted).size(13.0),
        ));
        ui.add_space(8.0);
        add(ui);
    });
    ui.add_space(4.0);
}
