use crate::engine::recorder;
use crate::i18n::{T, tr};
use crate::model::recording::{Action, REC_IDLE, RecEvent};
use crate::model::{key::key_name, settings::Settings};
use crate::ui::anim::page_reveal::staggered;
use crate::ui::components::{
    bind_field::bind_field,
    card::card,
    card_header::{card_header, card_header_reserve},
    dim_scope::dim_scope,
    drift::drift,
    dropdown::dropdown,
    form_row::form_row_w,
    logo_button::logo_button,
    page_title::page_title,
    rec_badge::rec_badge,
    rec_event_row::rec_event_row,
    toggle::toggle,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align, Layout, RichText, Ui, Vec2};
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

const LEFT_W: f32 = 400.0;
const RIGHT_W: f32 = 300.0;
const GAP: f32 = 28.0;
const V_GAP: f32 = 12.0;
const HEADER_H: f32 = 10.0;
const EVENTS_H: f32 = 150.0;
const LABEL_W: f32 = 118.0;
const HEADER_RESERVE: f32 = 150.0;

pub fn show(ui: &mut Ui, s: &Arc<Settings>) {
    let mut rec_on = s.recorder_enabled.load(Relaxed);

    staggered(ui, 0, |ui| {
        page_title(
            ui,
            tr(T::RecorderEyebrow),
            tr(T::Recorder),
            tr(T::RecorderSub),
        );
    });

    ui.add_space(36.0);

    let avail = ui.available_width();
    let k = ((avail - GAP) / (LEFT_W + RIGHT_W)).min(1.0);
    let lw = LEFT_W * k;
    let rw = RIGHT_W * k;
    let pad = ((avail - (lw + rw + GAP)) / 2.0).max(0.0);

    let m = MARGIN_BASE as f32 * 2.0;
    let left_total = (HEADER_H + m) + V_GAP + (EVENTS_H + m);
    let right_inner = left_total - m;

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = GAP;
        ui.add_space(pad);

        drift(
            ui,
            "rec_left",
            Vec2::new(lw, left_total),
            1.0,
            Layout::top_down(Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing.y = V_GAP;
                recording_header(ui, s, lw, rec_on);
                events_card(ui, s, lw, rec_on);
            },
        );

        drift(
            ui,
            "rec_right",
            Vec2::new(rw, left_total),
            1.0,
            Layout::top_down(Align::Min),
            |ui| playback_card(ui, s, rw, right_inner, &mut rec_on),
        );
    });

    if rec_on != s.recorder_enabled.load(Relaxed) {
        s.recorder_enabled.store(rec_on, Relaxed);
        if !rec_on {
            s.rec_state.store(REC_IDLE, Relaxed);
            s.capturing_start_hotkey.store(false, Relaxed);
            s.capturing_play_hotkey.store(false, Relaxed);
        }
    }

    ui.add_space(16.0);
}

fn recording_header(ui: &mut Ui, s: &Arc<Settings>, w: f32, rec_on: bool) {
    let recording = rec_on && recorder::is_recording(s);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.set_width(w - MARGIN_BASE as f32 * 2.0);
        ui.set_height(HEADER_H);

        ui.horizontal(|ui| {
            card_header_reserve(
                ui,
                ICON_FA_RECORD_VINYL,
                tr(T::Recording),
                tr(T::RecordingSub),
                HEADER_RESERVE,
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                rec_badge(ui, recording);
            });
        });
    });
}

fn events_card(ui: &mut Ui, s: &Arc<Settings>, w: f32, enabled: bool) {
    let events: Vec<RecEvent> = {
        let track = s.rec_track.lock().unwrap();
        filter_display_events(&track)
    };

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.set_width(w - MARGIN_BASE as f32 * 2.0);
        ui.set_height(EVENTS_H);

        dim_scope(ui, "rec_events", enabled, |ui| {
            if events.is_empty() {
                let h = ui.available_height();
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), h),
                    Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        ui.label(
                            RichText::new(tr(T::NoSavedEvents))
                                .font(regular(13.0))
                                .color(MUTED),
                        );
                    },
                );
                return;
            }

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(tr(T::recEvent))
                        .font(regular(11.0))
                        .color(MUTED)
                        .strong(),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{} {}", events.len(), tr(T::events)))
                            .font(regular(11.0))
                            .color(ACCENT),
                    );
                });
            });
            ui.add_space(6.0);

            egui::ScrollArea::vertical()
                .id_salt("saved_rec_event")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 4.0;
                    for (i, event) in events.iter().enumerate() {
                        rec_event_row(ui, i + 1, *event);
                    }
                });
        });
    });
}

fn playback_card(ui: &mut Ui, s: &Arc<Settings>, w: f32, inner_h: f32, rec_on: &mut bool) {
    let start_hotkey = s.start_hotkey.load(Relaxed);
    let play_hotkey = s.play_hotkey.load(Relaxed);
    let capturing_start = s.capturing_start_hotkey.load(Relaxed);
    let capturing_play = s.capturing_play_hotkey.load(Relaxed);
    let play_loops = s.play_loops.load(Relaxed);

    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 14.0);
        ui.set_width(w - MARGIN_BASE as f32 * 2.0);
        ui.set_height(inner_h);

        ui.horizontal(|ui| {
            card_header(ui, ICON_FA_CIRCLE_PLAY, tr(T::Playback), tr(T::PlaybackSub));
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                toggle(ui, rec_on);
            });
        });

        ui.add_space(4.0);

        form_row_w(ui, tr(T::RecordingHotkey), LABEL_W, |ui| {
            if bind_field(ui, &key_name(start_hotkey), capturing_start) {
                s.capturing_start_hotkey.store(!capturing_start, Relaxed);
            }
        });

        form_row_w(ui, tr(T::PlaybackHotkey), LABEL_W, |ui| {
            if bind_field(ui, &key_name(play_hotkey), capturing_play) {
                s.capturing_play_hotkey.store(!capturing_play, Relaxed);
            }
        });

        form_row_w(ui, tr(T::Repeat), LABEL_W, |ui| {
            let mut idx = if play_loops == 1 { 0 } else { 1 };
            let w = ui.available_width();
            if dropdown(
                ui,
                "repeat_dropdown",
                &mut idx,
                &[tr(T::Once), tr(T::Inf)],
                w,
            ) {
                s.play_loops.store(if idx == 0 { 1 } else { 0 }, Relaxed);
            }
        });

        let free = (ui.available_height() - 40.0).max(0.0);
        ui.add_space(free);

        if logo_button(
            ui,
            DANGER_IDLE,
            DANGER_HOVER,
            12,
            tr(T::ClearAll),
            Some(ICON_FA_TRASH),
            Vec2::new(ui.available_width(), 40.0),
        )
        .clicked()
        {
            s.rec_track.lock().unwrap().clear();
        }
    });
}

fn filter_display_events(track: &[RecEvent]) -> Vec<RecEvent> {
    let mut result = Vec::new();
    let mut last_pos: Option<(i32, i32)> = None;

    for event in track.iter().copied() {
        match event.action {
            Action::Move { x, y } => {
                if let Some((lx, ly)) = last_pos {
                    let dx = x - lx;
                    let dy = y - ly;
                    if dx * dx + dy * dy < 100 * 100 {
                        continue;
                    }
                }
                last_pos = Some((x, y));
                result.push(event);
            }
            _ => result.push(event),
        }
    }

    result
}
