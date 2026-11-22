mod changelog_dialog;
mod header;
mod update_dialog;

use crate::i18n::{T, tr};
use crate::ui::anim::page_reveal::page_started;
use crate::ui::components::ambient::ambient;
use crate::ui::components::nav_pill::nav_pill;
use crate::ui::components::window_border::window_border;
use crate::ui::fonts::fa_data::*;
use crate::ui::{
    pages,
    state::{Page, UiState},
    theme::*,
};
use egui::{CornerRadius, Frame, Margin, RichText};
use std::sync::atomic::Ordering::Relaxed;

pub fn draw(ui: &mut egui::Ui, st: &mut UiState) {
    ui.ctx().data_mut(|d| {
        d.insert_temp(
            egui::Id::new("parallax_on"),
            st.settings.parallax.load(Relaxed),
        );
    });
    window_border(ui.ctx());
    ambient(
        ui,
        st.settings.ambient.load(Relaxed),
        st.settings.enable_clicker.load(Relaxed),
    );
    header::show(ui, st);

    egui::Panel::bottom("nav")
        .exact_size(70.0)
        .resizable(false)
        .show_separator_line(false)
        .frame(
            Frame::new()
                .fill(egui::Color32::TRANSPARENT)
                .inner_margin(Margin {
                    left: 10,
                    right: 10,
                    top: 4,
                    bottom: 14,
                })
                .corner_radius(CornerRadius {
                    se: 25,
                    sw: 25,
                    ..Default::default()
                }),
        )
        .show_inside(ui, |ui| {
            let items = [
                (Page::Clicker, ICON_FA_CLOCK, tr(T::Clicker)),
                (Page::Advanced, ICON_FA_SLIDERS, tr(T::Advanced)),
                (
                    Page::PickLocation,
                    ICON_FA_LOCATION_DOT,
                    tr(T::PickLocation),
                ),
                (Page::Recorder, ICON_FA_CIRCLE_DOT, tr(T::Recorder)),
                (Page::Settings, ICON_FA_GEAR, tr(T::Settings)),
                (Page::About, ICON_FA_CIRCLE_INFO, tr(T::About)),
            ];
            nav_pill(ui, &mut st.page, &items);
        });

    page_started(ui.ctx(), st.page);

    egui::CentralPanel::default()
        .frame(
            Frame::new()
                .fill(egui::Color32::TRANSPARENT)
                .inner_margin(Margin {
                    left: 20,
                    right: 20,
                    top: 8,
                    bottom: 8,
                }),
        )
        .show_inside(ui, |ui| {
            match st.page {
                Page::Clicker => pages::clicker::show(ui, &st.settings),
                Page::Advanced => pages::advanced::show(ui, &st.settings),
                Page::PickLocation => pages::pick_location::show(ui, &st.settings),
                Page::Recorder => pages::recorder::show(ui, &st.settings),
                Page::Settings => pages::settings::show(ui, &st.settings, &mut st.page),
                Page::About => pages::about::show(ui),

                _ => {
                    ui.label(RichText::new("Not implemented").color(MUTED));
                }
            }

            let status = st.updater.status();
            changelog_dialog::changelog_dialog(ui, st);
            update_dialog::update_dialog(ui, status, st);
        });
}
