//! Main window — voice-first, single screen. Made by KebiLab

use crate::icons::{self, Icon};
use crate::i18n::{self as ui_i18n, Lang};
use crate::theme::{self, Palette, Theme};
use crate::voice::{VoiceController, VoiceEvent, VoiceState, VoiceReceiver};
use eframe::egui::{self, Align, Color32, Layout, Margin, RichText, Vec2};
use kebi_core::Config;
use std::sync::Arc;

pub struct MainApp {
    pub config: Config,
    pub query: String,
    pub status_text: String,
    pub heard_text: String,
    pub show_settings: bool,
    pub show_api_key: bool,
    pub api_key_input: String,
    pub stt_api_key_input: String,
    pub show_stt_api_key: bool,
    pub settings_msg: Option<String>,
    pub voice_state: VoiceState,
    pub controller: Arc<VoiceController>,
    pub voice_rx: VoiceReceiver,
}

impl MainApp {
    pub fn new(config: Config, controller: Arc<VoiceController>, voice_rx: VoiceReceiver) -> Self {
        Self {
            config,
            query: String::new(),
            status_text: "Готов. Нажмите микрофон и говорите.".into(),
            heard_text: String::new(),
            show_settings: false,
            show_api_key: false,
            api_key_input: String::new(),
            stt_api_key_input: String::new(),
            show_stt_api_key: false,
            settings_msg: None,
            voice_state: VoiceState::Idle,
            controller,
            voice_rx,
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain voice events
        while let Ok(ev) = self.voice_rx.try_recv() {
            match ev {
                VoiceEvent::State(s) => { self.voice_state = s; }
                VoiceEvent::Heard(s) => { self.heard_text = s; }
                VoiceEvent::Reply(s) => { self.status_text = s; }
                VoiceEvent::Error(s) => { self.status_text = s; }
            }
        }

        let lang = ui_i18n::Lang::from_code(&self.config.general.language);
        let theme = Theme::from_code(&self.config.ui.theme);
        let p = Palette::get(theme);
        theme::install(ctx, &p);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(p.bg).inner_margin(Margin::same(0.0)))
            .show(ctx, |ui| {
                // Top-right corner controls
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if icon_circle_button(ui, &p, Icon::Settings).clicked() {
                            self.show_settings = !self.show_settings;
                        }
                        ui.add_space(8.0);
                        let theme_label = match theme {
                            Theme::Dark => ui_i18n::t(lang, "theme.dark"),
                            Theme::Light => ui_i18n::t(lang, "theme.light"),
                        };
                        if ui.add(egui::Button::new(
                            RichText::new(theme_label).color(p.text).size(12.0),
                        )
                        .fill(p.surface)
                        .stroke(egui::Stroke::new(1.0, p.line))
                        .rounding(egui::Rounding::same(8.0))).clicked() {
                            self.config.ui.theme = match theme {
                                Theme::Dark => "light".into(),
                                Theme::Light => "dark".into(),
                            };
                        }
                        ui.add_space(6.0);
                        if ui.add(egui::Button::new(
                            RichText::new(lang.label()).color(p.text).size(12.0),
                        )
                        .fill(p.surface)
                        .stroke(egui::Stroke::new(1.0, p.line))
                        .rounding(egui::Rounding::same(8.0))).clicked() {
                            self.config.general.language = match lang {
                                Lang::Ru => "en".into(),
                                Lang::En => "ru".into(),
                            };
                        }
                    });
                });
                ui.add_space(8.0);

                // Logo + title
                ui.vertical_centered(|ui| {
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(72.0, 72.0), egui::Sense::hover());
                    icons::draw(ui, rect, p.accent, 1.0, Icon::Logo);
                    ui.add_space(8.0);
                    ui.add(egui::Label::new(
                        RichText::new(ui_i18n::t(lang, "app.name")).size(24.0).strong().color(p.text),
                    ));
                });
                ui.add_space(20.0);

                // Big microphone button
                ui.vertical_centered(|ui| {
                    let listening = matches!(self.voice_state, VoiceState::Listening);
                    let wake = matches!(self.voice_state, VoiceState::WakeListening);
                    let processing = matches!(self.voice_state, VoiceState::Recognizing | VoiceState::Thinking);
                    let speaking = matches!(self.voice_state, VoiceState::Speaking);
                    let accent_color = if listening { p.danger }
                        else if processing { p.accent }
                        else if speaking { p.success }
                        else if wake { p.accent }
                        else { p.accent };
                    let label = if listening { "■  Остановить".to_string() }
                        else if processing { "…  Обрабатываю".to_string() }
                        else if speaking { "♪  Говорю".to_string() }
                        else if wake { ui_i18n::t(lang, "home.wake_listening") }
                        else { "▶  Говорить".to_string() };
                    let r = ui.add(
                        egui::Button::new(
                            RichText::new(format!("  {}  ", label)).color(p.accent_text).size(16.0),
                        )
                        .fill(accent_color)
                        .rounding(egui::Rounding::same(28.0))
                        .min_size(Vec2::new(220.0, 56.0)),
                    );
                    if r.clicked() {
                        if listening {
                            self.controller.state().set(VoiceState::Idle);
                        } else {
                            self.controller.state().set(VoiceState::Listening);
                        }
                    }
                });
                ui.add_space(16.0);

                // Heard text
                if !self.heard_text.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add(egui::Label::new(
                            RichText::new(format!("«{}»", self.heard_text))
                                .color(p.text_muted).size(14.0).italics(),
                        ));
                    });
                    ui.add_space(12.0);
                }

                // Text input + run button
                ui.vertical_centered(|ui| {
                    let max_w = 520.0;
                    ui.allocate_ui_with_layout(
                        Vec2::new(max_w, 52.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            let resp = ui.add(
                                egui::TextEdit::singleline(&mut self.query)
                                    .hint_text(RichText::new(ui_i18n::t(lang, "home.input_hint")).color(p.text_muted))
                                    .desired_width(max_w - 80.0)
                                    .frame(true)
                                    .margin(Margin::same(14.0))
                                    .font(egui::TextStyle::Body),
                            );
                            if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                if !self.query.trim().is_empty() {
                                    self.status_text = format!("Выполнено: {}", self.query);
                                    self.query.clear();
                                }
                            }
                            ui.add_space(8.0);
                            if ui.add(
                                egui::Button::new(
                                    RichText::new("  ▶  ").color(p.accent_text).size(16.0),
                                )
                                .fill(p.accent)
                                .rounding(egui::Rounding::same(10.0))
                                .min_size(Vec2::new(64.0, 48.0)),
                            ).clicked() && !self.query.is_empty() {
                                self.status_text = format!("Выполнено: {}", self.query);
                                self.query.clear();
                            }
                        },
                    );
                });
                ui.add_space(16.0);

                // Quick actions
                ui.vertical_centered(|ui| {
                    ui.allocate_ui_with_layout(
                        Vec2::new(520.0, 60.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            quick_btn(ui, &p, Icon::Pause, "Пауза");
                            ui.add_space(8.0);
                            quick_btn(ui, &p, Icon::Screenshot, "Скриншот");
                            ui.add_space(8.0);
                            quick_btn(ui, &p, Icon::VolumeDown, "Тише");
                            ui.add_space(8.0);
                            quick_btn(ui, &p, Icon::VolumeUp, "Громче");
                        },
                    );
                });
                ui.add_space(16.0);

                // Status
                ui.vertical_centered(|ui| {
                    let state_color = match self.voice_state {
                        VoiceState::Listening => p.danger,
                        VoiceState::WakeListening => p.success,
                        VoiceState::Recognizing | VoiceState::Thinking => p.accent,
                        VoiceState::Speaking => p.success,
                        VoiceState::Error => p.danger,
                        VoiceState::Idle => p.text_muted,
                    };
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(8.0, 8.0), egui::Sense::hover());
                    icons::draw(ui, rect, state_color, 1.0, Icon::Dot);
                    ui.add_space(6.0);
                    ui.add(egui::Label::new(
                        RichText::new(&self.status_text).color(p.text_muted).size(12.0),
                    ));
                });

                if self.show_settings {
                    render_settings_modal(ctx, self, &p, lang);
                }
            });

        // Trigger repaint while listening so the UI feels live.
        if matches!(self.voice_state, VoiceState::Listening | VoiceState::WakeListening | VoiceState::Recognizing | VoiceState::Thinking | VoiceState::Speaking) {
            ctx.request_repaint_after(std::time::Duration::from_millis(80));
        }
    }
}

