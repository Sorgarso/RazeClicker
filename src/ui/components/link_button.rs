use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::theme::*;
use egui::{Align2, Color32, Response, Sense, Stroke, StrokeKind, Ui, Vec2, pos2};

pub fn link_button(ui: &mut Ui, icon: &str, text: &str, size: Vec2) -> Response {
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());

    let hover_t = ui
        .ctx()
        .animate_bool_with_time(resp.id.with("hover"), resp.hovered(), 0.15);
    let press_t = ui.ctx().animate_bool_with_time(
        resp.id.with("press"),
        resp.is_pointer_button_down_on(),
        0.1,
    );

    let r = rect.shrink(press_t * 1.5);
    let bg = WIDGET.lerp_to_gamma(WIDGET_HOVER, hover_t);
    let border = Color32::from_rgba_unmultiplied(222, 163, 195, (hover_t * 90.0) as u8);

    let p = ui.painter();
    p.rect_filled(r, 14, bg);

    let color = MUTED.lerp_to_gamma(TEXT, hover_t);
    let g = p.layout_no_wrap(text.to_owned(), regular(14.0), color);

    let icon_w = 16.0;
    let gap = 10.0;
    let total = icon_w + gap + g.size().x;
    let x = r.center().x - total / 2.0;

    p.text(
        pos2(x + icon_w / 2.0, r.center().y),
        Align2::CENTER_CENTER,
        icon,
        icon_font(15.0),
        color,
    );
    p.galley(
        pos2(x + icon_w + gap, r.center().y - g.size().y / 2.0),
        g,
        color,
    );

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
