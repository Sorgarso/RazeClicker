use crate::ui::fonts::fonts::regular;
use egui::{Align2, Color32, Sense, Ui, vec2};

pub fn rec_badge(ui: &mut Ui, recording: bool) {
    let (rect, _) = ui.allocate_exact_size(vec2(78.0, 30.0), Sense::hover());

    let bg = if recording {
        Color32::from_rgb(48, 22, 29)
    } else {
        Color32::from_rgb(30, 30, 38)
    };
    let red = if recording {
        Color32::from_rgb(255, 75, 75)
    } else {
        Color32::from_rgb(85, 85, 95)
    };

    let p = ui.painter();
    p.rect_filled(rect, 17, bg);

    let dot = egui::pos2(rect.left() + 22.0, rect.center().y);
    if recording {
        let t = ui.input(|i| i.time);
        let pulse = ((t * 2.5).sin() * 0.5 + 0.5) as f32;
        p.circle_filled(
            dot,
            6.0 + pulse * 4.0,
            Color32::from_rgba_unmultiplied(255, 75, 75, (pulse * 50.0) as u8),
        );
        ui.ctx().request_repaint();
    }
    p.circle_filled(dot, 6.0, red);
    p.text(
        egui::pos2(rect.left() + 34.0, rect.center().y),
        Align2::LEFT_CENTER,
        if recording { "REC" } else { "IDLE" },
        regular(14.0),
        red,
    );
}
