use crate::i18n::{T, tr};
use crate::ui::components::{card::card, logo_button::logo_button, pill::pill};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::images::{self, Img};
use crate::ui::theme::*;
use egui::{Align2, CornerRadius, RichText, Sense, Stroke, StrokeKind, Ui, Vec2};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const URL_SITE: &str = "https://razeclicker.pw/";
const URL_EULA: &str = "https://razeclicker.pw/license";
const URL_DONATE: &str = "https://www.donationalerts.com/r/sorgarso";
const URL_GITHUB: &str = "https://github.com/Sorgarso/RazeClicker";
const URL_LICENSES: &str =
    "https://github.com/Sorgarso/RazeClicker/blob/main/THIRD-PARTY-LICENSES.md";

pub fn show(ui: &mut Ui) {
    card(ui, MARGIN_BASE, MARGIN_BASE, 16, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 22.0;

            images::show(ui, Img::Logo, egui::vec2(90.0, 90.0));

            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 6.0;
                ui.add_space(4.0);
                ui.label(RichText::new("RazeClicker").font(regular(26.0)).color(TEXT));
                pill(ui, format!("v{VERSION}"), ACCENT);
                ui.label(RichText::new(tr(T::AboutText)).size(14.0).color(MUTED));
            });
        });
    });

    ui.add_space(14.0);

    let gap = 14.0;
    let w = (ui.available_width() - gap) / 2.0;
    let size = Vec2::new(w, 56.0);

    let items = [
        (ICON_FA_GLOBE, tr(T::Website), URL_SITE),
        (ICON_FA_FILE_CONTRACT, tr(T::Eula), URL_EULA),
        (ICON_FA_HEART, tr(T::Donate), URL_DONATE),
        (ICON_FA_CODE, "GitHub", URL_GITHUB),
    ];

    for row in items.chunks(2) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for (icon, text, url) in row {
                if logo_button(ui, WIDGET, WIDGET_HOVER, 15, text, Some(&icon), size).clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(*url));
                }
            }
        });
        ui.add_space(2.0);
    }

    let full = Vec2::new(ui.available_width(), 44.0);
    if logo_button(
        ui,
        WIDGET,
        WIDGET_HOVER,
        15,
        tr(T::ThirdPartyLicenses),
        Some(ICON_FA_SCALE_BALANCED),
        full,
    )
    .clicked()
    {
        ui.ctx().open_url(egui::OpenUrl::new_tab(URL_LICENSES));
    }
}
