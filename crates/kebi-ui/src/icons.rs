//! Minimalist vector icons drawn with egui's `Shape` API.
//! No emoji, no external assets. Made by KebiLab

use eframe::egui::{self, Color32, Pos2, Rect, Sense, Shape, Stroke, Vec2};

/// 16x16 logical icon, scaled at draw time.
fn px(rect: Rect, x: f32, y: f32) -> Pos2 {
    Pos2::new(rect.left() + x, rect.top() + y)
}

/// Draw a stroke icon into the given rect.
pub fn draw(ui: &mut egui::Ui, rect: Rect, color: Color32, stroke: f32, icon: Icon) {
    let s = stroke;
    match icon {
        Icon::Mic => {
            // Capsule + stand + base
            let capsule = [px(rect, 6.0, 2.0), px(rect, 10.0, 2.0),
                           px(rect, 10.0, 9.0), px(rect, 6.0, 9.0)];
            ui.painter().add(Shape::convex_polygon(capsule.to_vec(), color, Stroke::NONE));
            ui.painter().line_segment([px(rect, 4.0, 7.0), px(rect, 12.0, 7.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 8.0, 7.0), px(rect, 8.0, 12.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 5.0, 12.0), px(rect, 11.0, 12.0)], Stroke::new(s, color));
        }
        Icon::MicOff => {
            let capsule = [px(rect, 6.0, 2.0), px(rect, 10.0, 2.0),
                           px(rect, 10.0, 9.0), px(rect, 6.0, 9.0)];
            ui.painter().add(Shape::convex_polygon(capsule.to_vec(), color, Stroke::NONE));
            // strike
            ui.painter().line_segment([px(rect, 3.0, 3.0), px(rect, 13.0, 13.0)], Stroke::new(s + 0.5, color));
        }
        Icon::Pause => {
            ui.painter().rect_filled(Rect::from_min_size(px(rect, 4.0, 3.0), Vec2::new(2.5, 10.0)), 0.5, color);
            ui.painter().rect_filled(Rect::from_min_size(px(rect, 9.5, 3.0), Vec2::new(2.5, 10.0)), 0.5, color);
        }
        Icon::Stop => {
            ui.painter().rect_filled(Rect::from_min_size(px(rect, 4.0, 4.0), Vec2::new(8.0, 8.0)), 1.0, color);
        }
        Icon::Next => {
            let tri = vec![px(rect, 4.0, 3.0), px(rect, 9.0, 8.0), px(rect, 4.0, 13.0)];
            ui.painter().add(Shape::convex_polygon(tri, color, Stroke::NONE));
            ui.painter().rect_filled(Rect::from_min_size(px(rect, 10.0, 3.0), Vec2::new(1.5, 10.0)), 0.3, color);
        }
        Icon::Prev => {
            let tri = vec![px(rect, 12.0, 3.0), px(rect, 7.0, 8.0), px(rect, 12.0, 13.0)];
            ui.painter().add(Shape::convex_polygon(tri, color, Stroke::NONE));
            ui.painter().rect_filled(Rect::from_min_size(px(rect, 4.5, 3.0), Vec2::new(1.5, 10.0)), 0.3, color);
        }
        Icon::Screenshot => {
            // corners + center dot
            let c = 2.0;
            ui.painter().line_segment([px(rect, 2.0, 5.0), px(rect, 2.0, 2.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 2.0, 2.0), px(rect, 5.0, 2.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 11.0, 2.0), px(rect, 14.0, 2.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 14.0, 2.0), px(rect, 14.0, 5.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 2.0, 11.0), px(rect, 2.0, 14.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 2.0, 14.0), px(rect, 5.0, 14.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 11.0, 14.0), px(rect, 14.0, 14.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 14.0, 11.0), px(rect, 14.0, 14.0)], Stroke::new(s, color));
            let _ = c;
        }
        Icon::VolumeUp => {
            // speaker
            let s1 = [px(rect, 3.0, 6.0), px(rect, 6.0, 6.0), px(rect, 9.0, 3.0), px(rect, 9.0, 13.0), px(rect, 6.0, 10.0), px(rect, 3.0, 10.0)];
            ui.painter().add(Shape::convex_polygon(s1.to_vec(), color, Stroke::NONE));
            // waves
            ui.painter().line_segment([px(rect, 11.0, 5.0), px(rect, 13.0, 3.0)], Stroke::new(s, color));
            ui.painter().line_segment([px(rect, 11.0, 11.0), px(rect, 13.0, 13.0)], Stroke::new(s, color));
        }
        Icon::VolumeDown => {
            let s1 = [px(rect, 3.0, 6.0), px(rect, 6.0, 6.0), px(rect, 9.0, 3.0), px(rect, 9.0, 13.0), px(rect, 6.0, 10.0), px(rect, 3.0, 10.0)];
            ui.painter().add(Shape::convex_polygon(s1.to_vec(), color, Stroke::NONE));
            ui.painter().line_segment([px(rect, 11.0, 8.0), px(rect, 13.0, 8.0)], Stroke::new(s, color));
        }
        Icon::Settings => {
            // gear: outer ring (stroked) + inner hole
            let center = px(rect, 8.0, 8.0);
            ui.painter().circle_stroke(center, 5.0, Stroke::new(s, color));
            ui.painter().circle_filled(center, 1.6, color);
            // 8 teeth
            for i in 0..8 {
                let a = (i as f32) * std::f32::consts::TAU / 8.0 + std::f32::consts::PI / 8.0;
                let inner = Pos2::new(center.x + a.cos() * 5.0, center.y + a.sin() * 5.0);
                let outer = Pos2::new(center.x + a.cos() * 7.0, center.y + a.sin() * 7.0);
                ui.painter().line_segment([inner, outer], Stroke::new(s + 0.5, color));
            }
        }
        Icon::Save => {
            // floppy: square with notch and inner rect
            ui.painter().rect_filled(Rect::from_min_size(px(rect, 2.0, 2.0), Vec2::new(12.0, 12.0)), 1.0, Color32::TRANSPARENT);
            ui.painter().rect_stroke(Rect::from_min_size(px(rect, 2.0, 2.0), Vec2::new(12.0, 12.0)), 0.0, Stroke::new(s, color));
            // label area
            ui.painter().rect_filled(Rect::from_min_size(px(rect, 4.0, 2.0), Vec2::new(8.0, 4.0)), 0.0, color);
            // bottom slot
            ui.painter().rect_stroke(Rect::from_min_size(px(rect, 5.0, 8.0), Vec2::new(6.0, 4.0)), 0.0, Stroke::new(s, color));
        }
        Icon::Close => {
            ui.painter().line_segment([px(rect, 4.0, 4.0), px(rect, 12.0, 12.0)], Stroke::new(s + 0.5, color));
            ui.painter().line_segment([px(rect, 12.0, 4.0), px(rect, 4.0, 12.0)], Stroke::new(s + 0.5, color));
        }
        Icon::ChevronDown => {
            ui.painter().line_segment([px(rect, 4.0, 6.0), px(rect, 8.0, 10.0)], Stroke::new(s + 0.3, color));
            ui.painter().line_segment([px(rect, 8.0, 10.0), px(rect, 12.0, 6.0)], Stroke::new(s + 0.3, color));
        }
        Icon::Dot => {
            ui.painter().circle_filled(px(rect, 8.0, 8.0), 2.5, color);
        }
        Icon::Search => {
            // circle + handle
            ui.painter().circle_stroke(px(rect, 7.0, 7.0), 3.5, Stroke::new(s + 0.3, color));
            ui.painter().line_segment([px(rect, 9.5, 9.5), px(rect, 13.0, 13.0)], Stroke::new(s + 0.5, color));
        }
    }
}

/// Allocate space and draw an icon inline. Returns the response.
pub fn icon_button(ui: &mut egui::Ui, icon: Icon, color: Color32) -> egui::Response {
    let desired = Vec2::new(18.0, 18.0);
    let (rect, response) = ui.allocate_exact_size(desired, Sense::click());
    draw(ui, rect, color, 1.2, icon);
    response
}

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    Mic, MicOff, Pause, Stop, Next, Prev, Screenshot,
    VolumeUp, VolumeDown, Settings, Save, Close, ChevronDown, Dot, Search,
}
