use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui};

pub fn setting_item(
    ui: &mut Ui,
    icon: &str,
    title: &str,
    subtitle: &str,
    right: impl FnOnce(&mut Ui),
) {
    ui.horizontal(|ui| {
        ui.set_min_height(52.0);
        ui.spacing_mut().item_spacing.x = 16.0;

        ui.allocate_ui_with_layout(
            egui::vec2(30.0, 52.0),
            Layout::top_down(Align::Center),
            |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new(icon).font(icon_font(22.0)).color(ICON));
                });
            },
        );

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.add_space(6.0);
            ui.label(RichText::new(title).font(regular(16.0)).color(TEXT));
            ui.label(RichText::new(subtitle).size(12.0).color(MUTED));
        });

        ui.with_layout(Layout::right_to_left(Align::Center), right);
    });
}

/// Тонкая линия между строками внутри карточки.
pub fn row_divider(ui: &mut Ui) {
    ui.add_space(4.0);
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, CARD_BORDER);
    ui.add_space(4.0);
}
