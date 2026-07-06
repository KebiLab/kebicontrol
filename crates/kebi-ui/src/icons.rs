//! Minimalist icons. Stroke-based, scalable. Made by KebiLab

use eframe::egui::{Color32, Pos2, Rect, Shape, Stroke, Vec2};

fn px(rect: Rect, x: f32, y: f32) -> Pos2 { Pos2::new(rect.left() + x, rect.top() + y) }

pub fn draw(ui: &mut egui::Ui, rect: Rect, color: Color32, stroke: f32, icon: Icon) {
    let s = stroke;
    let p = |x, y| px(rect, x, y);
    match icon {
        Icon::Play => {
            let tri = [p(4.0, 3.0), p(12.0, 8.0), p(4.0, 13.0)];
            ui.painter().add(Shape::convex_polygon(tri.to_vec(), color, Stroke::NONE));
        }
        Icon::Pause => {
            ui.painter().rect_filled(Rect::from_min_size(p(4.0, 3.0), Vec2::new(2.5, 10.0)), 0.5, color);
            ui.painter().rect_filled(Rect::from_min_size(p(9.5, 3.0), Vec2::new(2.5, 10.0)), 0.5, color);
        }
        Icon::Screenshot => {
            ui.painter().line_segment([p(2.0, 5.0), p(2.0, 2.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(2.0, 2.0), p(5.0, 2.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(11.0, 2.0), p(14.0, 2.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(14.0, 2.0), p(14.0, 5.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(2.0, 11.0), p(2.0, 14.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(2.0, 14.0), p(5.0, 14.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(11.0, 14.0), p(14.0, 14.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(14.0, 11.0), p(14.0, 14.0)], Stroke::new(s, color));
        }
        Icon::VolumeUp => {
            let body = [p(3.0, 6.0), p(6.0, 6.0), p(9.0, 3.0), p(9.0, 13.0), p(6.0, 10.0), p(3.0, 10.0)];
            ui.painter().add(Shape::convex_polygon(body.to_vec(), color, Stroke::NONE));
            ui.painter().line_segment([p(11.0, 5.0), p(13.0, 3.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(11.0, 11.0), p(13.0, 13.0)], Stroke::new(s, color));
        }
        Icon::VolumeDown => {
            let body = [p(3.0, 6.0), p(6.0, 6.0), p(9.0, 3.0), p(9.0, 13.0), p(6.0, 10.0), p(3.0, 10.0)];
            ui.painter().add(Shape::convex_polygon(body.to_vec(), color, Stroke::NONE));
            ui.painter().line_segment([p(11.0, 8.0), p(13.0, 8.0)], Stroke::new(s, color));
        }
        Icon::Settings => {
            let center = p(8.0, 8.0);
            ui.painter().circle_stroke(center, 5.0, Stroke::new(s, color));
            ui.painter().circle_filled(center, 1.6, color);
            for i in 0..8 {
                let a = (i as f32) * std::f32::consts::TAU / 8.0 + std::f32::consts::PI / 8.0;
                let inner = Pos2::new(center.x + a.cos() * 5.0, center.y + a.sin() * 5.0);
                let outer = Pos2::new(center.x + a.cos() * 7.0, center.y + a.sin() * 7.0);
                ui.painter().line_segment([inner, outer], Stroke::new(s + 0.5, color));
            }
        }
        Icon::Close => {
            ui.painter().line_segment([p(4.0, 4.0), p(12.0, 12.0)], Stroke::new(s + 0.5, color));
            ui.painter().line_segment([p(12.0, 4.0), p(4.0, 12.0)], Stroke::new(s + 0.5, color));
        }
        Icon::ChevronDown => {
            ui.painter().line_segment([p(4.0, 6.0), p(8.0, 10.0)], Stroke::new(s + 0.3, color));
            ui.painter().line_segment([p(8.0, 10.0), p(12.0, 6.0)], Stroke::new(s + 0.3, color));
        }
        Icon::Dot => {
            ui.painter().circle_filled(p(8.0, 8.0), 2.5, color);
        }
        Icon::Eye => {
            ui.painter().line_segment([p(2.0, 8.0), p(4.0, 5.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(4.0, 5.0), p(8.0, 4.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(8.0, 4.0), p(12.0, 5.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(12.0, 5.0), p(14.0, 8.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(14.0, 8.0), p(12.0, 11.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(12.0, 11.0), p(8.0, 12.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(8.0, 12.0), p(4.0, 11.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(4.0, 11.0), p(2.0, 8.0)], Stroke::new(s, color));
            ui.painter().circle_filled(p(8.0, 8.0), 2.2, color);
        }
        Icon::Mic => {
            let capsule = [p(6.0, 2.0), p(10.0, 2.0), p(10.0, 9.0), p(6.0, 9.0)];
            ui.painter().add(Shape::convex_polygon(capsule.to_vec(), color, Stroke::NONE));
            ui.painter().line_segment([p(4.0, 7.0), p(12.0, 7.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(8.0, 7.0), p(8.0, 12.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(5.0, 12.0), p(11.0, 12.0)], Stroke::new(s, color));
        }
        Icon::Logo => {
            // Microphone: capsule, U-arc, stem, base — flat fill.
            let r = rect;
            let _ = r; // unused; use specific geometry
            // Capsule
            let capsule = [p(6.0, 2.0), p(10.0, 2.0), p(10.0, 9.0), p(6.0, 9.0)];
            ui.painter().add(Shape::convex_polygon(capsule.to_vec(), color, Stroke::NONE));
            // U arc
            ui.painter().line_segment([p(3.0, 7.0), p(3.0, 8.0)], Stroke::new(1.2, color));
            ui.painter().line_segment([p(13.0, 7.0), p(13.0, 8.0)], Stroke::new(1.2, color));
            ui.painter().line_segment([p(3.0, 7.0), p(13.0, 7.0)], Stroke::new(1.2, color));
            // Stem + base
            ui.painter().line_segment([p(8.0, 9.0), p(8.0, 12.0)], Stroke::new(1.2, color));
            ui.painter().line_segment([p(5.0, 13.0), p(11.0, 13.0)], Stroke::new(1.2, color));
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    Play, Pause, Screenshot, VolumeUp, VolumeDown,
    Settings, Close, ChevronDown, Dot, Eye, Mic, Logo,
}
