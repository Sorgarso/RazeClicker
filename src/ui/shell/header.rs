use egui::{CornerRadius, Frame, Margin, Vec2};

use crate::{
    i18n::{lang, set_lang, tr, Lang, T},
    ui::{
        components::{
            font_button,
            header_button::{close_button::close_button, minimize_button::minimize_button},
        },
        fonts::fa_data::ICON_FA_LANGUAGE,
        state::UiState,
        theme::*,
    },
};

pub fn show(ui: &mut egui::Ui, st: &mut UiState) {
    egui::Panel::top("header")
        .exact_size(56.0)
        .resizable(false)
        .show_separator_line(false)
        .frame(
            Frame::new()
                .fill(BG)
                .inner_margin(Margin::same(10))
                .corner_radius(CornerRadius {
                    ne: 20,
                    ..Default::default()
                }),
        )
        .show_inside(ui, |ui| {
            let drag = ui.interact(
                ui.max_rect(),
                ui.id().with("titlebar"),
                egui::Sense::click_and_drag(),
            );
            if drag.drag_started_by(egui::PointerButton::Primary) {
                st.drag = true;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 5.0;
                if close_button(ui).clicked() {
                    st.close = true;
                }
                if minimize_button(ui).clicked() {
                    st.minimize = true;
                }
            });
        });
}
