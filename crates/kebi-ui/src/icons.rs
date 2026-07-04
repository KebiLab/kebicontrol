//! Minimalist vector icons. No emoji. Made by KebiLab

use eframe::egui::{self, Color32, Pos2, Rect, Sense, Shape, Stroke, Vec2};

fn px(rect: Rect, x: f32, y: f32) -> Pos2 {
    Pos2::new(rect.left() + x, rect.top() + y)
}

pub fn draw(ui: &mut egui::Ui, rect: Rect, color: Color32, stroke: f32, icon: Icon) {
    let s = stroke;
    let r = rect;
    let p = |x, y| px(r, x, y);
    match icon {
        Icon::Mic => {
            let capsule = [p(6.0, 2.0), p(10.0, 2.0), p(10.0, 9.0), p(6.0, 9.0)];
            ui.painter().add(Shape::convex_polygon(capsule.to_vec(), color, Stroke::NONE));
            ui.painter().line_segment([p(4.0, 7.0), p(12.0, 7.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(8.0, 7.0), p(8.0, 12.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(5.0, 12.0), p(11.0, 12.0)], Stroke::new(s, color));
        }
        Icon::MicOff => {
            let capsule = [p(6.0, 2.0), p(10.0, 2.0), p(10.0, 9.0), p(6.0, 9.0)];
            ui.painter().add(Shape::convex_polygon(capsule.to_vec(), color, Stroke::NONE));
            ui.painter().line_segment([p(3.0, 3.0), p(13.0, 13.0)], Stroke::new(s + 0.5, color));
        }
        Icon::Pause => {
            ui.painter().rect_filled(Rect::from_min_size(p(4.0, 3.0), Vec2::new(2.5, 10.0)), 0.5, color);
            ui.painter().rect_filled(Rect::from_min_size(p(9.5, 3.0), Vec2::new(2.5, 10.0)), 0.5, color);
        }
        Icon::Stop => {
            ui.painter().rect_filled(Rect::from_min_size(p(4.0, 4.0), Vec2::new(8.0, 8.0)), 1.0, color);
        }
        Icon::Next => {
            let tri = vec![p(4.0, 3.0), p(9.0, 8.0), p(4.0, 13.0)];
            ui.painter().add(Shape::convex_polygon(tri, color, Stroke::NONE));
            ui.painter().rect_filled(Rect::from_min_size(p(10.0, 3.0), Vec2::new(1.5, 10.0)), 0.3, color);
        }
        Icon::Prev => {
            let tri = vec![p(12.0, 3.0), p(7.0, 8.0), p(12.0, 13.0)];
            ui.painter().add(Shape::convex_polygon(tri, color, Stroke::NONE));
            ui.painter().rect_filled(Rect::from_min_size(p(4.5, 3.0), Vec2::new(1.5, 10.0)), 0.3, color);
        }
        Icon::Screenshot => {
            let c = 2.0;
            ui.painter().line_segment([p(2.0, 5.0), p(2.0, 2.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(2.0, 2.0), p(5.0, 2.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(11.0, 2.0), p(14.0, 2.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(14.0, 2.0), p(14.0, 5.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(2.0, 11.0), p(2.0, 14.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(2.0, 14.0), p(5.0, 14.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(11.0, 14.0), p(14.0, 14.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(14.0, 11.0), p(14.0, 14.0)], Stroke::new(s, color));
            let _ = c;
        }
        Icon::VolumeUp => {
            let s1 = [p(3.0, 6.0), p(6.0, 6.0), p(9.0, 3.0), p(9.0, 13.0), p(6.0, 10.0), p(3.0, 10.0)];
            ui.painter().add(Shape::convex_polygon(s1.to_vec(), color, Stroke::NONE));
            ui.painter().line_segment([p(11.0, 5.0), p(13.0, 3.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(11.0, 11.0), p(13.0, 13.0)], Stroke::new(s, color));
        }
        Icon::VolumeDown => {
            let s1 = [p(3.0, 6.0), p(6.0, 6.0), p(9.0, 3.0), p(9.0, 13.0), p(6.0, 10.0), p(3.0, 10.0)];
            ui.painter().add(Shape::convex_polygon(s1.to_vec(), color, Stroke::NONE));
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
        Icon::Save => {
            ui.painter().rect_stroke(Rect::from_min_size(p(2.0, 2.0), Vec2::new(12.0, 12.0)), 0.0, Stroke::new(s, color));
            ui.painter().rect_filled(Rect::from_min_size(p(4.0, 2.0), Vec2::new(8.0, 4.0)), 0.0, color);
            ui.painter().rect_stroke(Rect::from_min_size(p(5.0, 8.0), Vec2::new(6.0, 4.0)), 0.0, Stroke::new(s, color));
        }
        Icon::Close => {
            ui.painter().line_segment([p(4.0, 4.0), p(12.0, 12.0)], Stroke::new(s + 0.5, color));
            ui.painter().line_segment([p(12.0, 4.0), p(4.0, 12.0)], Stroke::new(s + 0.5, color));
        }
        Icon::ChevronDown => {
            ui.painter().line_segment([p(4.0, 6.0), p(8.0, 10.0)], Stroke::new(s + 0.3, color));
            ui.painter().line_segment([p(8.0, 10.0), p(12.0, 6.0)], Stroke::new(s + 0.3, color));
        }
        Icon::ChevronRight => {
            ui.painter().line_segment([p(6.0, 4.0), p(10.0, 8.0)], Stroke::new(s + 0.3, color));
            ui.painter().line_segment([p(10.0, 8.0), p(6.0, 12.0)], Stroke::new(s + 0.3, color));
        }
        Icon::Dot => {
            ui.painter().circle_filled(p(8.0, 8.0), 2.5, color);
        }
        Icon::Search => {
            ui.painter().circle_stroke(p(7.0, 7.0), 3.5, Stroke::new(s + 0.3, color));
            ui.painter().line_segment([p(9.5, 9.5), p(13.0, 13.0)], Stroke::new(s + 0.5, color));
        }
        Icon::Play => {
            let tri = vec![p(4.0, 3.0), p(12.0, 8.0), p(4.0, 13.0)];
            ui.painter().add(Shape::convex_polygon(tri, color, Stroke::NONE));
        }
        Icon::Home => {
            let roof = vec![p(2.0, 8.0), p(8.0, 3.0), p(14.0, 8.0)];
            ui.painter().line_segment([p(2.0, 8.0), p(14.0, 8.0)], Stroke::new(s, color));
            ui.painter().add(Shape::convex_polygon(roof, color, Stroke::NONE));
            ui.painter().rect_filled(Rect::from_min_size(p(4.0, 8.0), Vec2::new(8.0, 6.0)), 0.0, color);
        }
        Icon::History => {
            ui.painter().circle_stroke(p(8.0, 8.0), 5.0, Stroke::new(s, color));
            ui.painter().line_segment([p(8.0, 8.0), p(8.0, 5.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(8.0, 8.0), p(11.0, 8.0)], Stroke::new(s, color));
        }
        Icon::Key => {
            ui.painter().circle_stroke(p(5.0, 8.0), 2.0, Stroke::new(s, color));
            ui.painter().line_segment([p(6.5, 8.0), p(14.0, 8.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(12.0, 8.0), p(12.0, 11.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(14.0, 8.0), p(14.0, 10.0)], Stroke::new(s, color));
        }
        Icon::Info => {
            ui.painter().circle_stroke(p(8.0, 8.0), 6.0, Stroke::new(s, color));
            ui.painter().circle_filled(p(8.0, 5.0), 0.8, color);
            ui.painter().rect_filled(Rect::from_min_size(p(7.0, 7.0), Vec2::new(2.0, 5.0)), 0.0, color);
        }
        Icon::Logo => {
            // Stylized K with voice dot.
            // Vertical bar
            ui.painter().rect_filled(Rect::from_min_size(p(3.0, 2.0), Vec2::new(2.5, 12.0)), 0.6, color);
            // Upper diagonal
            let upper = vec![p(5.5, 8.0), p(8.5, 4.0), p(10.5, 4.0), p(7.5, 8.0)];
            ui.painter().add(Shape::convex_polygon(upper, color, Stroke::NONE));
            // Lower diagonal
            let lower = vec![p(5.5, 8.0), p(8.5, 12.0), p(10.5, 12.0), p(7.5, 8.0)];
            ui.painter().add(Shape::convex_polygon(lower, color, Stroke::NONE));
            // Voice dot
            ui.painter().circle_filled(p(13.0, 4.0), 1.4, color);
        }
        Icon::Eye => {
            // Eye outline + pupil
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
        Icon::EyeOff => {
            ui.painter().line_segment([p(2.0, 8.0), p(14.0, 8.0)], Stroke::new(s, color));
            ui.painter().line_segment([p(3.0, 4.0), p(13.0, 12.0)], Stroke::new(s + 0.5, color));
        }
    }
}

/// Allocate space and draw an icon inline. Returns the response.
#[allow(dead_code)]
pub fn icon_button(ui: &mut egui::Ui, icon: Icon, color: Color32) -> egui::Response {
    let desired = Vec2::new(18.0, 18.0);
    let (rect, response) = ui.allocate_exact_size(desired, Sense::click());
    draw(ui, rect, color, 1.2, icon);
    response
}

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    Mic, MicOff, Pause, Stop, Next, Prev, Screenshot,
    VolumeUp, VolumeDown, Settings, Save, Close, ChevronDown, ChevronRight, Dot, Search,
    Play, Home, History, Key, Info, Logo, Eye, EyeOff,
}
