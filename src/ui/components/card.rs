use crate::ui::theme::*;
use egui::{CornerRadius, Frame, Margin, Stroke, Ui};

pub fn card(ui: &mut Ui, marginX: i8, marginY: i8, corner: u8, add_contents: impl FnOnce(&mut Ui)) {
    Frame::new()
        .fill(CARD)
        .stroke(Stroke::new(1.0, CARD_BORDER))
        .corner_radius(CornerRadius::same(corner))
        .inner_margin(Margin::symmetric(marginX, marginY))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add_contents(ui);
        });
}
