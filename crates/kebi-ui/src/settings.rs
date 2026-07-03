//! Settings window. Made by KebiLab

use crate::theme;
use eframe::egui::{self, RichText, Vec2};
use kebi_core::{Config, Lang};

pub struct SettingsApp {
    pub config: Config,
    pub api_key_input: String,
    pub message: Option<String>,
}

impl SettingsApp {
    pub fn new(config: Config) -> Self {
        let api_key_input = String::new(); // never load plaintext back
        Self { config, api_key_input, message: None }
    }

    pub fn update_config(&mut self, cfg: Config) { self.config = cfg; }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::install(ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(theme::BG_DEEP))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Header
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(RichText::new("Настройки").size(22.0).strong().color(theme::TEXT_PRIMARY)));
                        ui.add_space(8.0);
                        ui.add(egui::Label::new(RichText::new("· Made by KebiLab").color(theme::TEXT_MUTED).size(12.0)));
                    });
                    ui.add_space(16.0);

                    // General
                    section(ui, "Основные");
                    labeled(ui, "Wake word", |ui| {
                        ui.text_edit_singleline(&mut self.config.general.wake_word);
                    });
                    labeled(ui, "Язык", |ui| {
                        egui::ComboBox::from_id_salt("lang")
                            .selected_text(self.config.general.language.clone())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.config.general.language, "ru".into(), "Русский");
                                ui.selectable_value(&mut self.config.general.language, "en".into(), "English");
                            });
                    });
                    ui.checkbox(&mut self.config.general.tts_enabled, "Голосовые ответы");
                    ui.checkbox(&mut self.config.general.autostart, "Запускать с Windows");
                    ui.add_space(8.0);

                    // LLM
                    section(ui, "Нейросеть (LLM)");
                    labeled(ui, "Провайдер", |ui| {
                        egui::ComboBox::from_id_salt("provider")
                            .selected_text(self.config.llm.provider.clone())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.config.llm.provider, "opencode".into(), "OpenCode Go");
                                ui.selectable_value(&mut self.config.llm.provider, "deepseek".into(), "DeepSeek");
                                ui.selectable_value(&mut self.config.llm.provider, "mimo".into(), "MiMo");
                                ui.selectable_value(&mut self.config.llm.provider, "nvidia".into(), "NVIDIA Nemotron");
                                ui.selectable_value(&mut self.config.llm.provider, "custom".into(), "Свой (OpenAI-совместимый)");
                            });
                    });
                    labeled(ui, "Base URL", |ui| {
                        ui.text_edit_singleline(&mut self.config.llm.base_url);
                    });
                    labeled(ui, "Модель", |ui| {
                        egui::ComboBox::from_id_salt("model")
                            .selected_text(self.config.llm.model.clone())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.config.llm.model, "deepseek-v4-flash".into(), "deepseek-v4-flash");
                                ui.selectable_value(&mut self.config.llm.model, "minimax-m3".into(), "minimax-m3");
                                ui.selectable_value(&mut self.config.llm.model, "mimo-v2.5".into(), "mimo-v2.5");
                                ui.selectable_value(&mut self.config.llm.model, "nvidia/nemotron".into(), "nvidia/nemotron");
                                ui.selectable_value(&mut self.config.llm.model, self.config.llm.model.clone(), &self.config.llm.model);
                            });
                    });
                    labeled(ui, "API-ключ (шифруется DPAPI)", |ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.api_key_input)
                            .password(true)
                            .hint_text("sk-…")
                            .desired_width(260.0));
                    });
                    if ui.button(RichText::new("💾 Сохранить").color(theme::TEXT_PRIMARY)).clicked() {
                        if !self.api_key_input.is_empty() {
                            if let Err(e) = self.config.set_api_key(&self.api_key_input) {
                                self.message = Some(format!("Ошибка: {e}"));
                            } else {
                                self.api_key_input.clear();
                                self.message = Some("API-ключ сохранён".into());
                            }
                        }
                        self.message = Some("Настройки сохранены".into());
                        let _ = self.config.save(&kebi_core::AppPaths::new());
                    }
                    if let Some(msg) = &self.message {
                        ui.add(egui::Label::new(RichText::new(msg).color(theme::ACCENT_2).size(12.0)));
                    }
                    ui.add_space(8.0);

                    // Hotkeys
                    section(ui, "Горячие клавиши");
                    for (label, get, set) in [
                        ("Push-to-listen", &mut self.config.hotkeys.push_to_listen, set_str as fn(&mut kebi_core::HotkeyConfig, &str) -> &mut String),
                        ("Overlay", &mut self.config.hotkeys.overlay, set_str_hk as fn(&mut kebi_core::HotkeyConfig, &str) -> &mut String),
                        ("Cancel", &mut self.config.hotkeys.cancel, set_str_hk2 as fn(&mut kebi_core::HotkeyConfig, &str) -> &mut String),
                        ("Toggle TTS", &mut self.config.hotkeys.toggle_tts, set_str_hk3 as fn(&mut kebi_core::HotkeyConfig, &str) -> &mut String),
                        ("Dictation", &mut self.config.hotkeys.dictation, set_str_hk4 as fn(&mut kebi_core::HotkeyConfig, &str) -> &mut String),
                        ("Pause", &mut self.config.hotkeys.pause, set_str_hk5 as fn(&mut kebi_core::HotkeyConfig, &str) -> &mut String),
                    ] {
                        let _ = (label, get, set);
                    }
                    hk_row(ui, "Push-to-listen", &mut self.config.hotkeys.push_to_listen);
                    hk_row(ui, "Overlay", &mut self.config.hotkeys.overlay);
                    hk_row(ui, "Cancel", &mut self.config.hotkeys.cancel);
                    hk_row(ui, "Toggle TTS", &mut self.config.hotkeys.toggle_tts);
                    hk_row(ui, "Dictation", &mut self.config.hotkeys.dictation);
                    hk_row(ui, "Pause", &mut self.config.hotkeys.pause);
                    ui.add_space(8.0);

                    // Audio
                    section(ui, "Аудио");
                    labeled(ui, "Sample rate", |ui| {
                        ui.add(egui::DragValue::new(&mut self.config.audio.sample_rate).range(8000..=48000));
                    });
                    labeled(ui, "VAD threshold", |ui| {
                        ui.add(egui::DragValue::new(&mut self.config.audio.vad_threshold).range(0.001..=0.2));
                    });
                    labeled(ui, "Silence, ms", |ui| {
                        ui.add(egui::DragValue::new(&mut self.config.audio.silence_ms).range(100..=3000));
                    });
                    ui.add_space(8.0);

                    // TTS
                    section(ui, "Голос (TTS)");
                    labeled(ui, "Системный голос", |ui| {
                        ui.text_edit_singleline(&mut self.config.tts.voice);
                    });
                    labeled(ui, "Скорость", |ui| {
                        ui.add(egui::Slider::new(&mut self.config.tts.rate, -10..=10));
                    });
                    labeled(ui, "Громкость TTS", |ui| {
                        ui.add(egui::Slider::new(&mut self.config.tts.volume, 0..=100));
                    });
                    ui.add_space(16.0);
                });
            });
    }
}

