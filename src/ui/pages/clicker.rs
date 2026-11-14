use crate::i18n::{T, tr};

use crate::model::{key::key_name, settings::Settings};
use crate::platform::windows::hooks;
use crate::platform::windows::hooks::start;
use crate::ui::Arc;
use crate::ui::components::{
    card::card, card_header::card_header, card_logo::card_logo, dim_scope::dim_scope,
    hotkey_input::hotkey_input, radio_card::radio_card, row_label::row_label, segmented::segmented,
    toggle::toggle,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::icon_font;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui, Vec2};
use std::sync::atomic::Ordering::Relaxed;

const CARD_H: f32 = 190.0;

pub fn show(ui: &mut Ui, s: &Arc<Settings>) {
    let mut mode = s.mode.load(Relaxed);
    let mut enable_clicker = s.enable_clicker.load(Relaxed);

    ui.horizontal(|ui| {
        card_logo(ui, ICON_FA_COMPUTER_MOUSE);
        ui.add_space(7.0);
        ui.vertical(|ui| {
            ui.add_space(3.0);
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.label(
                RichText::new(tr(T::Clicker))
                    .font(regular(26.0))
                    .color(TEXT),
            );
            ui.label(
                RichText::new(tr(T::MouseClickerSub))
                    .size(14.0)
                    .color(MUTED),
            );
        });
    });

    ui.add_space(16.0);

    let gap = 16.0;
    let avail = ui.available_width() - gap;
    let w1 = avail / 1.75;
    let w2 = w1 * 0.75;

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        ui.allocate_ui_with_layout(
            Vec2::new(w1, 0.0),
            Layout::top_down(Align::Min),
            |ui| match mode {
                0 => mouse_settings(ui, s, &mut enable_clicker),
                1 => keyboard_settings(ui, s, &mut enable_clicker),
                _ => {}
            },
        );

        ui.allocate_ui_with_layout(Vec2::new(w2, 0.0), Layout::top_down(Align::Min), |ui| {
            card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(10.0, 10.0);
                ui.set_min_height(CARD_H);

                dim_scope(ui, "mode", enable_clicker, |ui| {
                    card_header(ui, ICON_FA_CIRCLE_PLAY, tr(T::Mode), tr(T::ModeSub));
                    ui.add_space(4.0);

                    if radio_card(ui, mode != 1, tr(T::MouseMode)).clicked() {
                        mode = 0;
                    }
                    if radio_card(ui, mode == 1, tr(T::KeyboardMode)).clicked() {
                        mode = 1;
                    }
                    ui.horizontal_centered(|ui| {
                        let mut toggle_mode = s.toggle.load(Relaxed) as usize;
                        segmented(ui, &mut toggle_mode, &[tr(T::Hold), tr(T::Toggle)], 120.0);
                        s.toggle.store(toggle_mode == 1, Relaxed);
                    });
                });
            });
        });
    });

    if mode != s.mode.load(Relaxed) {
        s.mode.store(mode, Relaxed);
    }

    if enable_clicker != s.enable_clicker.load(Relaxed) {
        s.enable_clicker.store(enable_clicker, Relaxed);
    }

    ui.add_space(16.0);
}

fn mouse_settings(ui: &mut Ui, s: &Arc<Settings>, enable_clicker: &mut bool) {
    let mut cps = s.cps();
    let mut unit = s.unit.load(Relaxed);
    let mut button = s.button.load(Relaxed);

    let capturing = s.capturing.load(Relaxed);
    let hotkey = s.hotkey.load(Relaxed);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 12.0);
        ui.set_min_height(CARD_H);

        ui.horizontal(|ui| {
            dim_scope(ui, "header", *enable_clicker, |ui| {
                card_header(
                    ui,
                    ICON_FA_GAUGE_HIGH,
                    tr(T::ClickSettings),
                    tr(T::ClickSettingsSub),
                );
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                toggle(ui, enable_clicker);
            });
        });

        ui.add_space(4.0);

        dim_scope(ui, "body", *enable_clicker, |ui| {
            click_rate(ui, &mut cps, &mut unit);

            ui.horizontal(|ui| {
                row_label(ui, tr(T::MouseButton));
                segmented(ui, &mut button, &[tr(T::Left), tr(T::Right)], 64.0);
            });

            ui.horizontal(|ui| {
                row_label(ui, tr(T::Hotkey));
                if hotkey_input(ui, &key_name(hotkey), capturing) {
                    s.capturing.store(!capturing, Relaxed);
                }
            });
        });
    });

    save_common_settings(s, cps, unit);

    if button != s.button.load(Relaxed) {
        s.button.store(button, Relaxed);
    }
}

fn keyboard_settings(ui: &mut Ui, s: &Arc<Settings>, enable_clicker: &mut bool) {
    let mut cps = s.cps();
    let mut unit = s.unit.load(Relaxed);

    let capturing = s.capturing.load(Relaxed);
    let key_capturing = s.capturing_key_button.load(Relaxed);
    let hotkey = s.hotkey.load(Relaxed);
    let key_button = s.key_button.load(Relaxed);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 12.0);
        ui.set_min_height(CARD_H);
        ui.horizontal(|ui| {
            dim_scope(ui, "header", *enable_clicker, |ui| {
                card_header(
                    ui,
                    ICON_FA_GAUGE_HIGH,
                    tr(T::ClickSettings),
                    tr(T::ClickSettingsSub),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                toggle(ui, enable_clicker);
            });
        });

        ui.add_space(4.0);

        dim_scope(ui, "body", *enable_clicker, |ui| {
            click_rate(ui, &mut cps, &mut unit);
            ui.horizontal(|ui| {
                row_label(ui, tr(T::Hotkey));
                if hotkey_input(ui, &key_name(hotkey), capturing) {
                    s.capturing.store(!capturing, Relaxed);
                }
            });
            ui.horizontal(|ui| {
                row_label(ui, tr(T::KeyboardKey));
                if hotkey_input(ui, &key_name(key_button), key_capturing) {
                    s.capturing_key_button.store(!key_capturing, Relaxed);
                }
            });
        });
    });

    save_common_settings(s, cps, unit);
}

fn click_rate(ui: &mut Ui, cps: &mut f32, unit: &mut usize) {
    ui.horizontal(|ui| {
        row_label(ui, tr(T::Clicks));

        ui.scope(|ui| {
            ui.style_mut()
                .text_styles
                .insert(egui::TextStyle::Button, regular(15.0));

            ui.add_sized(
                [90.0, ROW_H],
                egui::DragValue::new(cps)
                    .range(1.0..=1000.0)
                    .speed(0.5)
                    .max_decimals(0),
            );
        });

        segmented(
            ui,
            unit,
            &[
                tr(T::UnitSec),
                tr(T::UnitMin),
                tr(T::UnitHour),
                tr(T::UnitDay),
            ],
            40.0,
        );
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
