use crate::ui::theme::{CLOSE_HOVER, CLOSE_IDLE};
use egui::{Color32, FontFamily, FontId, Vec2};

pub fn close_button(ui: &mut egui::Ui) -> egui::Response {
    let desired = Vec2::splat(35.0);
    let (rect, resp) = ui.allocate_exact_size(desired, egui::Sense::click());

    let hovered = resp.contains_pointer();
    let t = ui.ctx().animate_bool_with_time(resp.id, hovered, 0.2);

    let bg = CLOSE_IDLE.lerp_to_gamma(CLOSE_HOVER, t);

    let corner = egui::CornerRadius::same(5);
    ui.painter().rect_filled(rect, corner, bg);

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "✖",
        FontId::new(14.0, FontFamily::Proportional),
        Color32::WHITE,
    );

    if hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
