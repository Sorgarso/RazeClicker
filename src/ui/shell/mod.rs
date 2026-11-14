mod changelog_dialog;
mod header;
mod sidebar;
mod update_dialog;

use egui::{Color32, CornerRadius, Frame, Margin, RichText, Vec2};

use crate::{
    i18n::{T, tr},
    ui::{
        components::{logo_button::logo_button, model::modal},
        fonts::{
            fa_data::*,
            fonts::{icon_font, regular},
        },
        pages,
        shell::sidebar::VERSION,
        state::{Page, UiState},
        theme::*,
    },
    update::state::UpdateStatus,
};

pub fn draw(ui: &mut egui::Ui, st: &mut UiState) {
    sidebar::show(ui, st);
    header::show(ui, st);

    egui::CentralPanel::default()
        .frame(
            Frame::new()
                .fill(BG)
                .inner_margin(Margin::same(20))
                .corner_radius(CornerRadius {
                    se: 20,
                    ..Default::default()
                }),
        )
        .show_inside(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| match st.page {
                    Page::Clicker => pages::clicker::show(ui, &st.settings),
                    Page::Advanced => pages::advanced::show(ui, &st.settings),
                    Page::PickLocation => pages::pick_location::show(ui, &st.settings),
                    Page::Recorder => pages::recorder::show(ui, &st.settings),
                    Page::Settings => pages::settings::show(ui, &st.settings, &mut st.page),
                    Page::About => pages::about::show(ui),

                    _ => {
                        ui.label(RichText::new("Not implemented").color(MUTED));
                    }
                });
            let status = st.updater.status();

            changelog_dialog::changelog_dialog(ui, st);
            update_dialog::update_dialog(ui, status, st);
        });
}
