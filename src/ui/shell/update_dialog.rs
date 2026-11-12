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

pub fn update_dialog(ui: &mut egui::Ui, status: UpdateStatus, st: &mut UiState) {
    if st.show_dialog
        && matches!(
            status,
            UpdateStatus::Available { .. } | UpdateStatus::Downloading | UpdateStatus::Failed(_)
        )
    {
        modal(
            ui.ctx(),
            "update_dialog",
            &mut st.show_dialog,
            250.0,
            300.0,
            |ui| match &status {
                UpdateStatus::Available { version, url } => {
                    ui.label(
                        RichText::new(ICON_FA_DOWNLOAD)
                            .font(icon_font(70.0))
                            .color(TEXT),
                    );

                    ui.label(
                        RichText::new(tr(T::UpdateAvailable))
                            .font(regular(20.0))
                            .color(TEXT),
                    );

                    ui.add_space(8.0);

                    ui.label(
                        RichText::new(format!("{} -> {}", VERSION, version))
                            .font(regular(14.0))
                            .color(MUTED),
                    );

                    ui.add_space(16.0);

                    if logo_button(
                        ui,
                        ACCENT,
                        ACCENT_HOVER,
                        10,
                        tr(T::Update),
                        Some(ICON_FA_DOWNLOAD),
                        Vec2::new(280.0, 50.0),
                    )
                    .clicked()
                    {
                        st.updater.install();
                    }
                }

                UpdateStatus::Downloading => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);

                        ui.spinner();

                        ui.add_space(16.0);

                        ui.label(
                            RichText::new(tr(T::Downloading))
                                .font(regular(20.0))
                                .color(TEXT),
                        );

                        ui.add_space(8.0);

                        ui.label(
                            RichText::new(tr(T::PleaseWait))
                                .font(regular(14.0))
                                .color(MUTED),
                        );

                        ui.add_space(20.0);
                    });
                }
                UpdateStatus::Failed(error) => {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new(ICON_FA_TRIANGLE_EXCLAMATION)
                                .font(icon_font(48.0))
                                .color(Color32::from_rgb(255, 85, 85)),
                        );

                        ui.add_space(12.0);

                        ui.label(
                            RichText::new(tr(T::UpdateFailed))
                                .font(regular(20.0))
                                .color(TEXT),
                        );

                        ui.add_space(8.0);

                        ui.label(RichText::new(error).font(regular(13.0)).color(MUTED));

                        ui.add_space(20.0);
                    });
                }
                _ => {}
            },
        );
    }
}