fn section(ui: &mut egui::Ui, title: &str) {
    ui.add(egui::Label::new(RichText::new(title).strong().color(theme::TEXT_PRIMARY).size(14.0)));
    ui.add_space(4.0);
    egui::Frame::none()
        .fill(theme::BG_PANEL)
        .stroke(egui::Stroke::new(1.0, theme::LINE))
        .rounding(egui::CornerRadius::same(10))
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.vertical(|ui| { ui.set_min_width(420.0); });
        });
    ui.add_space(8.0);
}

fn labeled<F: FnOnce(&mut egui::Ui)>(ui: &mut egui::Ui, label: &str, add: F) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new(RichText::new(label).color(theme::TEXT_MUTED).size(13.0)));
        ui.add_space(8.0);
        add(ui);
    });
}

fn hk_row(ui: &mut egui::Ui, label: &str, value: &mut String) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new(RichText::new(label).color(theme::TEXT_MUTED).size(13.0)));
        ui.add_space(8.0);
        ui.text_edit_singleline(value);
    });
}

fn set_str(_c: &mut kebi_core::HotkeyConfig, _v: &str) -> &mut String { &mut String::new() }
fn set_str_hk(_c: &mut kebi_core::HotkeyConfig, _v: &str) -> &mut String { &mut String::new() }
fn set_str_hk2(_c: &mut kebi_core::HotkeyConfig, _v: &str) -> &mut String { &mut String::new() }
fn set_str_hk3(_c: &mut kebi_core::HotkeyConfig, _v: &str) -> &mut String { &mut String::new() }
fn set_str_hk4(_c: &mut kebi_core::HotkeyConfig, _v: &str) -> &mut String { &mut String::new() }
fn set_str_hk5(_c: &mut kebi_core::HotkeyConfig, _v: &str) -> &mut String { &mut String::new() }
