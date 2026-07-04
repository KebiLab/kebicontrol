//! Settings page. Made by KebiLab

use crate::app::{MainApp, SettingsSection};
use crate::icons::{self, Icon};
use crate::i18n::{self, Lang};
use crate::theme::Palette;
use eframe::egui::{self, Margin, RichText, Vec2};
use kebi_llm::providers::LlmProvider;

pub fn page_settings(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    // Sub-navigation tabs
    ui.horizontal(|ui| {
        section_tab(ui, p, app, SettingsSection::General, &i18n::t(lang, "settings.section.general"));
        ui.add_space(6.0);
        section_tab(ui, p, app, SettingsSection::Appearance, &i18n::t(lang, "settings.section.appearance"));
        ui.add_space(6.0);
        section_tab(ui, p, app, SettingsSection::Llm, &i18n::t(lang, "settings.section.llm"));
        ui.add_space(6.0);
        section_tab(ui, p, app, SettingsSection::Audio, &i18n::t(lang, "settings.section.audio"));
        ui.add_space(6.0);
        section_tab(ui, p, app, SettingsSection::Hotkeys, &i18n::t(lang, "nav.hotkeys"));
    });
    ui.add_space(20.0);

    egui::Frame::none()
        .fill(p.bg_raised)
        .stroke(egui::Stroke::new(1.0, p.line))
        .rounding(egui::Rounding::same(14.0))
        .inner_margin(Margin::same(20.0))
        .show(ui, |ui| {
            match app.settings_section {
                SettingsSection::General => section_general(ui, p, lang, app),
                SettingsSection::Appearance => section_appearance(ui, p, lang, app),
                SettingsSection::Llm => section_llm(ui, p, lang, app),
                SettingsSection::Audio => section_audio(ui, p, lang, app),
                SettingsSection::Hotkeys => section_hotkeys(ui, p, lang),
            }
        });
}

use eframe::egui::Color32;

fn section_tab(ui: &mut egui::Ui, p: &Palette, app: &mut MainApp, sec: SettingsSection, label: &str) {
    let active = app.settings_section == sec;
    let c_text = if active { p.text } else { p.text_muted };
    let bg = if active { p.accent_soft } else { Color32::TRANSPARENT };
    let resp = ui.add(
        egui::Button::new(RichText::new(format!("  {label}")).color(c_text).size(12.0))
            .fill(bg)
            .stroke(if active { egui::Stroke::new(1.0, p.accent) } else { egui::Stroke::NONE })
            .rounding(egui::Rounding::same(8.0))
            .min_size(Vec2::new(0.0, 32.0)),
    );
    if resp.clicked() { app.settings_section = sec; }
}

fn section_general(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    section_header(ui, p, &i18n::t(lang, "settings.section.general"));
    ui.add_space(12.0);
    field(ui, p, &i18n::t(lang, "settings.field.wake"), |ui| {
        ui.add(egui::TextEdit::singleline(&mut app.config.general.wake_word)
            .desired_width(280.0));
    });
    field(ui, p, &i18n::t(lang, "settings.field.lang"), |ui| {
        egui::ComboBox::from_id_source("set_lang")
            .selected_text(lang.label())
            .width(160.0)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.config.general.language, "ru".into(), "Русский");
                ui.selectable_value(&mut app.config.general.language, "en".into(), "English");
            });
    });
    ui.checkbox(&mut app.config.general.tts_enabled, i18n::t(lang, "settings.field.tts"));
    ui.checkbox(&mut app.config.general.autostart, i18n::t(lang, "settings.field.autostart"));
    ui.add_space(16.0);
    save_button(ui, p, lang, app);
}

