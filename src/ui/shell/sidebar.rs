use egui::{Color32, CornerRadius, Frame, Margin, RichText};

use crate::{
    i18n::{Lang, T, lang, set_lang, tr},
    ui::{
        components::side_button::side_panel_button,
        fonts::{fa_data::*, fonts::regular},
        images::{self, Img},
        state::{Page, UiState},
        theme::*,
    },
    update::state::UpdateStatus,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn items() -> [(Page, &'static str, &'static str); 3] {
    [
        (Page::Clicker, ICON_FA_COMPUTER_MOUSE, tr(T::Clicker)),
        (
            Page::PickLocation,
            ICON_FA_MAP_LOCATION_DOT,
            tr(T::PickLocation),
        ),
        (Page::Recorder, ICON_FA_RECORD_VINYL, tr(T::Recorder)),
    ]
}
fn bottom_items() -> [(Page, &'static str, &'static str); 2] {
    [
        (Page::About, ICON_FA_CIRCLE_INFO, tr(T::About)),
        (Page::Settings, ICON_FA_GEAR, tr(T::Settings)),
    ]
}
pub fn show(ui: &mut egui::Ui, st: &mut UiState) {
    egui::Panel::left("side")
        .exact_size(180.0)
        .resizable(false)
        .show_separator_line(false)
        .frame(
            Frame::new()
                .fill(SIDEBAR)
                .inner_margin(Margin::same(16))
                .corner_radius(CornerRadius {
                    nw: 20,
                    sw: 20,
                    ne: 0,
                    se: 0,
                }),
        )
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                images::show(ui, Img::Logo, egui::vec2(90.0, 90.0));
                ui.label(RichText::new("RazeClicker").font(regular(20.0)));
            });
            ui.add_space(12.0);
            ui.spacing_mut().item_spacing.y = 4.0;

            for (page, icon, name) in items() {
                if side_panel_button(ui, Some(icon), name, st.page == page).clicked() {
                    st.page = page;
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.spacing_mut().item_spacing.y = 4.0;
                for (page, icon, name) in bottom_items() {
                    if side_panel_button(ui, Some(icon), name, st.page == page).clicked() {
                        st.page = page;
                    }
                }
            });
        });
}