use eframe::egui::{Pos2, Rect};

fn quick_btn(ui: &mut egui::Ui, p: &Palette, icon: icons::Icon, label: &str) -> egui::Response {
    let desired = Vec2::new(120.0, 56.0);
    let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
    let bg = if response.hovered() { p.surface_2 } else { p.surface };
    ui.painter().rect_filled(rect, egui::Rounding::same(10.0), bg);
    ui.painter().rect_stroke(rect, egui::Rounding::same(10.0), egui::Stroke::new(1.0, p.line));
    let icon_rect = Rect::from_min_size(Pos2::new(rect.left() + 14.0, rect.center().y - 10.0), Vec2::new(20.0, 20.0));
    let c = if response.hovered() { p.accent } else { p.text };
    icons::draw(ui, icon_rect, c, 1.3, icon);
    ui.painter().text(
        Pos2::new(rect.left() + 40.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(13.0),
        if response.hovered() { p.text } else { p.text_muted },
    );
    response
}

fn icon_circle_button(ui: &mut egui::Ui, p: &Palette, icon: icons::Icon) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(40.0, 40.0), egui::Sense::click());
    let bg = if response.hovered() { p.surface_2 } else { Color32::TRANSPARENT };
    ui.painter().circle_filled(rect.center(), 20.0, bg);
    let icon_rect = Rect::from_min_size(Pos2::new(rect.left() + 12.0, rect.top() + 12.0), Vec2::new(16.0, 16.0));
    let c = if response.hovered() { p.accent } else { p.text_muted };
    icons::draw(ui, icon_rect, c, 1.2, icon);
    response
}

