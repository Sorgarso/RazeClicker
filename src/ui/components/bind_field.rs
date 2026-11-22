use crate::i18n::{T, tr};
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Color32, Sense, Stroke, StrokeKind, Ui, vec2};

pub fn bind_field(ui: &mut Ui, text: &str, capturing: bool) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), ROW_H), Sense::click());

    let hover_t = ui
        .ctx()
        .animate_bool_with_time(resp.id.with("hover"), resp.hovered(), 0.15);
    let cap_t = ui
        .ctx()
        .animate_bool_with_time(resp.id.with("cap"), capturing, 0.15);

    let bg = MIN_IDLE.lerp_to_gamma(MIN_HOVER, hover_t);
    let shown = if capturing { tr(T::PressKey) } else { text };
    let color = TEXT.lerp_to_gamma(START_PINK, cap_t);

    let max_w = rect.width() - 16.0;
    let mut size = 14.0;
    let mut galley = ui
        .painter()
        .layout_no_wrap(shown.to_owned(), regular(size), color);
    if galley.size().x > max_w {
        size = (size * max_w / galley.size().x).max(9.0);
        galley = ui
            .painter()
            .layout_no_wrap(shown.to_owned(), regular(size), color);
    }

    let p = ui.painter();
    p.rect_filled(rect, 8, bg);
    p.rect_stroke(rect, 8, Stroke::new(1.0, CARD_BORDER), StrokeKind::Inside);
    if cap_t > 0.0 {
        let a = (cap_t * 200.0) as u8;
        p.rect_stroke(
            rect,
            8,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(222, 163, 195, a)),
            StrokeKind::Inside,
        );
    }
    let clip = p.with_clip_rect(rect.shrink2(vec2(6.0, 0.0)));
    let pos = rect.center() - galley.size() / 2.0;
    clip.galley(pos, galley, color);

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp.clicked()
}
