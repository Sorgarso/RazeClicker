use egui::{Area, CornerRadius, Frame, Id, Margin, Order, RichText, Vec2, vec2};

use crate::{
    i18n::{Lang, T, lang, set_lang, tr},
    ui::{
        components::{
            font_button::font_button,
            header_button::{close_button::close_button, minimize_button::minimize_button},
            tooltip_below::tooltip_below,
        },
        fonts::fa_data::{ICON_FA_BUG, ICON_FA_LANGUAGE},
        fonts::fonts::regular,
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
                let resp = font_button(ui, MIN_IDLE, MIN_HOVER, 5, ICON_FA_BUG, vec2(35.0, 35.0));
                tooltip_below(ui, &resp, tr(T::ReportABug));
                if resp.clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(
                        "https://www.razeclicker.pw/feedback",
                    ));
                }
            });
        });
}
