use crate::ui::theme::{CARD_BORDER, CLOSE_HOVER, CLOSE_IDLE, MUTED};
use egui::{Color32, Stroke, StrokeKind, Vec2, pos2};

pub fn close_button(ui: &mut egui::Ui) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(35.0), egui::Sense::click());

    let hover_t =
        ui.ctx()
            .animate_bool_with_time(resp.id.with("hover"), resp.contains_pointer(), 0.2);
    let press_t = ui.ctx().animate_bool_with_time(
        resp.id.with("press"),
        resp.is_pointer_button_down_on(),
        0.1,
    );

    let r = rect.shrink(press_t * 1.5);
    let bg = CLOSE_IDLE.lerp_to_gamma(CLOSE_HOVER, hover_t);
    let border = CARD_BORDER.lerp_to_gamma(Color32::from_rgb(255, 90, 100), hover_t * 0.7);
    let fg = MUTED.lerp_to_gamma(Color32::WHITE, hover_t);

    let p = ui.painter();
    p.rect_filled(r, 8, bg);
    p.rect_stroke(r, 8, Stroke::new(1.0, border), StrokeKind::Inside);

    let c = r.center();
    let d = 4.5;
    let stroke = Stroke::new(1.6, fg);
    p.line_segment([pos2(c.x - d, c.y - d), pos2(c.x + d, c.y + d)], stroke);
    p.line_segment([pos2(c.x - d, c.y + d), pos2(c.x + d, c.y - d)], stroke);

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
