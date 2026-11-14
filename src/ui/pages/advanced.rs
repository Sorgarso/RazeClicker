use crate::i18n::{T, tr};

use crate::model::{key::key_name, settings::Settings};
use crate::platform::windows::hooks;
use crate::platform::windows::hooks::start;
use crate::ui::Arc;
use crate::ui::components::card_header::card_header_reserve;
use crate::ui::components::{
    card::card, card_header::card_header, card_logo::card_logo, dim_scope::dim_scope,
    hotkey_input::hotkey_input, logo_button::logo_button, radio_card::radio_card,
    row_label::row_label, segmented::segmented, toggle::toggle,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::icon_font;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui, Vec2};
use std::sync::atomic::Ordering::Relaxed;

const CARD_H: f32 = 150.0;

pub fn show(ui: &mut Ui, s: &Arc<Settings>) {
    ui.horizontal(|ui| {
        card_logo(ui, ICON_FA_SLIDERS);
        ui.add_space(7.0);
        ui.vertical(|ui| {
            ui.add_space(3.0);
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.label(
                RichText::new(tr(T::Advanced))
                    .font(regular(26.0))
                    .color(TEXT),
            );
            ui.label(RichText::new(tr(T::AdvancedSub)).size(14.0).color(MUTED));
        });
    });

    ui.add_space(16.0);

    let gap = 16.0;
    let w = (ui.available_width() - gap) / 2.0;

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        ui.allocate_ui_with_layout(Vec2::new(w, 0.0), Layout::top_down(Align::Min), |ui| {
            jitter_card(ui, s);
        });
        ui.allocate_ui_with_layout(Vec2::new(w, 0.0), Layout::top_down(Align::Min), |ui| {
            click_limit_card(ui, s);
        });
    });
}

fn jitter_card(ui: &mut Ui, s: &Settings) {
    let cps = s.cps();
    let unit = s.unit.load(Relaxed);

    let mut enabled = s.jitter_enabled.load(Relaxed);
    let mut pct = s.jitter.load(Relaxed);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 10.0);

        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(ICON_FA_WAVE_SQUARE)
                        .font(icon_font(20.0))
                        .color(MUTED),
                );
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    ui.set_max_width((ui.available_width() - 64.0).max(80.0));
                    ui.label(RichText::new(tr(T::Jitter)).font(regular(18.0)).color(TEXT));
                    ui.add(
                        egui::Label::new(
                            RichText::new(tr(T::JitterSub))
                                .font(regular(12.0))
                                .color(MUTED),
                        )
                        .wrap(),
                    );
                });
            });

            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                let was = enabled;
                toggle(ui, &mut enabled);
                if enabled && !was && pct == 0 {
                    pct = 10;
                }
            });
        });

        dim_scope(ui, "jitter", enabled, |ui| {
            let amount = cps * pct as f32 / 100.0;
            let mut lo = cps - amount;
            let mut hi = cps + amount;

            ui.horizontal(|ui| {
                let content_w = 180.0;
                ui.add_space(((ui.available_width() - content_w) / 2.0).max(0.0));

                let lo_changed = ui
                    .add_sized(
                        [56.0, ROW_H],
                        egui::DragValue::new(&mut lo)
                            .range(1.0..=cps)
                            .speed(0.5)
                            .max_decimals(0),
                    )
                    .changed();
                ui.label(RichText::new("–").color(MUTED));
                let hi_changed = ui
                    .add_sized(
                        [56.0, ROW_H],
                        egui::DragValue::new(&mut hi)
                            .range(cps..=cps * 1.45)
                            .speed(0.5)
                            .max_decimals(0),
                    )
                    .changed();

                let units = [
                    tr(T::UnitSec),
                    tr(T::UnitMin),
                    tr(T::UnitHour),
                    tr(T::UnitDay),
                ];
                ui.label(
                    RichText::new(units[unit.min(3)].to_string())
                        .font(regular(12.0))
                        .color(MUTED),
                );

                if cps > 0.0 {
                    if lo_changed {
                        pct = (((cps - lo) / cps) * 100.0).round().clamp(0.0, 45.0) as usize;
                    } else if hi_changed {
                        pct = (((hi - cps) / cps) * 100.0).round().clamp(0.0, 45.0) as usize;
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = (ui.available_width() - 56.0).max(60.0);
                ui.add(egui::Slider::new(&mut pct, 0..=45).show_value(false));
                ui.label(
                    RichText::new(format!("±{:.1}", cps * pct as f32 / 100.0))
                        .font(regular(12.0))
                        .color(MUTED),
                );
            });
        });
    });

    if enabled != s.jitter_enabled.load(Relaxed) {
        s.jitter_enabled.store(enabled, Relaxed);
    }
    if pct != s.jitter.load(Relaxed) {
        s.jitter.store(pct, Relaxed);
    }
}

fn click_limit_card(ui: &mut Ui, s: &Arc<Settings>) {
    let mut on = s.click_limit_enabled.load(Relaxed);
    let mut limit = s.click_limit.load(Relaxed);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 12.0);

        ui.horizontal(|ui| {
            card_header_reserve(
                ui,
                ICON_FA_HASHTAG,
                tr(T::ClickLimit),
                tr(T::ClickLimitSub),
                64.0,
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                toggle(ui, &mut on);
            });
        });

        dim_scope(ui, "click_limit", on, |ui| {
            ui.horizontal(|ui| {
                row_label(ui, tr(T::StopAfter));
                ui.add_sized(
                    [90.0, ROW_H],
                    egui::DragValue::new(&mut limit)
                        .range(1..=1_000_000)
                        .speed(1.0),
                );
                row_label(ui, tr(T::ClicksLower));
            });
        });
    });

    if on != s.click_limit_enabled.load(Relaxed) {
        s.click_limit_enabled.store(on, Relaxed);
    }
    if limit != s.click_limit.load(Relaxed) {
        s.click_limit.store(limit, Relaxed);
    }
}
