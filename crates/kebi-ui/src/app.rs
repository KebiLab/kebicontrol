//! Main window — static desktop app. Made by KebiLab

use crate::icons::{self, Icon};
use crate::theme::{self, Palette, Theme};
use crate::i18n::{self, Lang};
use eframe::egui::{self, Align, Layout, RichText, Vec2};
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

        let nav_target = nav_panel(ctx, &p, lang, self.page);
        if let Some(t) = nav_target { self.page = t; }

        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(p.bg_panel)
                .inner_margin(egui::Margin::same(28.0)))
            .show(ctx, |ui| {
                top_bar(ui, &p, lang, &mut self.config, self.page);
                ui.add_space(8.0);

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

fn nav_panel(ctx: &egui::Context, p: &Palette, lang: Lang, current: Page) -> Option<Page> {
    let mut target: Option<Page> = None;
    egui::SidePanel::left("nav")
        .resizable(false)
        .exact_width(208.0)
        .frame(egui::Frame::none()
            .fill(p.bg_deep)
            .stroke(egui::Stroke::new(1.0, p.line))
            .inner_margin(egui::Margin::same(18.0)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(22.0, 22.0), egui::Sense::hover());
                icons::draw(ui, rect, p.accent, 1.4, Icon::Mic);
                ui.add_space(8.0);
                ui.add(egui::Label::new(
                    RichText::new(i18n::t(lang, "app.name")).strong().size(16.0).color(p.text),
                ));
            });
            ui.add(egui::Label::new(
                RichText::new(i18n::t(lang, "app.tagline")).size(10.0).color(p.text_muted),
            ));
            ui.add_space(24.0);

            nav_item(ui, p, lang, Icon::Home, Page::Home, current, &mut target);
            nav_item(ui, p, lang, Icon::History, Page::History, current, &mut target);
            nav_item(ui, p, lang, Icon::Key, Page::Hotkeys, current, &mut target);
            nav_item(ui, p, lang, Icon::Settings, Page::Settings, current, &mut target);
            nav_item(ui, p, lang, Icon::Info, Page::About, current, &mut target);

            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                ui.add_space(8.0);
                let dot_color = match current {
                    Page::Home => p.accent_2,
                    Page::History => p.accent,
                    Page::Settings => p.warn,
                    Page::Hotkeys => p.accent,
                    Page::About => p.text_muted,
                };
                let (rect, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), egui::Sense::hover());
                icons::draw(ui, rect, dot_color, 1.2, Icon::Dot);
                ui.add_space(6.0);
                let page_name = match current {
                    Page::Home => i18n::t(lang, "nav.home"),
                    Page::History => i18n::t(lang, "nav.history"),
                    Page::Settings => i18n::t(lang, "nav.settings"),
                    Page::Hotkeys => i18n::t(lang, "nav.hotkeys"),
                    Page::About => i18n::t(lang, "nav.about"),
                };
                ui.add(egui::Label::new(RichText::new(page_name).color(p.text).size(12.0)));
                ui.add_space(4.0);
                ui.add(egui::Label::new(
                    RichText::new(i18n::t(lang, "app.by")).color(p.text_muted).size(10.0),
                ));
            });
        });
    target
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
    let fill = if active { p.nav_active_bg } else { egui::Color32::TRANSPARENT };
    let stroke = if active { egui::Stroke::new(1.0, p.accent) } else { egui::Stroke::NONE };
    let text_color = if active { p.text } else { p.text_muted };

    let resp = ui.add(
        egui::Button::new(
            RichText::new(format!("  {label}")).color(text_color).size(13.0),
        )
        .fill(fill)
        .stroke(stroke)
        .rounding(egui::Rounding::same(8.0))
        .min_size(Vec2::new(172.0, 38.0)),
    );
    let r = resp.rect;
    let icon_color = if active { p.accent } else { p.text_muted };
    let icon_rect = egui::Rect::from_min_size(
        egui::Pos2::new(r.left() + 14.0, r.center().y - 7.0),
        Vec2::new(14.0, 14.0),
    );
    icons::draw(ui, icon_rect, icon_color, 1.2, icon);
    if resp.clicked() { *out = Some(target); }
}

fn top_bar(ui: &mut egui::Ui, p: &Palette, lang: Lang, config: &mut Config, _page: Page) {
    ui.horizontal(|ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            egui::ComboBox::from_id_source("topbar_lang")
                .selected_text(lang.label())
                .width(110.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut config.general.language, "ru".into(), "Русский");
                    ui.selectable_value(&mut config.general.language, "en".into(), "English");
                });
            ui.add_space(8.0);
            egui::ComboBox::from_id_source("topbar_theme")
                .selected_text(i18n::t(lang, match Theme::from_code(&config.ui.theme) {
                    Theme::Midnight => "theme.midnight",
                    Theme::Dawn => "theme.dawn",
                    Theme::Forest => "theme.forest",
                }))
                .width(120.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut config.ui.theme, "midnight".into(), i18n::t(lang, "theme.midnight"));
                    ui.selectable_value(&mut config.ui.theme, "dawn".into(), i18n::t(lang, "theme.dawn"));
                    ui.selectable_value(&mut config.ui.theme, "forest".into(), i18n::t(lang, "theme.forest"));
                });
        });
    });
}

