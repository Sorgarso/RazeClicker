use crate::i18n::{T, tr};
use crate::model::{key::key_name, settings::Settings};
use crate::ui::Arc;
use crate::ui::anim::page_reveal::staggered;
use crate::ui::components::{
    bind_field::bind_field, card::card, card_header::card_header, click_rate::click_rate,
    drift::drift, form_row::form_row, page_title::page_title, radio_card::radio_card,
    segmented::segmented, start_button::start_button, status_line::status_line,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::theme::*;
use egui::{Align, Layout, Ui, Vec2};
use std::sync::atomic::Ordering::Relaxed;

const CARD_H: f32 = 205.0;
const START_D: f32 = 150.0;
const CENTER_W: f32 = START_D + 60.0;
const GAP: f32 = 44.0;
const SIDE_W: f32 = 270.0;

pub fn show(ui: &mut Ui, s: &Arc<Settings>) {
    let mut mode = s.mode.load(Relaxed);
    let mut enabled = s.enable_clicker.load(Relaxed);

    staggered(ui, 0, |ui| {
        page_title(
            ui,
            tr(T::ClickerEyebrow),
            tr(T::Clicker),
            tr(T::ClickerSubtitle),
        );
    });

    ui.add_space(52.0);

    let side = ((ui.available_width() - CENTER_W - GAP * 2.0) / 2.0).min(SIDE_W);
    let total = side * 2.0 + CENTER_W + GAP * 2.0;
    let pad = ((ui.available_width() - total) / 2.0).max(0.0);
    let column_h = CARD_H + MARGIN_BASE as f32 * 2.0;

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = GAP;
        ui.add_space(pad);

        ui.allocate_ui_with_layout(
            Vec2::new(side, column_h),
            Layout::top_down(Align::Min),
            |ui| {
                drift(
                    ui,
                    "left",
                    Vec2::new(side, column_h),
                    1.0,
                    Layout::top_down(Align::Min),
                    |ui| match mode {
                        0 => mouse_settings(ui, s, side),
                        _ => keyboard_settings(ui, s, side),
                    },
                );
            },
        );

        ui.allocate_ui_with_layout(
            Vec2::new(CENTER_W, column_h),
            Layout::top_down(Align::Center),
            |ui| {
                drift(
                    ui,
                    "center",
                    Vec2::new(CENTER_W, column_h),
                    0.5,
                    Layout::top_down(Align::Center),
                    |ui| {
                        let content_h = START_D + 60.0 + 26.0;
                        ui.add_space(((column_h - content_h) / 2.0).max(0.0));

                        let hint = if enabled {
                            key_name(s.hotkey.load(Relaxed)).to_string()
                        } else {
                            tr(T::ReadyWhenYouAre).to_string()
                        };

                        if start_button(
                            ui,
                            enabled,
                            if enabled { tr(T::Stop) } else { tr(T::Start) },
                            &hint,
                            START_D,
                        )
                        .clicked()
                        {
                            enabled = !enabled;
                        }

                        status_line(
                            ui,
                            enabled,
                            tr(T::CurrentlyStatus),
                            if enabled { tr(T::Running) } else { tr(T::Idle) },
                        );
                    },
                );
            },
        );

        ui.allocate_ui_with_layout(
            Vec2::new(side, column_h),
            Layout::top_down(Align::Min),
            |ui| {
                drift(
                    ui,
                    "right",
                    Vec2::new(side, column_h),
                    1.0,
                    Layout::top_down(Align::Min),
                    |ui| activation_card(ui, s, &mut mode, side),
                );
            },
        );
    });

    if mode != s.mode.load(Relaxed) {
        s.mode.store(mode, Relaxed);
    }
    if enabled != s.enable_clicker.load(Relaxed) {
        s.enable_clicker.store(enabled, Relaxed);
    }

    ui.add_space(16.0);
}

fn mouse_settings(ui: &mut Ui, s: &Arc<Settings>, w: f32) {
    let mut cps = s.cps();
    let mut unit = s.unit.load(Relaxed);
    let mut button = s.button.load(Relaxed);

    let capturing = s.capturing.load(Relaxed);
    let hotkey = s.hotkey.load(Relaxed);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 14.0);
        ui.set_width(w - MARGIN_BASE as f32 * 2.0);
        ui.set_height(CARD_H);

        card_header(
            ui,
            ICON_FA_GAUGE_HIGH,
            tr(T::ClickInterval),
            tr(T::ClickIntervalSub),
        );
        ui.add_space(2.0);

        click_rate(ui, &mut cps, &mut unit);

        form_row(ui, tr(T::MouseButton), |ui| {
            segmented(ui, &mut button, &[tr(T::Left), tr(T::Right)], 70.0);
        });

        form_row(ui, tr(T::Hotkey), |ui| {
            if bind_field(ui, &key_name(hotkey), capturing) {
                s.capturing.store(!capturing, Relaxed);
            }
        });
    });

    save_common_settings(s, cps, unit);
    if button != s.button.load(Relaxed) {
        s.button.store(button, Relaxed);
    }
}

fn keyboard_settings(ui: &mut Ui, s: &Arc<Settings>, w: f32) {
    let mut cps = s.cps();
    let mut unit = s.unit.load(Relaxed);

    let capturing = s.capturing.load(Relaxed);
    let key_capturing = s.capturing_key_button.load(Relaxed);
    let hotkey = s.hotkey.load(Relaxed);
    let key_button = s.key_button.load(Relaxed);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 14.0);
        ui.set_width(w - MARGIN_BASE as f32 * 2.0);
        ui.set_height(CARD_H);

        card_header(
            ui,
            ICON_FA_GAUGE_HIGH,
            tr(T::ClickInterval),
            tr(T::ClickIntervalSub),
        );
        ui.add_space(2.0);

        click_rate(ui, &mut cps, &mut unit);

        form_row(ui, tr(T::Hotkey), |ui| {
            if bind_field(ui, &key_name(hotkey), capturing) {
                s.capturing.store(!capturing, Relaxed);
            }
        });

        form_row(ui, tr(T::KeyboardKey), |ui| {
            if bind_field(ui, &key_name(key_button), key_capturing) {
                s.capturing_key_button.store(!key_capturing, Relaxed);
            }
        });
    });

    save_common_settings(s, cps, unit);
}

fn activation_card(ui: &mut Ui, s: &Arc<Settings>, mode: &mut usize, w: f32) {
    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 14.0);
        ui.set_width(w - MARGIN_BASE as f32 * 2.0);
        ui.set_height(CARD_H);

        card_header(
            ui,
            ICON_FA_CIRCLE_PLAY,
            tr(T::Activation),
            tr(T::ActivationSub),
        );
        ui.add_space(2.0);

        if radio_card(ui, *mode != 1, tr(T::MouseMode)).clicked() {
            *mode = 0;
        }
        if radio_card(ui, *mode == 1, tr(T::KeyboardMode)).clicked() {
            *mode = 1;
        }

        let mut toggle_mode = s.toggle.load(Relaxed) as usize;
        segmented(ui, &mut toggle_mode, &[tr(T::Hold), tr(T::Toggle)], 115.0);
        s.toggle.store(toggle_mode == 1, Relaxed);
    });
}

fn save_common_settings(s: &Settings, cps: f32, unit: usize) {
    if cps != s.cps() {
        s.set_cps(cps);
    }
    if unit != s.unit.load(Relaxed) {
        s.unit.store(unit, Relaxed);
    }
}
