use crate::ui::anim::animate_ease::{animate_ease, ease_out_cubic};
use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::theme::*;
use egui::{Align2, Color32, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

pub fn side_tabs(ui: &mut Ui, cur: &mut usize, items: &[(&str, &str)], w: f32) {
    let h = 42.0;
    let gap = 6.0;
    let step = h + gap;
    let n = items.len();
    let total = h * n as f32 + gap * n.saturating_sub(1) as f32;

    let (full, alloc) = ui.allocate_exact_size(vec2(w, total), Sense::hover());
    let id = alloc.id;
    let ctx = ui.ctx().clone();

    let slot =
        |i: usize| Rect::from_min_size(pos2(full.min.x, full.min.y + i as f32 * step), vec2(w, h));

    let mut hov = vec![0.0; n];
    for i in 0..n {
        let r = ui.interact(slot(i), id.with(i), Sense::click());
        hov[i] = ctx.animate_bool_with_time(id.with(("h", i)), r.hovered(), 0.15);
        if r.hovered() {
            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if r.clicked() {
            *cur = i;
        }
    }

    let rel = animate_ease(
        &ctx,
        id.with("pill"),
        *cur as f32 * step,
        0.28,
        ease_out_cubic,
    );
    let pill = Rect::from_min_size(pos2(full.min.x, full.min.y + rel), vec2(w, h));

    let p = ui.painter();
    for i in 0..n {
        let a = (hov[i] * 12.0) as u8;
        p.rect_filled(
            slot(i),
            12,
            Color32::from_rgba_unmultiplied(255, 255, 255, a),
        );
    }
    p.rect_filled(pill, 12, Color32::from_rgba_unmultiplied(222, 163, 195, 30));
    p.rect_stroke(
        pill,
        12,
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(222, 163, 195, 110)),
        StrokeKind::Inside,
    );

    for (i, (icon, label)) in items.iter().enumerate() {
        let r = slot(i);
        let cover = (1.0 - (rel - i as f32 * step).abs() / step).clamp(0.0, 1.0);
        let color = MUTED.lerp_to_gamma(TEXT, cover.max(hov[i] * 0.6));
        let icon_color = MUTED.lerp_to_gamma(START_PINK, cover);

        p.text(
            pos2(r.left() + 24.0, r.center().y),
            Align2::CENTER_CENTER,
            *icon,
            icon_font(14.0),
            icon_color,
        );
        p.text(
            pos2(r.left() + 46.0, r.center().y),
            Align2::LEFT_CENTER,
            *label,
            regular(14.0),
            color,
        );
    }
}
