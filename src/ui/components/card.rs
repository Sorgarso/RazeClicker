use crate::ui::anim::page_reveal::staggered;
use crate::ui::theme::*;
use egui::{CornerRadius, Frame, Margin, Stroke, Ui};

pub fn card(
    ui: &mut Ui,
    margin_x: i8,
    margin_y: i8,
    corner: u8,
    add_contents: impl FnOnce(&mut Ui),
) {
    staggered(ui, 0, |ui| {
        Frame::new()
            .fill(CARD)
            .corner_radius(CornerRadius::same(corner))
            .inner_margin(Margin::symmetric(margin_x, margin_y))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                add_contents(ui);
            });
    });
}
