//! Overlay window. Made by KebiLab

use crate::theme;
use eframe::egui::{self, Align, Layout, RichText, Vec2};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct OverlayApp {
    pub visible: Arc<AtomicBool>,
    pub status: String,
    pub recent: Vec<String>,
    pub query: String,
    pub wake_word: String,
}

impl OverlayApp {
    pub fn new(wake_word: String) -> Self {
        Self {
            visible: Arc::new(AtomicBool::new(false)),
            status: format!("Готов — скажите «{wake_word}»"),
            recent: vec![],
            query: String::new(),
            wake_word,
        }
    }

    pub fn toggle(&self) {
        let v = !self.visible.load(Ordering::SeqCst);
        self.visible.store(v, Ordering::SeqCst);
    }
}

impl eframe::App for OverlayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::install(ctx);

        if !self.visible.load(Ordering::SeqCst) {
            ctx.request_repaint_after(std::time::Duration::from_millis(200));
            return;
        }

        egui::Window::new("KebiControl")
            .title_bar(false)
            .resizable(false)
            .fixed_size(Vec2::new(520.0, 420.0))
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
                        if ui.button(RichText::new("✕").size(14.0).color(theme::TEXT_MUTED)).clicked() {
                            self.visible.store(false, Ordering::SeqCst);
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

                ui.add(
                    egui::TextEdit::singleline(&mut self.query)
                        .hint_text(RichText::new("Введите команду…").color(theme::TEXT_MUTED))
                        .desired_width(f32::INFINITY)
                        .frame(true)
                        .margin(egui::Margin::same(12.0))
                        .font(egui::TextStyle::Body),
                );
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    for label in ["Пауза", "Стоп", "Дальше"] {
                        let r = ui.add(
                            egui::Button::new(RichText::new(label).color(theme::TEXT_PRIMARY))
                                .min_size(Vec2::new(120.0, 38.0))
                                .rounding(egui::Rounding::same(10.0))
                                .fill(theme::BG_RAISED),
                        );
                        let _ = r;
                    }
                });
                ui.horizontal(|ui| {
                    for label in ["Скриншот", "Тише", "Громче"] {
                        let r = ui.add(
                            egui::Button::new(RichText::new(label).color(theme::TEXT_PRIMARY))
                                .min_size(Vec2::new(120.0, 38.0))
                                .rounding(egui::Rounding::same(10.0))
                                .fill(theme::BG_RAISED),
                        );
                        let _ = r;
                    }
                });
                ui.add_space(8.0);

                ui.add(egui::Label::new(RichText::new("История").size(11.0).color(theme::TEXT_MUTED)));
                egui::ScrollArea::vertical()
                    .max_height(120.0)
                    .show(ui, |ui| {
                        for item in &self.recent {
                            ui.add(egui::Label::new(RichText::new(item).color(theme::TEXT_PRIMARY).size(13.0)));
                        }
                    });
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}
