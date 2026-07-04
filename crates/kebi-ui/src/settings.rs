//! Settings window. Made by KebiLab

use crate::icons::{self, Icon};
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
                    // Header
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(20.0, 20.0), egui::Sense::hover());
                        icons::draw(ui, rect, theme::ACCENT, 1.4, Icon::Settings);
                        ui.add_space(8.0);
                        ui.add(egui::Label::new(
                            RichText::new("Настройки").size(22.0).strong().color(theme::TEXT_PRIMARY),
                        ));
                    });
                    ui.add(egui::Label::new(
                        RichText::new("Made by KebiLab").color(theme::TEXT_MUTED).size(12.0),
                    ));
                    ui.add_space(16.0);

                    // General
                    section_header(ui, "Основные");
                    field(ui, "Wake word", |ui| {
                        ui.text_edit_singleline(&mut self.config.general.wake_word);
                    });
                    field(ui, "Язык", |ui| {
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

                    // LLM
                    section_header(ui, "Нейросеть");
                    field(ui, "Провайдер", |ui| {
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
                    field(ui, "Base URL", |ui| {
                        ui.text_edit_singleline(&mut self.config.llm.base_url);
                    });
                    field(ui, "Модель", |ui| {
                        ui.text_edit_singleline(&mut self.config.llm.model);
                    });
                    field(ui, "API-ключ", |ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.api_key_input)
                            .password(true)
                            .hint_text("sk-…")
                            .desired_width(260.0));
                    });
                    ui.add_space(6.0);
                    let save_btn = egui::Button::new(
                        RichText::new("  Сохранить").color(theme::TEXT_PRIMARY).size(13.0),
                    )
                    .fill(theme::ACCENT)
                    .rounding(egui::Rounding::same(8.0));
                    let save_resp = ui.add(save_btn).on_hover_cursor(egui::CursorIcon::PointingHand);
                    if save_resp.hovered() {
                        let r = save_resp.rect;
                        icons::draw(ui, egui::Rect::from_min_size(
                            egui::Pos2::new(r.left() + 14.0, r.center().y - 7.0),
                            egui::Vec2::new(14.0, 14.0),
                        ), theme::TEXT_PRIMARY, 1.3, Icon::Save);
                    }
                    if save_resp.clicked() {
                        if !self.api_key_input.is_empty() {
                            if let Err(e) = self.config.set_api_key(&self.api_key_input) {
                                self.message = Some(format!("Ошибка: {e}"));
                            } else {
                                self.api_key_input.clear();
                            }
                        }
                        self.message = Some("Настройки сохранены".into());
                        let _ = self.config.save(&kebi_core::AppPaths::new());
                    }
                    if let Some(msg) = &self.message {
                        ui.add_space(6.0);
                        ui.add(egui::Label::new(
                            RichText::new(msg).color(theme::ACCENT_2).size(12.0),
                        ));
                    }
                });
            });
    }
}

fn section_header(ui: &mut egui::Ui, title: &str) {
    ui.add(egui::Label::new(
        RichText::new(title).strong().color(theme::TEXT_PRIMARY).size(14.0),
    ));
    ui.add_space(6.0);
}

fn field<F: FnOnce(&mut egui::Ui)>(ui: &mut egui::Ui, label: &str, add: F) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new(
            RichText::new(label).color(theme::TEXT_MUTED).size(13.0),
        ));
        ui.add_space(8.0);
        add(ui);
    });
    ui.add_space(4.0);
}
