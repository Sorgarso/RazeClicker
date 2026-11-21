use crate::ui::theme::START_PINK;
use egui::{Color32, Rect, Sense, Ui, pos2, vec2};
use std::time::Duration;

pub fn progress_bar(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 6.0), Sense::hover());
    let t = ui.input(|i| i.time) as f32;
    ui.ctx().request_repaint_after(Duration::from_millis(16));

    let p = ui.painter();
    p.rect_filled(rect, 3, Color32::from_rgba_unmultiplied(255, 255, 255, 14));

    let seg_w = rect.width() * 0.38;
    let x = rect.left() - seg_w + (rect.width() + seg_w) * ((t * 0.7).fract());
    let seg = Rect::from_min_max(pos2(x, rect.top()), pos2(x + seg_w, rect.bottom()));

    let clip = p.with_clip_rect(rect);
    let glow = Color32::from_rgba_unmultiplied(222, 163, 195, 60);
    clip.rect_filled(seg.expand2(vec2(0.0, 3.0)), 6, glow);
    clip.rect_filled(seg, 3, START_PINK);
}
