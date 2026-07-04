//! Main window — static desktop app. Made by KebiLab

use crate::icons::{self, Icon};
use crate::i18n::{self, Lang};
use crate::theme::{self, Palette, Theme};
use eframe::egui::{self, Align, Layout, Margin, RichText, Vec2};
use kebi_core::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    History,
    Hotkeys,
    Settings,
    About,
}

pub struct MainApp {
    pub config: Config,
    pub page: Page,
    pub status: Status,
    pub query: String,
    pub history: Vec<HistoryEntry>,
    pub settings_msg: Option<String>,
    pub api_key_input: String,
    pub show_api_key: bool,
    pub settings_section: SettingsSection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsSection {
    General,
    Appearance,
    Llm,
    Audio,
    Hotkeys,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status { Idle, Listening, Thinking, Error }

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub text: String,
    pub at: chrono::DateTime<chrono::Local>,
    pub ok: bool,
}

impl MainApp {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page: Page::Home,
            status: Status::Idle,
            query: String::new(),
            settings_msg: None,
            api_key_input: String::new(),
            show_api_key: false,
            settings_section: SettingsSection::General,
            history: vec![
                HistoryEntry { text: "кеби хелп".into(), at: chrono::Local::now() - chrono::Duration::minutes(2), ok: true },
                HistoryEntry { text: "громкость на 40".into(), at: chrono::Local::now() - chrono::Duration::minutes(5), ok: true },
                HistoryEntry { text: "открой ютуб".into(), at: chrono::Local::now() - chrono::Duration::minutes(11), ok: true },
            ],
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let lang = i18n::Lang::from_code(&self.config.general.language);
        let theme = Theme::from_code(&self.config.ui.theme);
        let p = Palette::get(theme);
        theme::install(ctx, theme);

        // ===== LEFT SIDEBAR =====
        let mut target: Option<Page> = None;
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(220.0)
            .frame(egui::Frame::none()
                .fill(p.nav_bg)
                .stroke(egui::Stroke::new(1.0, p.line))
                .inner_margin(Margin::same(20.0)))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(4.0);
                    // Brand block
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(Vec2::new(32.0, 32.0), egui::Sense::hover());
                        icons::draw(ui, rect, p.accent, 1.0, Icon::Logo);
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            ui.add(egui::Label::new(
                                RichText::new(i18n::t(lang, "app.name"))
                                    .strong().size(16.0).color(p.text),
                            ));
                            ui.add(egui::Label::new(
                                RichText::new(i18n::t(lang, "app.tagline"))
                                    .size(10.0).color(p.text_muted),
                            ));
                        });
                    });
                    ui.add_space(28.0);

                    nav_item(ui, &p, lang, Icon::Home, Page::Home, self.page, &mut target);
                    nav_item(ui, &p, lang, Icon::History, Page::History, self.page, &mut target);
                    nav_item(ui, &p, lang, Icon::Key, Page::Hotkeys, self.page, &mut target);
                    nav_item(ui, &p, lang, Icon::Settings, Page::Settings, self.page, &mut target);
                    nav_item(ui, &p, lang, Icon::Info, Page::About, self.page, &mut target);
                });
                // Footer
                ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                    ui.add_space(4.0);
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), egui::Sense::hover());
                    let c = match self.status {
                        Status::Listening => p.accent_2,
                        Status::Thinking => p.accent,
                        Status::Error => p.danger,
                        Status::Idle => p.text_muted,
                    };
                    icons::draw(ui, rect, c, 1.0, Icon::Dot);
                    ui.add_space(6.0);
                    ui.add(egui::Label::new(
                        RichText::new(i18n::t(lang, "app.by")).color(p.text_muted).size(10.0),
                    ));
                });
            });
        if let Some(t) = target { self.page = t; }

        // ===== TOP BAR =====
        egui::TopBottomPanel::top("topbar")
            .resizable(false)
            .exact_height(56.0)
            .frame(egui::Frame::none()
                .fill(p.bg_panel)
                .stroke(egui::Stroke::new(1.0, p.line))
                .inner_margin(Margin::symmetric(24.0, 12.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let page_name = match self.page {
                        Page::Home => i18n::t(lang, "nav.home"),
                        Page::History => i18n::t(lang, "nav.history"),
                        Page::Settings => i18n::t(lang, "nav.settings"),
                        Page::Hotkeys => i18n::t(lang, "nav.hotkeys"),
                        Page::About => i18n::t(lang, "nav.about"),
                    };
                    ui.add(egui::Label::new(
                        RichText::new(page_name).strong().size(16.0).color(p.text),
                    ));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        egui::ComboBox::from_id_source("topbar_lang")
                            .selected_text(lang.label())
                            .width(110.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.config.general.language, "ru".into(), "Русский");
                                ui.selectable_value(&mut self.config.general.language, "en".into(), "English");
                            });
                        ui.add_space(8.0);
                        egui::ComboBox::from_id_source("topbar_theme")
                            .selected_text(i18n::t(lang, match Theme::from_code(&self.config.ui.theme) {
                                Theme::Midnight => "theme.midnight",
                                Theme::Dawn => "theme.dawn",
                                Theme::Forest => "theme.forest",
                            }))
                            .width(120.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.config.ui.theme, "midnight".into(), i18n::t(lang, "theme.midnight"));
                                ui.selectable_value(&mut self.config.ui.theme, "dawn".into(), i18n::t(lang, "theme.dawn"));
                                ui.selectable_value(&mut self.config.ui.theme, "forest".into(), i18n::t(lang, "theme.forest"));
                            });
                    });
                });
            });

        // ===== CONTENT =====
        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(p.bg_panel)
                .inner_margin(Margin::same(28.0)))
            .show(ctx, |ui| {
                match self.page {
                    Page::Home => page_home(ui, &p, lang, self),
                    Page::History => page_history(ui, &p, lang, self),
                    Page::Hotkeys => page_hotkeys(ui, &p, lang),
                    Page::Settings => crate::settings_view::page_settings(ui, &p, lang, self),
                    Page::About => page_about(ui, &p, lang),
                }
            });
    }
}

