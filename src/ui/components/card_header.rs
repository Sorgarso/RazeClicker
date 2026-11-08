use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub fn card_header(ui: &mut Ui, icon: &str, title: &str, subtitle: &str) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 12.0;
        ui.label(RichText::new(icon).font(icon_font(22.0)).color(ICON));
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(RichText::new(title).font(regular(16.0)).color(TEXT));
            ui.label(RichText::new(subtitle).size(12.0).color(MUTED));
        });
    });
}
