use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::state::Page;
use crate::ui::theme::*;
use egui::{Align2, Color32, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

pub fn nav_pill(ui: &mut Ui, current: &mut Page, items: &[(Page, &str, &str)]) {
    let pad_x = 14.0;
    let icon_w = 14.0;
    let gap = 8.0;
    let item_gap = 4.0;
    let pill_pad = 6.0;
    let h = 38.0;
    let pill_h = h + pill_pad * 2.0;

    let widths: Vec<f32> = items
        .iter()
        .map(|(_, _, label)| {
            let g = ui
                .painter()
                .layout_no_wrap(label.to_string(), regular(13.0), MUTED);
            pad_x * 2.0 + icon_w + gap + g.size().x
        })
        .collect();

    let total = widths.iter().sum::<f32>()
        + item_gap * (items.len().saturating_sub(1)) as f32
        + pill_pad * 2.0;

    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), pill_h), Sense::hover());
    let pill = Rect::from_center_size(rect.center(), vec2(total, pill_h));

    ui.painter()
        .rect_filled(pill, 22, Color32::from_rgba_unmultiplied(255, 255, 255, 6));

    let mut x = pill.left() + pill_pad;
    for (i, (page, icon, label)) in items.iter().enumerate() {
        let w = widths[i];
        let r = Rect::from_min_size(pos2(x, pill.top() + pill_pad), vec2(w, h));
        let resp = ui.interact(r, ui.id().with(("nav_item", i)), Sense::click());

        let active = *current == *page;
        let t_act = ui
            .ctx()
            .animate_bool_with_time(resp.id.with("act"), active, 0.18);
        let t_hov = ui
            .ctx()
            .animate_bool_with_time(resp.id.with("hov"), resp.hovered(), 0.15);

        let fill_a = (t_act * 28.0 + t_hov * 10.0 * (1.0 - t_act)) as u8;
        let stroke_a = (t_act * 70.0) as u8;
        let p = ui.painter();
        p.rect_filled(
            r,
            16,
            Color32::from_rgba_unmultiplied(222, 163, 195, fill_a),
        );

        let color = MUTED.lerp_to_gamma(TEXT, t_act.max(t_hov));
        p.text(
            pos2(r.left() + pad_x + icon_w / 2.0, r.center().y),
            Align2::CENTER_CENTER,
            *icon,
            icon_font(14.0),
            color,
        );
        p.text(
            pos2(r.left() + pad_x + icon_w + gap, r.center().y),
            Align2::LEFT_CENTER,
            *label,
            regular(13.0),
            color,
        );

        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if resp.clicked() {
            *current = *page;
        }
        x += w + item_gap;
    }
}
