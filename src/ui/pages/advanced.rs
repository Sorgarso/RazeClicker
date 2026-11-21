use crate::i18n::{T, tr};
use crate::model::settings::Settings;
use crate::ui::Arc;
use crate::ui::anim::page_reveal::staggered;
use crate::ui::components::{
    card::card, card_header::card_header_reserve, dim_scope::dim_scope, drift::drift,
    form_row::form_row, page_title::page_title, toggle::toggle,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui, Vec2};
use std::sync::atomic::Ordering::Relaxed;

const CARD_H: f32 = 150.0;
const CARD_W: f32 = 330.0;
const GAP: f32 = 28.0;
const TOGGLE_W: f32 = 64.0;

pub fn show(ui: &mut Ui, s: &Arc<Settings>) {
    staggered(ui, 0, |ui| {
        page_title(
            ui,
            tr(T::AdvancedEyebrow),
            tr(T::Advanced),
            tr(T::AdvancedSub),
        );
    });

    ui.add_space(52.0);

    let w = ((ui.available_width() - GAP) / 2.0).min(CARD_W);
    let total = w * 2.0 + GAP;
    let pad = ((ui.available_width() - total) / 2.0).max(0.0);
    let size = Vec2::new(w, CARD_H + MARGIN_BASE as f32 * 2.0);

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = GAP;
        ui.add_space(pad);

        drift(
            ui,
            "adv_jitter",
            size,
            1.0,
            Layout::top_down(Align::Min),
            |ui| jitter_card(ui, s, w),
        );
        drift(
            ui,
            "adv_limit",
            size,
            1.0,
            Layout::top_down(Align::Min),
            |ui| click_limit_card(ui, s, w),
        );
    });

    ui.add_space(16.0);
}

fn jitter_card(ui: &mut Ui, s: &Settings, w: f32) {
    let cps = s.cps();
    let unit = s.unit.load(Relaxed);

    let mut enabled = s.jitter_enabled.load(Relaxed);
    let mut pct = s.jitter.load(Relaxed);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 14.0);
        ui.set_width(w - MARGIN_BASE as f32 * 2.0);
        ui.set_height(CARD_H);

        ui.horizontal(|ui| {
            card_header_reserve(
                ui,
                ICON_FA_WAVE_SQUARE,
                tr(T::Jitter),
                tr(T::JitterSub),
                TOGGLE_W,
            );
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                let was = enabled;
                toggle(ui, &mut enabled);
                if enabled && !was && pct == 0 {
                    pct = 10;
                }
            });
        });

        ui.add_space(2.0);

        dim_scope(ui, "jitter", enabled, |ui| {
            let amount = cps * pct as f32 / 100.0;
            let mut lo = cps - amount;
            let mut hi = cps + amount;

            form_row(ui, tr(T::Range), |ui| {
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
                    RichText::new(units[unit.min(3)])
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

fn click_limit_card(ui: &mut Ui, s: &Arc<Settings>, w: f32) {
    let mut on = s.click_limit_enabled.load(Relaxed);
    let mut limit = s.click_limit.load(Relaxed);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 14.0);
        ui.set_width(w - MARGIN_BASE as f32 * 2.0);
        ui.set_height(CARD_H);

        ui.horizontal(|ui| {
            card_header_reserve(
                ui,
                ICON_FA_HASHTAG,
                tr(T::ClickLimit),
                tr(T::ClickLimitSub),
                TOGGLE_W,
            );
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                toggle(ui, &mut on);
            });
        });

        ui.add_space(2.0);

        dim_scope(ui, "click_limit", on, |ui| {
            form_row(ui, tr(T::StopAfter), |ui| {
                ui.add_sized(
                    [80.0, ROW_H],
                    egui::DragValue::new(&mut limit)
                        .range(1..=1_000_000)
                        .speed(1.0),
                );
                ui.label(
                    RichText::new(tr(T::ClicksLower))
                        .font(regular(12.0))
                        .color(MUTED),
                );
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