fn section_appearance(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    section_header(ui, p, &i18n::t(lang, "settings.section.appearance"));
    ui.add_space(12.0);
    field(ui, p, &i18n::t(lang, "settings.field.theme"), |ui| {
        egui::ComboBox::from_id_source("set_theme")
            .selected_text(i18n::t(lang, match app.config.ui.theme.as_str() {
                "dawn" => "theme.dawn",
                "forest" => "theme.forest",
                _ => "theme.midnight",
            }))
            .width(160.0)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.config.ui.theme, "midnight".into(), i18n::t(lang, "theme.midnight"));
                ui.selectable_value(&mut app.config.ui.theme, "dawn".into(), i18n::t(lang, "theme.dawn"));
                ui.selectable_value(&mut app.config.ui.theme, "forest".into(), i18n::t(lang, "theme.forest"));
            });
    });
    ui.add_space(16.0);
    save_button(ui, p, lang, app);
}

fn section_llm(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    section_header(ui, p, &i18n::t(lang, "settings.section.llm"));
    ui.add_space(12.0);

    let provider = LlmProvider::from_code(&app.config.llm.provider);

    // Provider
    field(ui, p, &i18n::t(lang, "settings.field.provider"), |ui| {
        egui::ComboBox::from_id_source("set_provider")
            .selected_text(provider_label(provider))
            .width(220.0)
            .show_ui(ui, |ui| {
                for pp in LlmProvider::all() {
                    ui.selectable_value(&mut app.config.llm.provider, pp.code().into(), provider_label(*pp));
                }
            });
    });

    // Model (preset list for selected provider)
    let models = provider.default_models();
    field(ui, p, &i18n::t(lang, "settings.field.model"), |ui| {
        egui::ComboBox::from_id_source("set_model")
            .selected_text(if models.iter().any(|m| m == &app.config.llm.model) {
                app.config.llm.model.clone()
            } else {
                "Своя".to_string()
            })
            .width(280.0)
            .show_ui(ui, |ui| {
                for m in models {
                    ui.selectable_value(&mut app.config.llm.model, m.to_string(), *m);
                }
                ui.selectable_value(&mut app.config.llm.model, "".into(), "Своя (ввести вручную)");
            });
        if !models.iter().any(|m| *m == app.config.llm.model) {
            ui.add_space(6.0);
            ui.add(egui::TextEdit::singleline(&mut app.config.llm.model)
                .hint_text("model-name")
                .desired_width(280.0));
        }
    });

    // Base URL
    if provider == LlmProvider::Custom {
        field(ui, p, &i18n::t(lang, "settings.field.baseurl"), |ui| {
            ui.add(egui::TextEdit::singleline(&mut app.config.llm.base_url)
                .hint_text("https://api.example.com/v1")
                .desired_width(360.0));
        });
    } else {
        field(ui, p, "Base URL", |ui| {
            ui.add(egui::Label::new(
                RichText::new(provider.default_base_url()).color(p.text_muted).size(12.0),
            ));
        });
    }

    // API key with show/hide
    let key_set = !app.config.llm.api_key_enc.is_empty();
    field(ui, p, &i18n::t(lang, "settings.field.apikey"), |ui| {
        let mut edit = egui::TextEdit::singleline(&mut app.api_key_input);
        if !app.show_api_key { edit = edit.password(true); }
        ui.add(edit
            .hint_text(if key_set { "•••••••• (зашифрован, введите чтобы заменить)" } else { "sk-..." })
            .desired_width(320.0));
        let eye_icon = if app.show_api_key { Icon::EyeOff } else { Icon::Eye };
        let (eye_rect, eye_resp) = ui.allocate_exact_size(Vec2::new(20.0, 20.0), egui::Sense::click());
        icons::draw(ui, eye_rect, p.text_muted, 1.0, eye_icon);
        if eye_resp.clicked() {
            app.show_api_key = !app.show_api_key;
        }
    });
    if key_set {
        ui.add(egui::Label::new(
            RichText::new("Ключ сохранён (DPAPI). Очистите поле, чтобы удалить.")
                .color(p.success).size(11.0),
        ));
        ui.add_space(4.0);
    }

    ui.add_space(16.0);
    save_button(ui, p, lang, app);
}

