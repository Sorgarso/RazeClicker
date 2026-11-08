use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align2, CornerRadius, FontFamily, FontId, Sense, Ui, Vec2};
pub fn coord_row(ui: &mut Ui, index: usize, x: f32, y: f32) {
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 32.0), Sense::hover());

    let bg = if resp.hovered() { WIDGET_HOVER } else { WIDGET };
    ui.painter().rect_filled(rect, CornerRadius::same(8), bg);

    let mono = regular(13.0);
    let label = regular(11.0);

    let badge =
        egui::Rect::from_min_size(rect.left_top() + Vec2::new(6.0, 5.0), Vec2::new(28.0, 22.0));
    ui.painter()
        .rect_filled(badge, CornerRadius::same(6), ACCENT.gamma_multiply(0.2));
    ui.painter().text(
        badge.center(),
        Align2::CENTER_CENTER,
        format!("{}", index + 1),
        mono.clone(),
        ACCENT_HOVER,
    );

    let mut cx = badge.right() + 16.0;
    for (name, val) in [("X", x), ("Y", y)] {
        ui.painter().text(
            egui::pos2(cx, rect.center().y),
            Align2::LEFT_CENTER,
            name,
            label.clone(),
            MUTED,
        );
        ui.painter().text(
            egui::pos2(cx + 14.0, rect.center().y),
            Align2::LEFT_CENTER,
            format!("{:>5.0}", val),
            mono.clone(),
            TEXT,
        );
        cx += 90.0;
    }
}
