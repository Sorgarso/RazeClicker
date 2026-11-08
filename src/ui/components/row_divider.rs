use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui};

pub fn row_divider(ui: &mut Ui) {
    ui.add_space(4.0);
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, CARD_BORDER);
    ui.add_space(4.0);
}