fn page_home(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    ui.add(egui::Label::new(
        RichText::new(format!("{}, {}", i18n::t(lang, "home.greeting"), who(lang)))
            .size(28.0).strong().color(p.text),
    ));
    ui.add_space(4.0);
    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "home.subtitle")).size(13.0).color(p.text_muted),
    ));
    ui.add_space(20.0);

    let mut submitted = false;
    let resp = ui.add(
        egui::TextEdit::singleline(&mut app.query)
            .hint_text(RichText::new(i18n::t(lang, "home.input_hint")).color(p.text_muted))
            .desired_width(f32::INFINITY)
            .frame(true)
            .margin(egui::Margin::same(14.0))
            .font(egui::TextStyle::Body),
    );
    if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        submitted = true;
    }
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let btn = egui::Button::new(
            RichText::new(format!("  {}", i18n::t(lang, "home.run"))).color(p.text).size(13.0),
        )
        .fill(p.accent)
        .rounding(egui::Rounding::same(8.0))
        .min_size(Vec2::new(140.0, 40.0));
        let btn_resp = ui.add(btn).on_hover_cursor(egui::CursorIcon::PointingHand);
        icons::draw(ui, egui::Rect::from_min_size(
            egui::Pos2::new(btn_resp.rect.left() + 18.0, btn_resp.rect.center().y - 7.0),
            Vec2::new(14.0, 14.0),
        ), p.text, 1.3, Icon::Play);
        if btn_resp.clicked() { submitted = true; }
    });
    if submitted && !app.query.trim().is_empty() {
        app.history.insert(0, HistoryEntry {
            text: app.query.clone(),
            at: chrono::Local::now(),
            ok: true,
        });
        app.status = Status::Thinking;
        app.query.clear();
    }
    ui.add_space(20.0);

    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "home.quick")).strong().color(p.text).size(13.0),
    ));
    ui.add_space(8.0);
    let actions = [
        (Icon::Pause, "Пауза", "Pause"),
        (Icon::Play, "Играй", "Play"),
        (Icon::Screenshot, "Скриншот", "Screenshot"),
        (Icon::VolumeDown, "Тише", "Quieter"),
        (Icon::VolumeUp, "Громче", "Louder"),
        (Icon::Mic, "Микрофон", "Microphone"),
    ];
    egui::Grid::new("quick_grid")
        .spacing(Vec2::new(10.0, 10.0))
        .min_col_width(140.0)
        .show(ui, |ui| {
            for chunk in actions.chunks(3) {
                ui.horizontal(|ui| {
                    for (icon, ru, en) in chunk {
                        let label = if lang == Lang::Ru { ru } else { en };
                        let r = ui.add(
                            egui::Button::new(
                                RichText::new(format!("  {label}")).color(p.text).size(13.0),
                            )
                            .fill(p.bg_raised)
                            .stroke(egui::Stroke::new(1.0, p.line))
                            .rounding(egui::Rounding::same(10.0))
                            .min_size(Vec2::new(140.0, 48.0)),
                        );
                        icons::draw(ui, egui::Rect::from_min_size(
                            egui::Pos2::new(r.rect.left() + 14.0, r.rect.center().y - 7.0),
                            Vec2::new(14.0, 14.0),
                        ), p.text, 1.2, *icon);
                    }
                });
                ui.end_row();
            }
        });

    ui.add_space(20.0);

    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "home.recent")).strong().color(p.text).size(13.0),
    ));
    ui.add_space(8.0);
    if app.history.is_empty() {
        ui.add(egui::Label::new(
            RichText::new(i18n::t(lang, "home.empty")).italics().color(p.text_muted).size(12.0),
        ));
    } else {
        for h in app.history.iter().take(5) {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(12.0, 12.0), egui::Sense::hover());
                let color = if h.ok { p.success } else { p.danger };
                icons::draw(ui, rect, color, 1.0, Icon::Dot);
                ui.add_space(8.0);
                ui.add(egui::Label::new(
                    RichText::new(&h.text).color(p.text).size(13.0),
                ));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add(egui::Label::new(
                        RichText::new(h.at.format("%H:%M").to_string()).color(p.text_muted).size(11.0),
                    ));
                });
            });
            ui.add_space(4.0);
        }
    }
}

fn page_history(ui: &mut egui::Ui, p: &Palette, lang: Lang, app: &mut MainApp) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new(
            RichText::new(i18n::t(lang, "history.title")).size(24.0).strong().color(p.text),
        ));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.add(egui::Button::new(
                RichText::new(format!("  {}", i18n::t(lang, "history.clear"))).color(p.text).size(12.0),
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
                let (rect, _) = ui.allocate_exact_size(Vec2::new(12.0, 12.0), egui::Sense::hover());
                let color = if h.ok { p.success } else { p.danger };
                icons::draw(ui, rect, color, 1.0, Icon::Dot);
                ui.add_space(8.0);
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
    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "about.title")).size(24.0).strong().color(p.text),
    ));
    ui.add_space(20.0);
    ui.add(egui::Label::new(
        RichText::new(format!("{}: 0.1.0", i18n::t(lang, "about.version"))).color(p.text_muted).size(13.0),
    ));
    ui.add_space(8.0);
    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "about.desc")).color(p.text).size(13.0),
    ));
    ui.add_space(20.0);
    ui.add(egui::Label::new(
        RichText::new(i18n::t(lang, "app.by")).strong().color(p.text).size(13.0),
    ));
}

fn who(lang: Lang) -> &'static str {
    match lang { Lang::Ru => "друг", Lang::En => "there" }
}
