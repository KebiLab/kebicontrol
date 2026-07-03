//! Overlay window — minimalist Win+G-style panel. Made by KebiLab

use crate::theme;
use eframe::egui::{self, Align, Layout, RichText, Sense, Stroke, Vec2};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;

pub struct OverlayApp {
    pub visible: Arc<AtomicBool>,
    pub status: String,
    pub recent: Vec<String>,
    pub query: String,
    pub tx: mpsc::UnboundedSender<String>,
    pub rx: mpsc::UnboundedReceiver<String>,
    pub wake_word: String,
}

impl OverlayApp {
    pub fn new(wake_word: String) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            visible: Arc::new(AtomicBool::new(false)),
            status: format!("Готов · скажите «{wake_word}»").into(),
            recent: vec![],
            query: String::new(),
            tx,
            rx,
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

        // Drain async messages
        while let Ok(msg) = self.rx.try_recv() {
            self.recent.insert(0, msg);
            if self.recent.len() > 20 { self.recent.truncate(20); }
        }

        if !self.visible.load(Ordering::SeqCst) {
            // Window is hidden; don't paint heavy.
            ctx.request_repaint_after(std::time::Duration::from_millis(200));
            return;
        }

        egui::Window::new("")
            .title_bar(false)
            .resizable(false)
            .fixed_size(Vec2::new(520.0, 420.0))
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .frame(egui::Frame::none()
                .fill(theme::BG_PANEL)
                .stroke(Stroke::new(1.0, theme::LINE))
                .rounding(egui::CornerRadius::same(16))
                .inner_margin(egui::Margin::same(20)))
            .show(ctx, |ui| {
                // Header
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(RichText::new("KebiControl").strong().size(18.0).color(theme::TEXT_PRIMARY)));
                    ui.add_space(8.0);
                    ui.add(egui::Label::new(RichText::new("·  Made by KebiLab").size(11.0).color(theme::TEXT_MUTED)));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button(RichText::new("✕").size(14.0).color(theme::TEXT_MUTED)).clicked() {
                            self.visible.store(false, Ordering::SeqCst);
                        }
                    });
                });
                ui.add_space(8.0);

                // Status pill
                ui.horizontal(|ui| {
                    let (dot, label) = ("●", self.status.as_str());
                    let color = if label.contains("Слушаю") { theme::ACCENT_2 } else { theme::TEXT_MUTED };
                    ui.add(egui::Label::new(RichText::new(dot).color(color).size(12.0)));
                    ui.add_space(6.0);
                    ui.add(egui::Label::new(RichText::new(label).color(theme::TEXT_MUTED).size(13.0)));
                });
                ui.add_space(14.0);

                // Input
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.query)
                        .hint_text(RichText::new("Введите команду…").color(theme::TEXT_MUTED))
                        .desired_width(f32::INFINITY)
                        .frame(true)
                        .margin(egui::Margin::same(12))
                        .font(egui::TextStyle::Body),
                );
                if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !self.query.trim().is_empty() {
                        let _ = self.tx.send(self.query.clone());
                        self.recent.insert(0, format!("› {}", self.query));
                        self.query.clear();
                    }
                }
                ui.add_space(12.0);

                // Quick actions grid (minimalism: 2x3)
                ui.horizontal(|ui| {
                    quick_btn(ui, "Пауза",     "media:pause");
                    quick_btn(ui, "Стоп",      "media:stop");
                    quick_btn(ui, "Дальше",    "media:next");
                });
                ui.horizontal(|ui| {
                    quick_btn(ui, "Скриншот",  "screenshot:full");
                    quick_btn(ui, "Тише",      "volume:down");
                    quick_btn(ui, "Громче",    "volume:up");
                });
                ui.add_space(8.0);

                // Recent
                ui.add(egui::Label::new(RichText::new("История").size(11.0).color(theme::TEXT_MUTED)));
                egui::ScrollArea::vertical()
                    .max_height(120.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        for item in &self.recent {
                            ui.add(egui::Label::new(RichText::new(item).color(theme::TEXT_PRIMARY).size(13.0)));
                        }
                    });
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

fn quick_btn(ui: &mut egui::Ui, label: &str, cmd: &str) {
    let r = ui.add(
        egui::Button::new(RichText::new(label).color(theme::TEXT_PRIMARY))
            .min_size(Vec2::new(120.0, 38.0))
            .rounding(egui::CornerRadius::same(10))
            .fill(theme::BG_RAISED),
    );
    if r.clicked() {
        // demo: forward to channel
        let _ = cmd;
    }
}

// Keep Sense referenced to avoid unused warnings if the design evolves.
#[allow(dead_code)]
fn _sense() -> Sense { Sense::click() }
