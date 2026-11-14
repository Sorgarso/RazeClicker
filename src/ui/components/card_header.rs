use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub fn card_header(ui: &mut Ui, icon: &str, title: &str, subtitle: &str) {
    card_header_reserve(ui, icon, title, subtitle, 0.0);
}

pub fn card_header_reserve(ui: &mut Ui, icon: &str, title: &str, subtitle: &str, reserve: f32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 12.0;
        ui.label(RichText::new(icon).font(icon_font(22.0)).color(ICON));
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.set_max_width((ui.available_width() - reserve).max(80.0));
            ui.label(RichText::new(title).font(regular(16.0)).color(TEXT));
            ui.add(egui::Label::new(RichText::new(subtitle).size(12.0).color(MUTED)).wrap());
        });
    });
}
