use egui::{CornerRadius, Frame, Margin, RichText, vec2};

use crate::{
    i18n::{T, tr},
    ui::{
        components::{
            font_button::font_button,
            header_button::{close_button::close_button, minimize_button::minimize_button},
            tooltip_below::tooltip_below,
        },
        fonts::fa_data::ICON_FA_BUG,
        fonts::fonts::regular,
        images::{self, Img},
        state::UiState,
        theme::*,
    },
};

pub fn show(ui: &mut egui::Ui, st: &mut UiState) {
    egui::Panel::top("header")
        .exact_size(64.0)
        .resizable(false)
        .show_separator_line(true)
        .frame(
            Frame::new()
                .fill(egui::Color32::TRANSPARENT)
                .inner_margin(Margin::same(14))
                .corner_radius(CornerRadius {
                    ne: 2,
                    nw: 2,
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

            ui.horizontal(|ui| {
                images::show(ui, Img::Logo, egui::vec2(34.0, 34.0));
                ui.label(
                    RichText::new("raze")
                        .font(regular(17.0))
                        .strong()
                        .color(TEXT),
                );

                ui.label(
                    RichText::new("clicker")
                        .font(regular(17.0))
                        .strong()
                        .color(MUTED),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 5.0;
                    if close_button(ui).clicked() {
                        st.close = true;
                    }
                    if minimize_button(ui).clicked() {
                        st.minimize = true;
                    }
                    let resp =
                        font_button(ui, MIN_IDLE, MIN_HOVER, 8, ICON_FA_BUG, vec2(35.0, 35.0));
                    tooltip_below(ui, &resp, tr(T::ReportABug));
                    if resp.clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(
                            "https://www.razeclicker.pw/feedback",
                        ));
                    }
                });
            });
        });
}
