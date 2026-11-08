use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;

use egui::{Align, Layout, RichText, Ui, Vec2};

pub fn row_label(ui: &mut Ui, text: &str) {
    ui.allocate_ui_with_layout(
        Vec2::new(LABEL_W, ROW_H),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.label(RichText::new(text).font(regular(15.0)).color(MUTED));
        },
    );
}
