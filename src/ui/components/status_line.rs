use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Color32, Sense, Ui, pos2, vec2};

pub fn status_line(ui: &mut Ui, active: bool, prefix: &str, status: &str) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 18.0), Sense::hover());
    let p = ui.painter();

    let g1 = p.layout_no_wrap(prefix.to_owned(), regular(12.0), MUTED);
    let g2 = p.layout_no_wrap(status.to_owned(), regular(12.0), TEXT);

    let dot = 6.0;
    let gap = 8.0;
    let word_gap = 5.0;
    let total = dot + gap + g1.size().x + word_gap + g2.size().x;

    let y = rect.center().y;
    let mut x = rect.center().x - total / 2.0;

    let dot_color = if active {
        START_PINK
    } else {
        Color32::from_gray(90)
    };
    p.circle_filled(pos2(x + dot / 2.0, y), dot / 2.0, dot_color);
    x += dot + gap;

    let w1 = g1.size().x;
    let h1 = g1.size().y;
    let h2 = g2.size().y;
    p.galley(pos2(x, y - h1 / 2.0), g1, MUTED);
    x += w1 + word_gap;
    p.galley(pos2(x, y - h2 / 2.0), g2, TEXT);
}