fn nav_item(ui: &mut egui::Ui, p: &Palette, lang: Lang, icon: Icon, target: Page, current: Page, out: &mut Option<Page>) {
    let active = current == target;
    let label = match target {
        Page::Home => i18n::t(lang, "nav.home"),
        Page::History => i18n::t(lang, "nav.history"),
        Page::Settings => i18n::t(lang, "nav.settings"),
        Page::Hotkeys => i18n::t(lang, "nav.hotkeys"),
        Page::About => i18n::t(lang, "nav.about"),
    };

    // Allocate the row. Icon is drawn in a fixed 24px column on the left.
    let row_h = 40.0;
    let (rect, response) = ui.allocate_exact_size(Vec2::new(180.0, row_h), egui::Sense::click());

    // Background
    let bg = if active { p.nav_active_bg } else if response.hovered() { p.nav_hover_bg } else { Color32::TRANSPARENT };
    let rounding = egui::Rounding::same(10.0);
    ui.painter().rect_filled(rect, rounding, bg);

    // Active accent bar on the left
    if active {
        let bar = Rect::from_min_size(Pos2::new(rect.left() - 20.0, rect.top() + 6.0), Vec2::new(3.0, rect.height() - 12.0));
        ui.painter().rect_filled(bar, egui::Rounding::same(2.0), p.nav_active_bar);
    }

    // Icon column
    let icon_rect = Rect::from_min_size(Pos2::new(rect.left() + 14.0, rect.center().y - 8.0), Vec2::new(16.0, 16.0));
    let icon_color = if active { p.accent } else if response.hovered() { p.text } else { p.text_muted };
    icons::draw(ui, icon_rect, icon_color, 1.2, icon);

    // Label
    let text_color = if active { p.text } else if response.hovered() { p.text } else { p.text_muted };
    ui.painter().text(
        Pos2::new(rect.left() + 40.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(13.0),
        text_color,
    );

    if response.clicked() { *out = Some(target); }
}

use eframe::egui::{Color32, Pos2, Rect};

fn page_home(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    // Hero header
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.add(egui::Label::new(
                RichText::new(format!("{}, {}", i18n::t(lang, "home.greeting"), who(lang)))
                    .size(28.0).strong().color(p.text),
            ));
            ui.add_space(4.0);
            ui.add(egui::Label::new(
                RichText::new(i18n::t(lang, "home.subtitle"))
                    .size(13.0).color(p.text_muted),
            ));
        });
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let status_text = match app.status {
                Status::Idle => i18n::t(lang, "status.idle"),
                Status::Listening => i18n::t(lang, "status.listening"),
                Status::Thinking => i18n::t(lang, "status.thinking"),
                Status::Error => i18n::t(lang, "status.error"),
            };
            let c = match app.status {
                Status::Listening => p.accent_2,
                Status::Thinking => p.accent,
                Status::Error => p.danger,
                Status::Idle => p.text_muted,
            };
            egui::Frame::none()
                .fill(p.bg_raised)
                .stroke(egui::Stroke::new(1.0, p.line))
                .rounding(egui::Rounding::same(20.0))
                .inner_margin(Margin::symmetric(14.0, 6.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(Vec2::new(10.0, 10.0), egui::Sense::hover());
                        icons::draw(ui, rect, c, 1.0, Icon::Dot);
                        ui.add_space(6.0);
                        ui.add(egui::Label::new(
                            RichText::new(status_text).color(p.text).size(12.0),
                        ));
                    });
                });
        });
    });
    ui.add_space(24.0);

    // Input with embedded Run button
    let mut submitted = false;
    ui.horizontal(|ui| {
        let resp = ui.add(
            egui::TextEdit::singleline(&mut app.query)
                .hint_text(RichText::new(i18n::t(lang, "home.input_hint")).color(p.text_muted))
                .desired_width(f32::INFINITY)
                .frame(true)
                .margin(Margin::same(14.0))
                .font(egui::TextStyle::Body),
        );
        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            submitted = true;
        }
    });
    ui.add_space(10.0);
    if ui.add(egui::Button::new(
        RichText::new(format!("   {}   ", i18n::t(lang, "home.run"))).color(p.text).size(13.0),
    )
    .fill(p.accent)
    .rounding(egui::Rounding::same(10.0))
    .min_size(Vec2::new(160.0, 40.0))).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
        submitted = true;
    }
    if submitted && !app.query.trim().is_empty() {
        app.history.insert(0, HistoryEntry {
            text: app.query.clone(),
            at: chrono::Local::now(),
            ok: true,
        });
        app.status = Status::Thinking;
        app.query.clear();
    }

    ui.add_space(28.0);

    // Quick actions grid
    section_title(ui, p, &i18n::t(lang, "home.quick"));
    ui.add_space(10.0);
    let actions = [
        (Icon::Pause, "Пауза", "Pause"),
        (Icon::Play, "Играй", "Play"),
        (Icon::Screenshot, "Скриншот", "Screenshot"),
        (Icon::VolumeDown, "Тише", "Quieter"),
        (Icon::VolumeUp, "Громче", "Louder"),
        (Icon::Mic, "Микрофон", "Microphone"),
    ];
    ui.horizontal(|ui| {
        for (icon, ru, en) in &actions {
            let label = if lang == Lang::Ru { *ru } else { *en };
            let _ = quick_card(ui, p, *icon, label);
        }
    });
    ui.add_space(28.0);

    // Recent
    section_title(ui, p, &i18n::t(lang, "home.recent"));
    ui.add_space(10.0);
    if app.history.is_empty() {
        ui.add(egui::Label::new(
            RichText::new(i18n::t(lang, "home.empty")).italics().color(p.text_muted).size(12.0),
        ));
    } else {
        for h in app.history.iter().take(6) {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(10.0, 10.0), egui::Sense::hover());
                let c = if h.ok { p.success } else { p.danger };
                icons::draw(ui, rect, c, 1.0, Icon::Dot);
                ui.add_space(10.0);
                ui.add(egui::Label::new(
                    RichText::new(&h.text).color(p.text).size(13.0),
                ));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add(egui::Label::new(
                        RichText::new(h.at.format("%H:%M").to_string()).color(p.text_muted).size(11.0),
                    ));
                });
            });
            ui.add_space(6.0);
        }
    }
}

