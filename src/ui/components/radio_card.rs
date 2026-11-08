use crate::ui::{fonts::fonts::regular, theme::*};
use egui::{
    Align2, CornerRadius, FontFamily, FontId, Response, Sense, Stroke, StrokeKind, Ui, Vec2,
};

pub fn radio_card(ui: &mut Ui, selected: bool, text: &str) -> Response {
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 46.0), Sense::click());
    let t = ui
        .ctx()
        .animate_bool_with_time(resp.id.with("sel"), selected, 0.12);

    let bg = if selected {
        ACCENT.gamma_multiply(0.12)
    } else if resp.hovered() {
        WIDGET_HOVER
    } else {
        WIDGET
    };
    let border = CARD_BORDER.lerp_to_gamma(ACCENT, t);
    ui.painter().rect(
        rect,
        CornerRadius::same(10),
        bg,
        Stroke::new(1.0 + 0.5 * t, border),
        StrokeKind::Inside,
    );

    let c = rect.left_center() + Vec2::new(24.0, 0.0);
    ui.painter().circle_stroke(c, 9.0, Stroke::new(1.5, border));
    if selected {
        ui.painter().circle_filled(c, 5.0, ACCENT);
    }
    ui.painter().text(
        rect.left_center() + Vec2::new(54.0, 0.0),
        Align2::LEFT_CENTER,
        text,
        regular(15.0),
        TEXT,
    );

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
