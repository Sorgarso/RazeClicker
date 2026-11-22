use crate::ui::anim::animate_ease::ease_out_cubic;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::TEXT;
use egui::{Area, Color32, Frame, Id, Margin, Order, Pos2, RichText, Shadow, Shape, Stroke, pos2};

const FILL: Color32 = Color32::from_rgb(24, 22, 30);

pub fn tooltip_below(ui: &egui::Ui, resp: &egui::Response, text: &str) {
    let id = Id::new(("tip", resp.id));
    let ctx = ui.ctx();

    let t = ctx.animate_bool_with_time(id.with("vis"), resp.contains_pointer(), 0.16);
    if t <= 0.001 {
        return;
    }
    let e = ease_out_cubic(t);

    let pos = resp.rect.center_bottom() + egui::vec2(0.0, 11.0 - 5.0 * (1.0 - e));

    let area = Area::new(id)
        .order(Order::Tooltip)
        .pivot(egui::Align2::CENTER_TOP)
        .fixed_pos(pos)
        .constrain(true)
        .interactable(false)
        .show(ctx, |ui| {
            ui.set_opacity(e);
            Frame::new()
                .fill(FILL)
                .stroke(Stroke::new(
                    1.0,
                    Color32::from_rgba_unmultiplied(222, 163, 195, 90),
                ))
                .corner_radius(9)
                .inner_margin(Margin::symmetric(11, 6))
                .shadow(Shadow {
                    offset: [0, 4],
                    blur: 16,
                    spread: 0,
                    color: Color32::from_rgba_unmultiplied(222, 163, 195, 34),
                })
                .show(ui, |ui| {
                    ui.label(RichText::new(text).font(regular(12.0)).color(TEXT));
                });
        });

    let r = area.response.rect;
    let cx = resp
        .rect
        .center()
        .x
        .clamp(r.left() + 14.0, r.right() - 14.0);
    let base = r.top() + 0.5;
    let a = (e * 255.0) as u8;
    let fill = Color32::from_rgba_unmultiplied(FILL.r(), FILL.g(), FILL.b(), a);
    let line = Color32::from_rgba_unmultiplied(222, 163, 195, (e * 90.0) as u8);

    let p = ctx.layer_painter(area.response.layer_id);
    let tip: Pos2 = pos2(cx, base - 5.0);
    let l: Pos2 = pos2(cx - 5.5, base);
    let rr: Pos2 = pos2(cx + 5.5, base);

    p.add(Shape::convex_polygon(vec![l, tip, rr], fill, Stroke::NONE));
    p.line_segment([l, tip], Stroke::new(1.0, line));
    p.line_segment([tip, rr], Stroke::new(1.0, line));
}
