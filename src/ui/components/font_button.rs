use crate::ui::fonts::fonts::icon_font;
use crate::ui::theme::{CARD_BORDER, MUTED};
use egui::{Color32, Stroke, StrokeKind, Vec2};

pub fn font_button(
    ui: &mut egui::Ui,
    color_bg: Color32,
    color_hover: Color32,
    corner_radius: u8,
    text: &str,
    size: Vec2,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    let hover_t =
        ui.ctx()
            .animate_bool_with_time(resp.id.with("hover"), resp.contains_pointer(), 0.2);
    let press_t = ui.ctx().animate_bool_with_time(
        resp.id.with("press"),
        resp.is_pointer_button_down_on(),
        0.1,
    );

    let r = rect.shrink(press_t * 1.5);
    let bg = color_bg.lerp_to_gamma(color_hover, hover_t);
    let fg = MUTED.lerp_to_gamma(Color32::WHITE, hover_t);

    let p = ui.painter();
    p.rect_filled(r, corner_radius, bg);

    p.text(
        r.center(),
        egui::Align2::CENTER_CENTER,
        text,
        icon_font(14.0),
        fg,
    );

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
