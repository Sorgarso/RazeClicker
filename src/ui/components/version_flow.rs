use crate::ui::fonts::fa_data::ICON_FA_ARROW_RIGHT;
use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::theme::*;
use egui::{Align2, Color32, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

pub fn version_flow(ui: &mut Ui, from: &str, to: &str) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::hover());
    let p = ui.painter();

    let g1 = p.layout_no_wrap(from.to_owned(), regular(13.0), MUTED);
    let g2 = p.layout_no_wrap(to.to_owned(), regular(13.0), START_PINK);
    let pad = 14.0;
    let w1 = g1.size().x + pad * 2.0;
    let w2 = g2.size().x + pad * 2.0;
    let arrow = 26.0;
    let total = w1 + arrow + w2;
    let x0 = rect.center().x - total / 2.0;
    let cy = rect.center().y;

    let r1 = Rect::from_min_size(pos2(x0, cy - 15.0), vec2(w1, 30.0));
    let r2 = Rect::from_min_size(pos2(x0 + w1 + arrow, cy - 15.0), vec2(w2, 30.0));

    p.rect_filled(r1, 10, Color32::from_rgba_unmultiplied(255, 255, 255, 8));
    p.rect_stroke(
        r1,
        10,
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 18)),
        StrokeKind::Inside,
    );
    p.rect_filled(r2, 10, Color32::from_rgba_unmultiplied(222, 163, 195, 30));
    p.rect_stroke(
        r2,
        10,
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(222, 163, 195, 120)),
        StrokeKind::Inside,
    );

    p.galley(pos2(r1.left() + pad, cy - g1.size().y / 2.0), g1, MUTED);
    p.galley(
        pos2(r2.left() + pad, cy - g2.size().y / 2.0),
        g2,
        START_PINK,
    );
    p.text(
        pos2(x0 + w1 + arrow / 2.0, cy),
        Align2::CENTER_CENTER,
        ICON_FA_ARROW_RIGHT,
        icon_font(12.0),
        START_PINK,
    );
}
