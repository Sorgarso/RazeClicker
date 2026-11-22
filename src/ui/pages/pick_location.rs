use crate::i18n::{T, tr};
use crate::model::settings::Settings;
use crate::platform::windows::hooks::start_pick_location;
use crate::ui::anim::page_reveal::staggered;
use crate::ui::components::{
    card::card, coord_row::coord_row, drift::drift, logo_button::logo_button,
    outlined_button::outlined_button, page_title::page_title, toggle::toggle,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui, Vec2};
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

const CARD_H: f32 = 240.0;
const LEFT_W: f32 = 360.0;
const RIGHT_W: f32 = 290.0;
const GAP: f32 = 28.0;
const BTN_H: f32 = 48.0;
const TOGGLE_H: f32 = 30.0;
const V_GAP: f32 = 14.0;

pub fn show(ui: &mut Ui, s: &Arc<Settings>) {
    let mut pick_enabled = s.pick_location_enabled.load(Relaxed);
    let points = s.pos.lock().unwrap().clone();

    staggered(ui, 0, |ui| {
        page_title(
            ui,
            tr(T::PickLocationEyebrow),
            tr(T::PickLocation),
            tr(T::PickLocationSub),
        );
    });

    ui.add_space(52.0);

    let avail = ui.available_width();
    let k = ((avail - GAP) / (LEFT_W + RIGHT_W)).min(1.0);
    let lw = LEFT_W * k;
    let rw = RIGHT_W * k;
    let pad = ((avail - (lw + rw + GAP)) / 2.0).max(0.0);

    let m = MARGIN_BASE as f32 * 2.0;
    let left_h = CARD_H + m;
    let right_h = (TOGGLE_H + m) + V_GAP + BTN_H + V_GAP + BTN_H;

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = GAP;
        ui.add_space(pad);

        drift(
            ui,
            "pick_left",
            Vec2::new(lw, left_h),
            1.0,
            Layout::top_down(Align::Min),
            |ui| points_card(ui, &points, lw),
        );

        drift(
            ui,
            "pick_right",
            Vec2::new(rw, right_h),
            1.0,
            Layout::top_down(Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing.y = V_GAP;

                card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
                    ui.set_width(rw - m);
                    ui.set_height(TOGGLE_H);
                    ui.horizontal_centered(|ui| {
                        ui.label(
                            RichText::new(tr(T::EnablePickLocation))
                                .font(regular(14.0))
                                .color(MUTED),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            toggle(ui, &mut pick_enabled);
                        });
                    });
                });

                if logo_button(
                    ui,
                    ACCENT,
                    ACCENT_HOVER,
                    12,
                    tr(T::PickFromScreen),
                    Some(ICON_FA_CROSSHAIRS),
                    Vec2::new(rw, BTN_H),
                )
                .clicked()
                    && pick_enabled
                {
                    s.request_minimize.store(true, Relaxed);
                    start_pick_location();
                }

                if logo_button(
                    ui,
                    DANGER_IDLE,
                    DANGER_HOVER,
                    12,
                    tr(T::ClearAll),
                    Some(ICON_FA_TRASH),
                    Vec2::new(rw, BTN_H),
                )
                .clicked()
                {
                    s.pos.lock().unwrap().clear();
                }
            },
        );
    });

    if pick_enabled != s.pick_location_enabled.load(Relaxed) {
        s.pick_location_enabled.store(pick_enabled, Relaxed);
    }

    ui.add_space(16.0);
}

fn points_card(ui: &mut Ui, points: &[egui::Pos2], w: f32) {
    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 10.0);
        ui.set_width(w - MARGIN_BASE as f32 * 2.0);
        ui.set_height(CARD_H);

        ui.label(
            RichText::new(format!("{}: {}", tr(T::SavedPoints), points.len()))
                .font(regular(17.0))
                .color(TEXT),
        );
        ui.separator();

        if points.is_empty() {
            let h = ui.available_height();
            ui.allocate_ui_with_layout(
                Vec2::new(ui.available_width(), h),
                Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.label(
                        RichText::new(tr(T::NoSavedPoints))
                            .font(regular(13.0))
                            .color(MUTED),
                    );
                },
            );
        } else {
            egui::ScrollArea::vertical()
                .id_salt("saved_points_scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (i, pos) in points.iter().enumerate() {
                        coord_row(ui, i, pos.x, pos.y);
                    }
                });
        }
    });
}
