use crate::i18n::{T, tr};
use crate::ui::anim::page_reveal::staggered;
use crate::ui::components::{drift::drift, link_button::link_button, pill::pill};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::regular;
use crate::ui::images::{self, Img};
use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui, Vec2};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const URL_SITE: &str = "https://razeclicker.pw/";
const URL_EULA: &str = "https://razeclicker.pw/license";
const URL_DONATE: &str = "https://www.donationalerts.com/r/sorgarso";
const URL_GITHUB: &str = "https://github.com/Sorgarso/RazeClicker";
const URL_LICENSES: &str =
    "https://github.com/Sorgarso/RazeClicker/blob/main/THIRD-PARTY-LICENSES.md";

const COL_W: f32 = 520.0;
const GAP: f32 = 12.0;
const BTN_H: f32 = 48.0;
const HEAD_H: f32 = 190.0;
const GRID_H: f32 = BTN_H * 3.0 + GAP * 2.0;

pub fn show(ui: &mut Ui) {
    staggered(ui, 0, |ui| {
        ui.add_space(12.0);

        drift(
            ui,
            "about_head",
            Vec2::new(ui.available_width(), HEAD_H),
            0.6,
            Layout::top_down(Align::Center),
            |ui| {
                ui.vertical_centered(|ui| {
                    images::show(ui, Img::Logo, egui::vec2(84.0, 84.0));
                    ui.add_space(12.0);
                    ui.label(RichText::new("RazeClicker").font(regular(30.0)).color(TEXT));
                    ui.add_space(8.0);
                    pill(ui, format!("v{VERSION}"), ACCENT);
                    ui.add_space(8.0);
                    ui.label(RichText::new(tr(T::AboutText)).size(13.0).color(MUTED));
                });
            },
        );

        ui.add_space(32.0);

        let w = ui.available_width().min(COL_W);
        let pad = ((ui.available_width() - w) / 2.0).max(0.0);
        let half = (w - GAP) / 2.0;

        let items = [
            (ICON_FA_GLOBE, tr(T::Website), URL_SITE),
            (ICON_FA_FILE_CONTRACT, tr(T::Eula), URL_EULA),
            (ICON_FA_HEART, tr(T::Donate), URL_DONATE),
            (ICON_FA_CODE, "GitHub", URL_GITHUB),
        ];

        ui.horizontal_top(|ui| {
            ui.add_space(pad);
            drift(
                ui,
                "about_grid",
                Vec2::new(w, GRID_H),
                1.0,
                Layout::top_down(Align::Min),
                |ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(GAP, GAP);

                    for row in items.chunks(2) {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = GAP;
                            for (icon, text, url) in row {
                                if link_button(ui, icon, text, Vec2::new(half, BTN_H)).clicked() {
                                    ui.ctx().open_url(egui::OpenUrl::new_tab(*url));
                                }
                            }
                        });
                    }

                    if link_button(
                        ui,
                        ICON_FA_SCALE_BALANCED,
                        tr(T::ThirdPartyLicenses),
                        Vec2::new(w, BTN_H),
                    )
                    .clicked()
                    {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(URL_LICENSES));
                    }
                },
            );
        });
    });
}