fn quick_card(ui: &mut egui::Ui, p: &Palette, icon: icons::Icon, label: &str) -> egui::Response {
    let desired = Vec2::new(140.0, 88.0);
    let resp = ui.allocate_exact_size(desired, egui::Sense::click());
    let r = resp.0;
    let bg = if resp.1.hovered() { p.nav_hover_bg } else { p.bg_raised };
    ui.painter().rect_filled(r, egui::Rounding::same(12.0), bg);
    ui.painter().rect_stroke(r, egui::Rounding::same(12.0), egui::Stroke::new(1.0, p.line));

    // Icon at top-center
    let icon_rect = Rect::from_min_size(Pos2::new(r.center().x - 12.0, r.top() + 16.0), Vec2::new(24.0, 24.0));
    let c = if resp.1.hovered() { p.accent } else { p.text };
    icons::draw(ui, icon_rect, c, 1.4, icon);

    // Label at bottom
    ui.painter().text(
        Pos2::new(r.center().x, r.bottom() - 12.0),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(12.0),
        if resp.1.hovered() { p.text } else { p.text_muted },
    );
    resp.1
}

fn section_title(ui: &mut egui::Ui, p: &Palette, title: &str) {
    ui.add(egui::Label::new(
        RichText::new(title).strong().color(p.text).size(13.0),
    ));
}