fn render_settings_modal(ctx: &egui::Context, app: &mut MainApp, p: &Palette, lang: Lang) {
    let mut open = true;
    egui::Window::new("Настройки")
        .title_bar(true).resizable(false).collapsible(false)
        .fixed_size(Vec2::new(560.0, 580.0))
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .frame(egui::Frame::none().fill(p.surface).stroke(egui::Stroke::new(1.0, p.line)).rounding(egui::Rounding::same(16.0)).inner_margin(Margin::same(28.0)))
        .open(&mut open)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(egui::Label::new(
                    RichText::new(ui_i18n::t(lang, "settings.title")).size(20.0).strong().color(p.text),
                ));
                ui.add_space(16.0);

                // LLM
                ui.add(egui::Label::new(RichText::new("Нейросеть (LLM)").color(p.text).size(14.0).strong()));
                ui.add_space(8.0);
                section_label(ui, p, ui_i18n::t(lang, "settings.field.provider"));
                let provider = kebi_llm::providers::LlmProvider::from_code(&app.config.llm.provider);
                egui::ComboBox::from_id_source("set_provider")
                    .selected_text(provider_label(provider))
                    .width(500.0)
                    .show_ui(ui, |ui| {
                        for pp in kebi_llm::providers::LlmProvider::all() {
                            ui.selectable_value(&mut app.config.llm.provider, pp.code().into(), provider_label(*pp));
                        }
                    });
                ui.add_space(10.0);
                section_label(ui, p, ui_i18n::t(lang, "settings.field.model"));
                let models = provider.default_models();
                egui::ComboBox::from_id_source("set_model")
                    .selected_text(if app.config.llm.model.is_empty() { "Своя".to_string() } else { app.config.llm.model.clone() })
                    .width(500.0)
                    .show_ui(ui, |ui| {
                        for m in models {
                            ui.selectable_value(&mut app.config.llm.model, m.to_string(), *m);
                        }
                        ui.selectable_value(&mut app.config.llm.model, "".into(), "Своя");
                    });
                if !models.iter().any(|m| *m == app.config.llm.model) {
                    ui.add_space(6.0);
                    ui.add(egui::TextEdit::singleline(&mut app.config.llm.model).hint_text("model-name").desired_width(500.0));
                }
                ui.add_space(10.0);
                section_label(ui, p, ui_i18n::t(lang, "settings.field.apikey"));
                ui.horizontal(|ui| {
                    let mut edit = egui::TextEdit::singleline(&mut app.api_key_input);
                    if !app.show_api_key { edit = edit.password(true); }
                    let key_set = !app.config.llm.api_key_enc.is_empty();
                    ui.add(edit.hint_text(if key_set { "•••••••• (введите чтобы заменить)" } else { "sk-..." }).desired_width(440.0));
                    let eye = if app.show_api_key { Icon::Eye } else { Icon::Close };
                    if icon_circle_button(ui, p, eye).clicked() {
                        app.show_api_key = !app.show_api_key;
                    }
                });

                ui.add_space(18.0);

                // STT
                ui.add(egui::Label::new(RichText::new("Распознавание речи (Whisper)").color(p.text).size(14.0).strong()));
                ui.add_space(8.0);
                section_label(ui, p, "Endpoint".to_string());
                ui.add(egui::TextEdit::singleline(&mut app.config.stt.whisper_endpoint)
                    .hint_text("https://api.openai.com/v1/audio/transcriptions")
                    .desired_width(500.0));
                ui.add_space(8.0);
                section_label(ui, p, "Модель".to_string());
                ui.add(egui::TextEdit::singleline(&mut app.config.stt.whisper_model)
                    .hint_text("whisper-1").desired_width(500.0));
                ui.add_space(8.0);
                section_label(ui, p, "Язык (ru/en)".to_string());
                ui.add(egui::TextEdit::singleline(&mut app.config.stt.whisper_language)
                    .hint_text("ru").desired_width(500.0));
                ui.add_space(8.0);
                section_label(ui, p, "API-ключ (Whisper)".to_string());
                ui.horizontal(|ui| {
                    let mut edit = egui::TextEdit::singleline(&mut app.stt_api_key_input);
                    if !app.show_stt_api_key { edit = edit.password(true); }
                    let stt_key_set = !app.config.stt.api_key_enc.is_empty();
                    ui.add(edit.hint_text(if stt_key_set { "•••••••• (введите чтобы заменить)" } else { "sk-..." }).desired_width(440.0));
                    let eye = if app.show_stt_api_key { Icon::Eye } else { Icon::Close };
                    if icon_circle_button(ui, p, eye).clicked() {
                        app.show_stt_api_key = !app.show_stt_api_key;
                    }
                });

                ui.add_space(18.0);

                // Wake word
                ui.add(egui::Label::new(RichText::new(ui_i18n::t(lang, "settings.wake_word_section")).color(p.text).size(14.0).strong()));
                ui.add_space(8.0);
                section_label(ui, p, ui_i18n::t(lang, "settings.wake_word_toggle"));
                ui.add(egui::Checkbox::new(&mut app.config.general.wake_word_enabled, ""));
                ui.add_space(6.0);
                section_label(ui, p, ui_i18n::t(lang, "settings.wake_word_phrase"));
                ui.add(egui::TextEdit::singleline(&mut app.config.general.wake_word)
                    .hint_text("кеби").desired_width(500.0));
                if app.config.general.wake_word_enabled && app.config.get_stt_api_key().is_none() {
                    ui.add_space(4.0);
                    ui.add(egui::Label::new(
                        RichText::new(ui_i18n::t(lang, "settings.wake_word_hint"))
                            .color(p.danger).size(11.0),
                    ));
                }

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.add(egui::Button::new(
                        RichText::new(format!("   {}   ", ui_i18n::t(lang, "settings.save"))).color(p.accent_text).size(14.0),
                    )
                    .fill(p.accent)
                    .rounding(egui::Rounding::same(10.0))
                    .min_size(Vec2::new(200.0, 44.0))).clicked() {
                        if !app.api_key_input.is_empty() {
                            let _ = app.config.set_api_key(&app.api_key_input);
                            app.api_key_input.clear();
                        }
                        if !app.stt_api_key_input.is_empty() {
                            let _ = app.config.set_stt_api_key(&app.stt_api_key_input);
                            app.stt_api_key_input.clear();
                        }
                        let result = app.config.save(&kebi_core::AppPaths::new());
                        app.settings_msg = Some(if result.is_ok() { "Сохранено".into() } else { "Ошибка".into() });
                        app.show_settings = false;
                    }
                    ui.add_space(8.0);
                    if ui.add(egui::Button::new(
                        RichText::new(format!("   {}   ", ui_i18n::t(lang, "settings.cancel"))).color(p.text).size(14.0),
                    )
                    .fill(p.surface_2)
                    .rounding(egui::Rounding::same(10.0))
                    .min_size(Vec2::new(140.0, 44.0))).clicked() {
                        app.show_settings = false;
                    }
                });
            });
        });
    if !open { app.show_settings = false; }
}

fn section_label(ui: &mut egui::Ui, p: &Palette, text: String) {
    ui.add(egui::Label::new(RichText::new(text).color(p.text_muted).size(12.0)));
}

fn provider_label(p: kebi_llm::providers::LlmProvider) -> &'static str {
    use kebi_llm::providers::LlmProvider::*;
    match p {
        OpenCode => "OpenCode Go",
        OpenAI => "OpenAI",
        Anthropic => "Anthropic (Claude)",
        Google => "Google Gemini",
        Mistral => "Mistral AI",
        Groq => "Groq",
        DeepSeek => "DeepSeek",
        XAI => "xAI (Grok)",
        Custom => "Свой",
    }
}
