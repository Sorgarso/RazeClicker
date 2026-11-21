use egui::{Color32, Frame, Margin, RichText, Stroke};

use crate::{
    i18n::{T, tr},
    ui::{
        components::{
            logo_button::logo_button, model::modal, progress_bar::progress_bar,
            pulse_badge::pulse_badge, version_flow::version_flow,
        },
        fonts::{fa_data::*, fonts::regular},
        state::UiState,
        theme::*,
    },
    update::state::UpdateStatus,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const RED: Color32 = Color32::from_rgb(255, 95, 95);

pub fn update_dialog(ui: &mut egui::Ui, status: UpdateStatus, st: &mut UiState) {
    if !(st.show_dialog
        && matches!(
            status,
            UpdateStatus::Available { .. } | UpdateStatus::Downloading | UpdateStatus::Failed(_)
        ))
    {
        return;
    }

    modal(
        ui.ctx(),
        "update_dialog",
        &mut st.show_dialog,
        320.0,
        340.0,
        |ui| {
            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;

                match &status {
                    UpdateStatus::Available { version, .. } => {
                        pulse_badge(ui, ICON_FA_DOWNLOAD, START_PINK, false);
                        ui.label(
                            RichText::new(tr(T::UpdateAvailable))
                                .font(regular(21.0))
                                .color(TEXT),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(tr(T::UpdateSub))
                                .font(regular(12.0))
                                .color(MUTED),
                        );
                        ui.add_space(16.0);
                        version_flow(ui, &format!("v{VERSION}"), &format!("v{version}"));
                        ui.add_space(20.0);

                        if logo_button(
                            ui,
                            ACCENT,
                            ACCENT_HOVER,
                            14,
                            tr(T::Update),
                            Some(ICON_FA_DOWNLOAD),
                            egui::vec2(ui.available_width(), 48.0),
                        )
                        .clicked()
                        {
                            st.updater.install();
                        }
                    }

                    UpdateStatus::Downloading => {
                        pulse_badge(ui, ICON_FA_DOWNLOAD, START_PINK, true);
                        ui.label(
                            RichText::new(tr(T::Downloading))
                                .font(regular(21.0))
                                .color(TEXT),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(tr(T::PleaseWait))
                                .font(regular(12.0))
                                .color(MUTED),
                        );
                        ui.add_space(22.0);
                        progress_bar(ui);
                    }

                    UpdateStatus::Failed(error) => {
                        pulse_badge(ui, ICON_FA_TRIANGLE_EXCLAMATION, RED, false);
                        ui.label(
                            RichText::new(tr(T::UpdateFailed))
                                .font(regular(21.0))
                                .color(TEXT),
                        );
                        ui.add_space(14.0);

                        Frame::new()
                            .fill(Color32::from_rgba_unmultiplied(255, 95, 95, 14))
                            .stroke(Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(255, 95, 95, 70),
                            ))
                            .corner_radius(10)
                            .inner_margin(Margin::same(10))
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(error).font(regular(12.0)).color(MUTED),
                                    )
                                    .wrap(),
                                );
                            });

                        ui.add_space(16.0);

                        if logo_button(
                            ui,
                            ACCENT,
                            ACCENT_HOVER,
                            14,
                            tr(T::TryAgain),
                            Some(ICON_FA_ARROWS_ROTATE),
                            egui::vec2(ui.available_width(), 44.0),
                        )
                        .clicked()
                        {
                            st.updater.install();
                        }
                    }

                    _ => {}
                }
            });
        },
    );
}
