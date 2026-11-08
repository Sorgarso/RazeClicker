use crate::ui::theme::*;
use egui::{Color32, CornerRadius, Sense, Ui, Vec2};

pub fn toggle(ui: &mut Ui, on: &mut bool) {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(52.0, 28.0), Sense::click());
    if resp.clicked() {
        *on = !*on;
    }
    let t = ui.ctx().animate_bool_with_time(resp.id, *on, 0.12);
    ui.painter().rect_filled(
        rect,
        CornerRadius::same(14),
        WIDGET_HOVER.lerp_to_gamma(ACCENT, t),
    );

    let r = rect.height() / 2.0 - 3.0;
    let x = egui::lerp((rect.left() + r + 3.0)..=(rect.right() - r - 3.0), t);
    ui.painter()
        .circle_filled(egui::pos2(x, rect.center().y), r, Color32::WHITE);

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
}
