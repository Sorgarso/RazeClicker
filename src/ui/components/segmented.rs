use crate::ui::{anim::animate_ease::*, fonts::fonts::regular, theme::*};
use egui::{Align2, Color32, CornerRadius, Id, Rect, Sense, Ui, Vec2, pos2};

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_unmultiplied(
        l(a.r(), b.r()),
        l(a.g(), b.g()),
        l(a.b(), b.b()),
        l(a.a(), b.a()),
    )
}

pub fn segmented(ui: &mut Ui, value: &mut usize, items: &[&str], item_w: f32) {
    let gap = 5.0;
    let n = items.len();
    let total_w = item_w * n as f32 + gap * (n.saturating_sub(1)) as f32;

    let (full_rect, _) = ui.allocate_exact_size(Vec2::new(total_w, ROW_H), Sense::hover());
    let base_id = ui
        .id()
        .with(("segmented", full_rect.min.x as i32, full_rect.min.y as i32));
    let ctx = ui.ctx().clone();

    let item_rect = |i: usize| {
        let x = full_rect.min.x + i as f32 * (item_w + gap);
        Rect::from_min_size(pos2(x, full_rect.min.y), Vec2::new(item_w, ROW_H))
    };

    let mut hover_t = vec![0.0; n];
    let mut press = vec![0.0; n];
    for i in 0..n {
        let resp = ui.interact(item_rect(i), base_id.with(i), Sense::click());
        hover_t[i] = ctx.animate_bool_with_time(base_id.with(("hover", i)), resp.hovered(), 0.12);
        press[i] = ctx.animate_bool_with_time(
            base_id.with(("press", i)),
            resp.is_pointer_button_down_on(),
            0.08,
        );
        if resp.hovered() {
            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if resp.clicked() {
            *value = i;
        }
    }

    for i in 0..n {
        let rect = item_rect(i).shrink(press[i] * 1.5);
        let bg = lerp_color(WIDGET, WIDGET_HOVER, hover_t[i]);
        ui.painter().rect_filled(rect, CornerRadius::same(7), bg);
    }

    let target_x = item_rect(*value).min.x;
    let anim_x = animate_ease(&ctx, base_id.with("pill_x"), target_x, 0.3, ease_out_cubic);

    let shrink = press[*value] * 1.5;
    let pill =
        Rect::from_min_size(pos2(anim_x, full_rect.min.y), Vec2::new(item_w, ROW_H)).shrink(shrink);

    ui.painter()
        .rect_filled(pill, CornerRadius::same(7), ACCENT);

    for (i, name) in items.iter().enumerate() {
        let rect = item_rect(i);
        let shrunk = rect.shrink(press[i] * 1.5);
        let scale = shrunk.height() / rect.height();

        let cover = (1.0 - ((anim_x - rect.min.x).abs() / (item_w + gap))).clamp(0.0, 1.0);
        let color = lerp_color(MUTED, Color32::WHITE, cover);

        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            name,
            regular(14.0 * scale),
            color,
        );
    }
}
