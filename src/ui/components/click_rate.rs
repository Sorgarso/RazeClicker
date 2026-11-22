use crate::i18n::{T, tr};
use crate::ui::components::segmented::segmented;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{RichText, Ui};

pub fn click_rate(ui: &mut Ui, cps: &mut f32, unit: &mut usize) {
    ui.label(RichText::new(tr(T::ClicksPer)).size(12.0).color(MUTED));

    ui.horizontal(|ui| {
        ui.scope(|ui| {
            ui.style_mut()
                .text_styles
                .insert(egui::TextStyle::Button, regular(15.0));
            ui.add_sized(
                [52.0, ROW_H],
                egui::DragValue::new(cps)
                    .range(1.0..=1000.0)
                    .speed(0.5)
                    .max_decimals(0),
            );
        });

        segmented(
            ui,
            unit,
            &[
                tr(T::UnitSec),
                tr(T::UnitMin),
                tr(T::UnitHour),
                tr(T::UnitDay),
            ],
            42.0,
        );
    });
}
