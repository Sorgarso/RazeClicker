use crate::ui::theme::*;
use egui::{
    Color32, CornerRadius, FontFamily, FontId, Response, Stroke, StrokeKind, TextEdit, Ui, Vec2,
};

pub fn input_text(ui: &mut Ui, text: &mut String, hint: &str, width: f32, height: f32) -> Response {
    let size = Vec2::new(width, height);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(Vec2::new(10.0, 0.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let resp = child.add_sized(
        [rect.width() - 20.0, rect.height()],
        TextEdit::singleline(text)
            .frame(egui::Frame::NONE)
            .hint_text(egui::RichText::new(hint).color(MUTED))
            .font(FontId::new(15.0, FontFamily::Proportional))
            .text_color(TEXT)
            .vertical_align(egui::Align::Center),
    );

    let focused = resp.has_focus();
    let border = if focused {
        Stroke::new(1.5, ACCENT)
    } else if resp.hovered() {
        Stroke::new(1.0, ACCENT.gamma_multiply(0.5))
    } else {
        Stroke::new(1.0, CARD_BORDER)
    };
    let painter = ui.painter();
    painter.add(egui::Shape::rect_filled(
        rect,
        CornerRadius::same(8),
        Color32::TRANSPARENT,
    ));
    painter.rect_stroke(rect, CornerRadius::same(8), border, StrokeKind::Inside);

    resp
}
