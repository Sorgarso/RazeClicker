use crate::ui::images::{self, Img};
use crate::ui::theme::*;
use egui::{vec2, CornerRadius, Image, Sense, Stroke, StrokeKind, Ui};

pub fn logo_tile(ui: &mut Ui, img: Img, size: f32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    ui.painter().rect(
        rect,
        CornerRadius::same((size * 0.25) as u8),
        WIDGET,
        Stroke::new(1.0, CARD_BORDER),
        StrokeKind::Inside,
    );

    let inner = rect.shrink(size * 0.22);
    let tex = images::texture(ui.ctx(), img);
    ui.put(inner, Image::new(&tex).fit_to_exact_size(inner.size()));
}
