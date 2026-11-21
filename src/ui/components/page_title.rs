use crate::ui::components::spaced::spaced;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub fn page_title(ui: &mut Ui, eyebrow: &str, title: &str, subtitle: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(6.0);
        ui.label(
            RichText::new(spaced(&eyebrow.to_uppercase()))
                .size(10.0)
                .color(EYEBROW_ORANGE),
        );
        ui.add_space(6.0);
        ui.label(RichText::new(title).font(regular(30.0)).color(TEXT));
        ui.add_space(4.0);
        ui.label(RichText::new(subtitle).size(13.0).color(MUTED));
    });
}
