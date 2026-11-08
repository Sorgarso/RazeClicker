use crate::ui::fonts::fonts::regular;
use egui::{vec2, Color32, CornerRadius, Sense, Ui};

pub fn pill(ui: &mut Ui, text: impl Into<String>, color: Color32) {
    let galley = ui
        .painter()
        .layout_no_wrap(text.into(), regular(13.0), color);
    let size = galley.size() + vec2(22.0, 8.0);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());

    ui.painter()
        .rect_filled(rect, CornerRadius::same(255), color.gamma_multiply(0.18));
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, color);
}
