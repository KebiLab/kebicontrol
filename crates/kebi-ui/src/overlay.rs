//! Overlay window. Made by KebiLab

use crate::theme;
use eframe::egui::{self, Align, Layout, RichText, Vec2};
use std::sync::atomic::Ordering;

pub struct OverlayApp {
    pub status: String,
    pub recent: Vec<String>,
    pub query: String,
    pub open_settings: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl OverlayApp {
    pub fn new(open_settings: std::sync::Arc<std::sync::atomic::AtomicBool>) -> Self {
        Self {
            status: "Готов — скажите «кеби»".into(),
            recent: vec![],
            query: String::new(),
            open_settings,
        }
    }
}

impl eframe::App for OverlayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::install(ctx);

        egui::Window::new("KebiControl")
            .title_bar(true)
            .resizable(true)
            .default_size(Vec2::new(520.0, 520.0))
            .min_width(480.0)
            .min_height(420.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .frame(egui::Frame::none()
                .fill(theme::BG_PANEL)
                .stroke(egui::Stroke::new(1.0, theme::LINE))
                .rounding(egui::Rounding::same(16.0))
                .inner_margin(egui::Margin::same(20.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(
                        RichText::new("KebiControl").strong().size(18.0).color(theme::TEXT_PRIMARY),
                    ));
                    ui.add_space(8.0);
                    ui.add(egui::Label::new(
                        RichText::new("· Made by KebiLab").size(11.0).color(theme::TEXT_MUTED),
                    ));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button(RichText::new("⚙ Настройки").color(theme::TEXT_PRIMARY)).clicked() {
                            self.open_settings.store(true, Ordering::SeqCst);
                        }
                    });
                });
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    let color = if self.status.starts_with("Слушаю") { theme::ACCENT_2 } else { theme::TEXT_MUTED };
                    ui.add(egui::Label::new(RichText::new("●").color(color).size(12.0)));
                    ui.add_space(6.0);
                    ui.add(egui::Label::new(RichText::new(&self.status).color(theme::TEXT_MUTED).size(13.0)));
                });
                ui.add_space(14.0);

                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.query)
                        .hint_text(RichText::new("Введите команду…").color(theme::TEXT_MUTED))
                        .desired_width(f32::INFINITY)
                        .frame(true)
                        .margin(egui::Margin::same(12.0))
                        .font(egui::TextStyle::Body),
                );
                if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !self.query.trim().is_empty() {
                        self.recent.insert(0, format!("› {}", self.query));
                        if self.recent.len() > 20 { self.recent.truncate(20); }
                        self.query.clear();
                    }
                }
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    for label in ["Пауза", "Стоп", "Дальше"] {
                        let _ = ui.add(
                            egui::Button::new(RichText::new(label).color(theme::TEXT_PRIMARY))
                                .min_size(Vec2::new(120.0, 38.0))
                                .rounding(egui::Rounding::same(10.0))
                                .fill(theme::BG_RAISED),
                        );
                    }
                });
                ui.horizontal(|ui| {
                    for label in ["Скриншот", "Тише", "Громче"] {
                        let _ = ui.add(
                            egui::Button::new(RichText::new(label).color(theme::TEXT_PRIMARY))
                                .min_size(Vec2::new(120.0, 38.0))
                                .rounding(egui::Rounding::same(10.0))
                                .fill(theme::BG_RAISED),
                        );
                    }
                });
                ui.add_space(8.0);

                ui.add(egui::Label::new(RichText::new("История").size(11.0).color(theme::TEXT_MUTED)));
                egui::ScrollArea::vertical()
                    .max_height(160.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        if self.recent.is_empty() {
                            ui.add(egui::Label::new(
                                RichText::new("Пока пусто. Скажите: «кеби хелп»")
                                    .color(theme::TEXT_MUTED).size(12.0).italics(),
                            ));
                        } else {
                            for item in &self.recent {
                                ui.add(egui::Label::new(
                                    RichText::new(item).color(theme::TEXT_PRIMARY).size(13.0),
                                ));
                            }
                        }
                    });

                ui.add_space(8.0);
                ui.collapsing(RichText::new("Горячие клавиши").size(11.0).color(theme::TEXT_MUTED), |ui| {
                    for (k, v) in [
                        ("Ctrl+Shift+Space", "Push-to-listen"),
                        ("Ctrl+Shift+K", "Открыть/скрыть это окно"),
                        ("Ctrl+Shift+M", "Голос вкл/выкл"),
                        ("Ctrl+Shift+D", "Диктовка"),
                        ("Ctrl+Shift+P", "Пауза"),
                    ] {
                        ui.horizontal(|ui| {
                            ui.add(egui::Label::new(
                                RichText::new(k).color(theme::ACCENT).size(12.0),
                            ));
                            ui.add_space(12.0);
                            ui.add(egui::Label::new(
                                RichText::new(v).color(theme::TEXT_MUTED).size(12.0),
                            ));
                        });
                    }
                });
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}
