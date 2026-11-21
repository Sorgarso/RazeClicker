use crate::ui::theme::{CARD_BORDER, MIN_HOVER, MIN_IDLE, MUTED};
use egui::{Color32, Stroke, StrokeKind, Vec2, pos2};

pub fn minimize_button(ui: &mut egui::Ui) -> egui::Response {
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
    let bg = MIN_IDLE.lerp_to_gamma(MIN_HOVER, hover_t);
    let border = CARD_BORDER.lerp_to_gamma(
        Color32::from_rgba_unmultiplied(222, 163, 195, 160),
        hover_t * 0.6,
    );
    let fg = MUTED.lerp_to_gamma(Color32::WHITE, hover_t);

    let p = ui.painter();
    p.rect_filled(r, 8, bg);
    p.rect_stroke(r, 8, Stroke::new(1.0, border), StrokeKind::Inside);

    let c = r.center();
    p.line_segment(
        [pos2(c.x - 5.5, c.y), pos2(c.x + 5.5, c.y)],
        Stroke::new(1.6, fg),
    );

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