fn section_audio(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    section_header(ui, p, &i18n::t(lang, "settings.section.audio"));
    ui.add_space(12.0);
    field(ui, p, "Sample rate", |ui| {
        ui.add(egui::DragValue::new(&mut app.config.audio.sample_rate).clamp_range(8000..=48000));
    });
    field(ui, p, "VAD threshold", |ui| {
        ui.add(egui::DragValue::new(&mut app.config.audio.vad_threshold).clamp_range(0.001..=0.2));
    });
    field(ui, p, "Silence, ms", |ui| {
        ui.add(egui::DragValue::new(&mut app.config.audio.silence_ms).clamp_range(100..=3000));
    });
    ui.add_space(16.0);
    save_button(ui, p, lang, app);
}

fn section_hotkeys(ui: &mut egui::Ui, p: &Palette, lang: Lang) {
    section_header(ui, p, &i18n::t(lang, "nav.hotkeys"));
    ui.add_space(12.0);
    let hk = [
        ("Ctrl+Shift+Space", "hotkeys.listen"),
        ("Esc", "hotkeys.cancel"),
        ("Ctrl+Shift+M", "hotkeys.tts"),
        ("Ctrl+Shift+D", "hotkeys.dictation"),
        ("Ctrl+Shift+P", "hotkeys.pause"),
    ];
    for (k, label_key) in hk {
        ui.horizontal(|ui| {
            ui.add(egui::Label::new(RichText::new(k).color(p.accent).size(13.0)));
            ui.add_space(20.0);
            ui.add(egui::Label::new(
                RichText::new(i18n::t(lang, label_key)).color(p.text_muted).size(13.0),
            ));
        });
        ui.add_space(6.0);
    }
}

fn provider_label(p: LlmProvider) -> &'static str {
    match p {
        LlmProvider::OpenCode => "OpenCode Go",
        LlmProvider::OpenAI => "OpenAI",
        LlmProvider::Anthropic => "Anthropic (Claude)",
        LlmProvider::Google => "Google Gemini",
        LlmProvider::Mistral => "Mistral AI",
        LlmProvider::Groq => "Groq",
        LlmProvider::DeepSeek => "DeepSeek",
        LlmProvider::XAI => "xAI (Grok)",
        LlmProvider::Custom => "Свой (OpenAI-совместимый)",
    }
}

fn section_header(ui: &mut egui::Ui, p: &Palette, title: &str) {
    ui.add(egui::Label::new(
        RichText::new(title).strong().color(p.text).size(16.0),
    ));
}

fn field<F: FnOnce(&mut egui::Ui)>(ui: &mut egui::Ui, p: &Palette, label: &str, add: F) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new(
            RichText::new(label).color(p.text_muted).size(13.0),
        ));
        ui.add_space(8.0);
        add(ui);
    });
    ui.add_space(8.0);
}

fn save_button(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    if ui.add(egui::Button::new(
        RichText::new(format!("   {}   ", i18n::t(lang, "settings.save"))).color(p.text).size(13.0),
    )
    .fill(p.accent)
    .rounding(egui::Rounding::same(8.0))
    .min_size(Vec2::new(160.0, 38.0))).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
        // Save API key if a new one was typed
        if !app.api_key_input.is_empty() {
            let _ = app.config.set_api_key(&app.api_key_input);
            app.api_key_input.clear();
        }
        let result = app.config.save(&kebi_core::AppPaths::new());
        app.settings_msg = Some(if result.is_ok() {
            i18n::t(lang, "settings.saved")
        } else {
            i18n::t(lang, "settings.error")
        });
    }
    if let Some(msg) = &app.settings_msg {
        ui.add_space(6.0);
        ui.add(egui::Label::new(
            RichText::new(msg).color(p.accent_2).size(12.0),
        ));
    }
}
