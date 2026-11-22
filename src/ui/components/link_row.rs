use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align2, Color32, Sense, Stroke, Ui, pos2, vec2};

pub fn link_row(ui: &mut Ui, label: &str, value: &str, divider: bool) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 40.0), Sense::click());
    let t = ui
        .ctx()
        .animate_bool_with_time(resp.id.with("hover"), resp.hovered(), 0.15);

    let col = START_PINK.lerp_to_gamma(Color32::WHITE, t * 0.5);
    let p = ui.painter();

    p.text(
        rect.left_center(),
        Align2::LEFT_CENTER,
        label,
        regular(13.0),
        MUTED,
    );

    let g = p.layout_no_wrap(value.to_owned(), regular(13.0), col);
    let ax = rect.right() - 5.0 + t * 1.5;
    let ay = rect.center().y - t * 1.5;
    let tx = rect.right() - 20.0 - g.size().x;
    p.galley(pos2(tx, rect.center().y - g.size().y / 2.0), g, col);

    let s = Stroke::new(1.3, col);
    p.line_segment([pos2(ax - 3.5, ay + 3.5), pos2(ax + 3.5, ay - 3.5)], s);
    p.line_segment([pos2(ax - 1.0, ay - 3.5), pos2(ax + 3.5, ay - 3.5)], s);
    p.line_segment([pos2(ax + 3.5, ay - 3.5), pos2(ax + 3.5, ay + 1.0)], s);

    if divider {
        p.line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 14)),
        );
    }

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp.clicked()
}
