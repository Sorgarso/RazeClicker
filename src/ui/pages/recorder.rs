use crate::engine::recorder;
use crate::i18n::{T, tr};
use crate::model::recording::Action;
use crate::model::recording::RecEvent;
use crate::model::{key::key_name, settings::Settings};
use crate::platform::windows::hooks;
use crate::platform::windows::hooks::start_pick_location;
use crate::ui::components::logo_button::logo_button;
use crate::ui::components::toggle::toggle;
use crate::ui::components::{
    button, card::card, card_header::card_header, card_logo::card_logo, coord_row::coord_row,
    dropdown::dropdown, hotkey_input::hotkey_input, input_text::input_text, logo_button,
    outlined_button::outlined_button, radio_card::radio_card, rec_event_row::rec_event_row,
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
    let start_hotkey = s.start_hotkey.load(Relaxed);
    let play_hotkey = s.play_hotkey.load(Relaxed);
    let capturing_play_hotkey = s.capturing_play_hotkey.load(Relaxed);
    let capturing_start_hotkey = s.capturing_start_hotkey.load(Relaxed);
    let mut play_loops = s.play_loops.load(Relaxed) as usize;

    ui.horizontal(|ui| {
        card_logo(ui, ICON_FA_RECORD_VINYL);
        ui.add_space(7.0);
        ui.vertical(|ui| {
            ui.add_space(3.0);
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.label(
                RichText::new(tr(T::Recorder))
                    .font(regular(26.0))
                    .color(TEXT),
            );
            ui.label(RichText::new(tr(T::RecorderSub)).size(14.0).color(MUTED));
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
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.allocate_ui(card_size, |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
                        ui.horizontal(|ui| {
                            card_header(
                                ui,
                                ICON_FA_RECORD_VINYL,
                                tr(T::Recording),
                                tr(T::RecordingSub),
                            );

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let recording = recorder::is_recording(s);

                                    let (rect, _) = ui.allocate_exact_size(
                                        egui::vec2(78.0, 30.0),
                                        egui::Sense::hover(),
                                    );

                                    let painter = ui.painter();

                                    let bg = if recording {
                                        egui::Color32::from_rgb(48, 22, 29)
                                    } else {
                                        egui::Color32::from_rgb(30, 30, 38)
                                    };

                                    let red = if recording {
                                        egui::Color32::from_rgb(255, 75, 75)
                                    } else {
                                        egui::Color32::from_rgb(85, 85, 95)
                                    };

                                    painter.rect_filled(rect, egui::CornerRadius::same(17), bg);

                                    painter.circle_filled(
                                        egui::pos2(rect.left() + 22.0, rect.center().y),
                                        6.0,
                                        red,
                                    );

                                    painter.text(
                                        egui::pos2(rect.left() + 34.0, rect.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        if recording { "REC" } else { "IDLE" },
                                        regular(14.0),
                                        red,
                                    );
                                },
                            );
                        });
                    });
                    ui.add_space(10.0);
                    card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.spacing_mut().interact_size.y = 18.0;
                        let events: Vec<RecEvent> = {
                            let track = s.rec_track.lock().unwrap();
                            filter_display_events(&track)
                        };

                        if events.is_empty() {
                            ui.allocate_ui_with_layout(
                                ui.available_size(),
                                Layout::top_down(Align::Center),
                                |ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(tr(T::NoSavedEvents));
                                    });
                                },
                            );
                        } else {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.label(
                                    egui::RichText::new(tr(T::recEvent))
                                        .font(regular(11.0))
                                        .color(MUTED)
                                        .strong(),
                                );

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{} {}",
                                            events.len(),
                                            tr(T::events)
                                        ))
                                        .font(regular(11.0))
                                        .color(ACCENT),
                                    );
                                });
                            });

                            ui.add_space(6.0);

                            egui::ScrollArea::vertical()
                                .id_salt("saved_rec_event")
                                .max_height(CARD_H)
                                .auto_shrink([false, true])
                                .show(ui, |ui| {
                                    ui.spacing_mut().item_spacing.y = 4.0;

                                    for (i, event) in events.iter().enumerate() {
                                        rec_event_row(ui, i + 1, *event);
                                    }
                                });
                        }
                    });
                });
            });
        });

        ui.allocate_ui_with_layout(Vec2::new(w2, 250.0), Layout::top_down(Align::Min), |ui| {
            card(ui, 10, 10, 10, |ui| {
                card_header(ui, ICON_FA_CIRCLE_PLAY, tr(T::Playback), tr(T::PlaybackSub));
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        row_label(ui, tr(T::RecordingHotkey));

                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            if hotkey_input(ui, &key_name(start_hotkey), capturing_start_hotkey) {
                                s.capturing_start_hotkey
                                    .store(!capturing_start_hotkey, Relaxed);
                            }
                        });
                    });
                    ui.horizontal(|ui| {
                        row_label(ui, tr(T::PlaybackHotkey));
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            if hotkey_input(ui, &key_name(play_hotkey), capturing_play_hotkey) {
                                s.capturing_play_hotkey
                                    .store(!capturing_play_hotkey, Relaxed);
                            }
                        });
                    });

                    ui.horizontal(|ui| {
                        row_label(ui, tr(T::Repeat));
                        ui.add_space(65.0);

                        let mut repeat_index = if play_loops == 1 { 0 } else { 1 };

                        if dropdown(
                            ui,
                            "repeat_dropdown",
                            &mut repeat_index,
                            &[tr(T::Once), tr(T::Inf)],
                            140.0,
                        ) {
                            play_loops = if repeat_index == 0 { 1 } else { 0 };
                            s.play_loops.store(play_loops as u32, Relaxed);
                        }
                    });

                    if outlined_button(
                        ui,
                        DANGER_IDLE,
                        DANGER_HOVER,
                        10,
                        tr(T::ClearAll),
                        Some(ICON_FA_TRASH),
                        Vec2::new(ui.available_width(), 40.0),
                    )
                    .clicked()
                    {
                        s.rec_track.lock().unwrap().clear();
                    }
                });
            });
        });
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

            _ => {
                result.push(event);
            }
        }
    }

    result
}