fn page_history(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new(
            RichText::new(i18n::t(lang, "history.title")).size(24.0).strong().color(p.text),
        ));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.add(egui::Button::new(
                RichText::new(format!("   {}", i18n::t(lang, "history.clear"))).color(p.text).size(12.0),
            )
            .fill(p.bg_raised)
            .rounding(egui::Rounding::same(8.0))).clicked() {
                app.history.clear();
            }
        });
    });
    ui.add_space(20.0);
    if app.history.is_empty() {
        ui.add(egui::Label::new(
            RichText::new(i18n::t(lang, "history.empty")).italics().color(p.text_muted).size(12.0),
        ));
    } else {
        for h in &app.history {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(10.0, 10.0), egui::Sense::hover());
                let c = if h.ok { p.success } else { p.danger };
                icons::draw(ui, rect, c, 1.0, Icon::Dot);
                ui.add_space(10.0);
                ui.add(egui::Label::new(RichText::new(&h.text).color(p.text).size(13.0)));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add(egui::Label::new(
                        RichText::new(h.at.format("%Y-%m-%d %H:%M").to_string()).color(p.text_muted).size(11.0),
                    ));
                });
            });
            ui.add_space(6.0);
        }
    }
}

fn page_hotkeys(ui: &mut egui::Ui, p: &Palette, lang: Lang) {
    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "hotkeys.title")).size(24.0).strong().color(p.text),
    ));
    ui.add_space(20.0);
    let hk = [
        ("Ctrl+Shift+Space", "hotkeys.listen"),
        ("Esc", "hotkeys.cancel"),
        ("Ctrl+Shift+M", "hotkeys.tts"),
        ("Ctrl+Shift+D", "hotkeys.dictation"),
        ("Ctrl+Shift+P", "hotkeys.pause"),
    ];
    for (k, label_key) in hk {
        ui.horizontal(|ui| {
            ui.add(egui::Label::new(
                RichText::new(k).color(p.accent).size(13.0),
            ));
            ui.add_space(20.0);
            ui.add(egui::Label::new(
                RichText::new(i18n::t(lang, label_key)).color(p.text_muted).size(13.0),
            ));
        });
        ui.add_space(6.0);
    }
}

fn page_about(ui: &mut egui::Ui, p: &Palette, lang: Lang) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(72.0, 72.0), egui::Sense::hover());
        icons::draw(ui, rect, p.accent, 1.0, Icon::Logo);
        ui.add_space(20.0);
        ui.vertical(|ui| {
            ui.add(egui::Label::new(
                RichText::new(i18n::t(lang, "app.name")).size(28.0).strong().color(p.text),
            ));
            ui.add(egui::Label::new(
                RichText::new(format!("{} 0.1.0", i18n::t(lang, "about.version"))).color(p.text_muted).size(12.0),
            ));
        });
    });
    ui.add_space(20.0);
    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "about.desc")).color(p.text).size(13.0),
    ));
    ui.add_space(16.0);
    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "app.by")).strong().color(p.text).size(13.0),
    ));
}

fn who(lang: Lang) -> &'static str {
    match lang { Lang::Ru => "друг", Lang::En => "there" }
}
