//! Settings window. Made by KebiLab

use crate::theme;
use eframe::egui::{self, RichText};

pub struct SettingsApp {
    pub config: kebi_core::Config,
    pub api_key_input: String,
    pub message: Option<String>,
}

impl SettingsApp {
    pub fn new(config: kebi_core::Config) -> Self {
        Self { config, api_key_input: String::new(), message: None }
    }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::install(ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(theme::BG_DEEP))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(egui::Label::new(
                        RichText::new("Настройки").size(22.0).strong().color(theme::TEXT_PRIMARY),
                    ));
                    ui.add(egui::Label::new(
                        RichText::new("Made by KebiLab").color(theme::TEXT_MUTED).size(12.0),
                    ));
                    ui.add_space(16.0);

                    ui.add(egui::Label::new(RichText::new("Основные").strong().color(theme::TEXT_PRIMARY).size(14.0)));
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(RichText::new("Wake word").color(theme::TEXT_MUTED).size(13.0)));
                        ui.add_space(8.0);
                        ui.text_edit_singleline(&mut self.config.general.wake_word);
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(RichText::new("Язык").color(theme::TEXT_MUTED).size(13.0)));
                        ui.add_space(8.0);
                        egui::ComboBox::from_id_source("lang")
                            .selected_text(self.config.general.language.clone())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.config.general.language, "ru".into(), "Русский");
                                ui.selectable_value(&mut self.config.general.language, "en".into(), "English");
                            });
                    });
                    ui.checkbox(&mut self.config.general.tts_enabled, "Голосовые ответы");
                    ui.checkbox(&mut self.config.general.autostart, "Запускать с Windows");
                    ui.add_space(12.0);

                    ui.add(egui::Label::new(RichText::new("Нейросеть").strong().color(theme::TEXT_PRIMARY).size(14.0)));
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(RichText::new("Провайдер").color(theme::TEXT_MUTED).size(13.0)));
                        ui.add_space(8.0);
                        egui::ComboBox::from_id_source("provider")
                            .selected_text(self.config.llm.provider.clone())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.config.llm.provider, "opencode".into(), "OpenCode Go");
                                ui.selectable_value(&mut self.config.llm.provider, "deepseek".into(), "DeepSeek");
                                ui.selectable_value(&mut self.config.llm.provider, "mimo".into(), "MiMo");
                                ui.selectable_value(&mut self.config.llm.provider, "nvidia".into(), "NVIDIA");
                                ui.selectable_value(&mut self.config.llm.provider, "custom".into(), "Свой");
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(RichText::new("Base URL").color(theme::TEXT_MUTED).size(13.0)));
                        ui.add_space(8.0);
                        ui.text_edit_singleline(&mut self.config.llm.base_url);
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(RichText::new("Модель").color(theme::TEXT_MUTED).size(13.0)));
                        ui.add_space(8.0);
                        ui.text_edit_singleline(&mut self.config.llm.model);
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(RichText::new("API-ключ").color(theme::TEXT_MUTED).size(13.0)));
                        ui.add_space(8.0);
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
                });
            });
    }
}
