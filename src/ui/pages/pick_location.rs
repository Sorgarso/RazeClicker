use crate::i18n::{T, tr};
use crate::model::{key::key_name, settings::Settings};
use crate::platform::windows::hooks;
use crate::platform::windows::hooks::start_pick_location;
use crate::ui::components::logo_button::logo_button;
use crate::ui::components::toggle::toggle;
use crate::ui::components::{
    button, card::card, card_header::card_header, coord_row::coord_row, hotkey_input::hotkey_input,
    input_text::input_text, logo_button, outlined_button::outlined_button, radio_card::radio_card,
    row_label::row_label, segmented::segmented, toggle,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::icon_font;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui, Vec2};
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

const CARD_H: f32 = 260.0;

pub fn show(ui: &mut Ui, s: &Arc<Settings>) {
    let mut pick_location_enabled = s.pick_location_enabled.load(Relaxed);
    let points_list = s.pos.lock().unwrap().clone();
    ui.horizontal(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(tr(T::PickLocation))
                    .font(regular(26.0))
                    .color(TEXT),
            );
            ui.label(
                RichText::new(tr(T::PickLocationSub))
                    .size(14.0)
                    .color(MUTED),
            );
        });
    });

    ui.add_space(16.0);

    let gap = 14.0;
    let avail = ui.available_width() - gap;
    let w1 = avail / 1.75;
    let w2 = w1 * 0.75;

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        ui.spacing_mut().item_spacing.y = 14.0;

        ui.allocate_ui_with_layout(Vec2::new(w1, 220.0), Layout::top_down(Align::Min), |ui| {
            let card_size = Vec2::new(ui.available_width(), CARD_H);

            ui.allocate_ui(card_size, |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 1.0;
                    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
                        ui.label(
                            RichText::new(format!("{}: {}", tr(T::SavedPoints), points_list.len()))
                                .font(regular(18.0))
                                .strong()
                                .color(TEXT),
                        );
                        ui.add_space(17.0);
                        ui.separator();
                        if points_list.len() != 0 {
                            egui::ScrollArea::vertical()
                                .id_salt("saved_points_scroll")
                                .max_height(CARD_H)
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    for (i, pos) in s.pos.lock().unwrap().iter().enumerate() {
                                        coord_row(ui, i, pos.x, pos.y);
                                    }
                                });
                        } else {
                            ui.allocate_ui_with_layout(
                                ui.available_size(),
                                Layout::top_down(Align::Center),
                                |ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(tr(T::NoSavedPoints));
                                    });
                                },
                            );
                        }
                    });
                });
            });
        });

        ui.allocate_ui_with_layout(Vec2::new(w2, 250.0), Layout::top_down(Align::Min), |ui| {
            card(ui, 19, 10, 10, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 16.0;
                    row_label(ui, tr(T::EnablePickLocation));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        toggle(ui, &mut pick_location_enabled);
                    });
                });
            });
            if logo_button(
                ui,
                ACCENT,
                ACCENT_HOVER,
                10,
                tr(T::PickFromScreen),
                Some(ICON_FA_CROSSHAIRS),
                Vec2::new(280.0, 50.0),
            )
            .clicked()
                && pick_location_enabled
            {
                s.request_minimize.store(true, Relaxed);
                start_pick_location();
            }
            if outlined_button(
                ui,
                DANGER_IDLE,
                DANGER_HOVER,
                10,
                tr(T::ClearAll),
                Some(ICON_FA_TRASH),
                Vec2::new(ui.available_width(), 50.0),
            )
            .clicked()
            {
                s.pos.lock().unwrap().clear();
            }
        });
    });

    s.pick_location_enabled
        .store(pick_location_enabled, Relaxed);
}
