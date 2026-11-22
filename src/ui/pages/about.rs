use crate::i18n::{T, tr};
use crate::ui::anim::page_reveal::staggered;
use crate::ui::components::{
    card::card, drift::drift, link_button::link_button, link_row::link_row, page_title::page_title,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::regular;
use crate::ui::images::{self, Img};
use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui, Vec2};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const URL_SITE: &str = "https://razeclicker.pw/";
const URL_SUPPORT: &str = "https://www.razeclicker.pw/feedback";
const URL_EULA: &str = "https://razeclicker.pw/license";
const URL_DONATE: &str = "https://www.donationalerts.com/r/sorgarso";
const URL_GITHUB: &str = "https://github.com/Sorgarso/RazeClicker";
const URL_LICENSES: &str =
    "https://github.com/Sorgarso/RazeClicker/blob/main/THIRD-PARTY-LICENSES.md";

const CARD_W: f32 = 372.0;
const CARD_H: f32 = 160.0;
const GAP: f32 = 16.0;
const CHIP_H: f32 = 42.0;
const CHIP_GAP: f32 = 10.0;

pub fn show(ui: &mut Ui) {
    staggered(ui, 0, |ui| {
        page_title(ui, tr(T::AboutEyebrow), tr(T::AboutTitle), tr(T::AboutSub));
    });

    ui.add_space(34.0);

    let avail = ui.available_width();
    let w = ((avail - GAP) / 2.0).min(CARD_W);
    let total = w * 2.0 + GAP;
    let pad = ((avail - total) / 2.0).max(0.0);
    let m = MARGIN_BASE as f32 * 2.0;
    let size = Vec2::new(w, CARD_H + m);

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = GAP;
        ui.add_space(pad);

        drift(
            ui,
            "about_app",
            size,
            1.0,
            Layout::top_down(Align::Min),
            |ui| {
                card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
                    ui.set_width(w - m);
                    ui.set_height(CARD_H);
                    ui.spacing_mut().item_spacing.y = 0.0;

                    images::show(ui, Img::Logo, egui::vec2(48.0, 48.0));
                    ui.add_space(10.0);
                    ui.label(RichText::new("RazeClicker").font(regular(17.0)).color(TEXT));
                    ui.add_space(5.0);
                    ui.add(
                        egui::Label::new(
                            RichText::new(tr(T::AboutTagline))
                                .font(regular(12.0))
                                .color(MUTED),
                        )
                        .wrap(),
                    );

                    let free = (ui.available_height() - 16.0).max(0.0);
                    ui.add_space(free);
                    ui.label(
                        RichText::new(format!(
                            "{} {} · {}",
                            tr(T::Version),
                            VERSION,
                            tr(T::MadeWithCare)
                        ))
                        .font(regular(11.0))
                        .color(START_PINK),
                    );
                });
            },
        );

        drift(
            ui,
            "about_links",
            size,
            1.0,
            Layout::top_down(Align::Min),
            |ui| {
                card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
                    ui.set_width(w - m);
                    ui.set_height(CARD_H);
                    ui.spacing_mut().item_spacing.y = 0.0;

                    ui.label(
                        RichText::new(tr(T::MadeSimple))
                            .font(regular(17.0))
                            .color(TEXT),
                    );
                    ui.add_space(6.0);
                    ui.add(
                        egui::Label::new(
                            RichText::new(tr(T::MadeSimpleSub))
                                .font(regular(12.0))
                                .color(MUTED),
                        )
                        .wrap(),
                    );

                    let free = (ui.available_height() - 80.0).max(0.0);
                    ui.add_space(free);

                    if link_row(ui, tr(T::Website), "razeclicker.pw", true) {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(URL_SITE));
                    }
                    if link_row(ui, tr(T::Support), tr(T::GetInTouch), false) {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(URL_SUPPORT));
                    }
                });
            },
        );
    });

    ui.add_space(14.0);

    let chip_w = (total - CHIP_GAP * 3.0) / 4.0;
    let chips = [
        (ICON_FA_FILE_CONTRACT, tr(T::Eula), URL_EULA),
        (ICON_FA_HEART, tr(T::Donate), URL_DONATE),
        (ICON_FA_CODE, "GitHub", URL_GITHUB),
        (
            ICON_FA_SCALE_BALANCED,
            tr(T::ThirdPartyLicenses),
            URL_LICENSES,
        ),
    ];

    ui.horizontal_top(|ui| {
        ui.add_space(pad);
        drift(
            ui,
            "about_chips",
            Vec2::new(total, CHIP_H),
            0.8,
            Layout::left_to_right(Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing.x = CHIP_GAP;
                for (icon, text, url) in chips {
                    if link_button(ui, icon, text, Vec2::new(chip_w, CHIP_H)).clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    }
                }
            },
        );
    });

    ui.add_space(12.0);
}
