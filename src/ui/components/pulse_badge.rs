use crate::ui::components::glow::radial_glow;
use crate::ui::fonts::fonts::icon_font;
use egui::{Align2, Color32, Rect, Sense, Stroke, StrokeKind, Ui, Vec2, pos2, vec2};
use std::f32::consts::TAU;
use std::time::Duration;

pub fn pulse_badge(ui: &mut Ui, icon: &str, color: Color32, spinning: bool) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 128.0), Sense::hover());
    let c = rect.center();
    let t = ui.input(|i| i.time) as f32;
    ui.ctx().request_repaint_after(Duration::from_millis(33));

    let tint = |a: f32| {
        Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), a.clamp(0.0, 255.0) as u8)
    };
    let p = ui.painter();

    let breath = 0.5 + 0.5 * (t * 1.6).sin();
    radial_glow(p, c, 1.0, 86.0, tint(46.0 + 22.0 * breath));

    for i in 0..2 {
        let ph = (t * 0.45 + i as f32 * 0.5).fract();
        p.circle_stroke(
            c,
            40.0 + ph * 34.0,
            Stroke::new(1.2, tint((1.0 - ph) * 70.0)),
        );
    }

    if spinning {
        let n = 28;
        for i in 0..n {
            let a0 = t * 3.2 - i as f32 * 0.09;
            let a1 = a0 - 0.09;
            let f = 1.0 - i as f32 / n as f32;
            let r = 46.0;
            p.line_segment(
                [
                    pos2(c.x + r * a0.cos(), c.y + r * a0.sin()),
                    pos2(c.x + r * a1.cos(), c.y + r * a1.sin()),
                ],
                Stroke::new(2.0, tint(230.0 * f * f)),
            );
        }
    }

    let body = Rect::from_center_size(c, Vec2::splat(64.0));
    p.rect_filled(body, 18, tint(34.0));
    p.rect_stroke(body, 18, Stroke::new(1.0, tint(130.0)), StrokeKind::Inside);
    p.text(c, Align2::CENTER_CENTER, icon, icon_font(26.0), color);

    let _ = TAU;
}
