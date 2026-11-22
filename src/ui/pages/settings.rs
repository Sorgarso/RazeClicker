use crate::i18n::{Lang, T, lang, set_lang, tr};
use crate::model::settings::Settings;
use crate::platform::windows::autostart;
use crate::ui::anim::page_reveal::staggered;
use crate::ui::components::logo_button::logo_button;
use crate::ui::components::{
    card::card, card_header::card_header, drift::drift, dropdown::dropdown,
    outlined_button::outlined_button, page_title::page_title, row_divider::row_divider,
    setting_row::setting_row, side_tabs::side_tabs, toggle::toggle,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::state::Page;
use crate::ui::theme::*;
use egui::{Align, Align2, CornerRadius, Layout, RichText, Stroke, Ui, Vec2};
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

const TABS_W: f32 = 176.0;
const PANE_W: f32 = 470.0;
const PANE_MAX_H: f32 = 380.0;
const HEADER_H: f32 = 56.0;
const GAP: f32 = 20.0;
const BTN_H: f32 = 34.0;

pub fn show(ui: &mut Ui, s: &Arc<Settings>, page: &mut Page) {
    let mut lang_idx = if lang() == Lang::Ru { 1 } else { 0 };
    let mut overlay = s.overlay.load(Relaxed);
    let mut on_top = s.always_on_top.load(Relaxed);
    let mut ambient_on = s.ambient.load(Relaxed);
    let mut parallax_on = s.parallax.load(Relaxed);

    let auto_id = egui::Id::new("autostart_state");
    let mut autostart_on: bool = ui
        .data_mut(|d| d.get_temp(auto_id))
        .unwrap_or_else(autostart::is_enabled);

    let cat_id = egui::Id::new("settings_cat");
    let mut cat: usize = ui.data(|d| d.get_temp(cat_id)).unwrap_or(0);

    let (old_lang, old_auto, old_overlay, old_on_top, old_ambient, old_parallax) = (
        lang_idx,
        autostart_on,
        overlay,
        on_top,
        ambient_on,
        parallax_on,
    );

    staggered(ui, 0, |ui| {
        page_title(
            ui,
            tr(T::SettingsEyebrow),
            tr(T::Settings),
            tr(T::SettingsSub),
        );
    });

    ui.add_space(26.0);

    let items = [
        (ICON_FA_GEAR, tr(T::General)),
        (ICON_FA_STAR, tr(T::Appearance)),
        (ICON_FA_SLIDERS, tr(T::Other)),
    ];
    let meta = [
        (ICON_FA_GEAR, tr(T::General), tr(T::GeneralSub)),
        (ICON_FA_STAR, tr(T::Appearance), tr(T::AppearanceSub)),
        (ICON_FA_SLIDERS, tr(T::Other), tr(T::OtherSub)),
    ];

    let m = MARGIN_BASE as f32;
    let pane_h = PANE_MAX_H + m * 2.0;
    let avail = ui.available_width();
    let pane_w = PANE_W.min(avail - TABS_W - GAP).max(300.0);
    let pad = ((avail - (TABS_W + GAP + pane_w)) / 2.0).max(0.0);

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = GAP;
        ui.add_space(pad);

        ui.allocate_ui_with_layout(
            Vec2::new(TABS_W, pane_h),
            Layout::top_down(Align::Min),
            |ui| {
                drift(
                    ui,
                    "set_tabs",
                    Vec2::new(TABS_W, pane_h),
                    1.0,
                    Layout::top_down(Align::Min),
                    |ui| side_tabs(ui, &mut cat, &items, TABS_W),
                );
            },
        );

        ui.allocate_ui_with_layout(
            Vec2::new(pane_w, pane_h),
            Layout::top_down(Align::Min),
            |ui| {
                drift(
                    ui,
                    "set_pane",
                    Vec2::new(pane_w, pane_h),
                    1.0,
                    Layout::top_down(Align::Min),
                    |ui| {
                        card(ui, MARGIN_BASE, MARGIN_BASE, 15, |ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(10.0, 6.0);
                            ui.set_width(pane_w - m * 2.0);

                            let (icon, title, sub) = meta[cat.min(meta.len() - 1)];
                            card_header(ui, icon, title, sub);
                            ui.add_space(4.0);

                            egui::ScrollArea::vertical()
                                .id_salt(("settings_scroll", cat))
                                .max_height(PANE_MAX_H - HEADER_H)
                                .auto_shrink([false, true])
                                .show(ui, |ui| match cat {
                                    0 => {
                                        setting_row(
                                            ui,
                                            ICON_FA_GLOBE,
                                            tr(T::Language),
                                            tr(T::LanguageSub),
                                            130.0,
                                            |ui| {
                                                dropdown(
                                                    ui,
                                                    "lang",
                                                    &mut lang_idx,
                                                    &["English", "Русский"],
                                                    120.0,
                                                );
                                            },
                                        );
                                        row_divider(ui);
                                        setting_row(
                                            ui,
                                            ICON_FA_POWER_OFF,
                                            tr(T::StartOnWindows),
                                            tr(T::StartOnWindowsSub),
                                            56.0,
                                            |ui| toggle(ui, &mut autostart_on),
                                        );
                                        row_divider(ui);
                                        setting_row(
                                            ui,
                                            ICON_FA_THUMBTACK,
                                            tr(T::AlwaysOnTop),
                                            tr(T::AlwaysOnTopSub),
                                            56.0,
                                            |ui| toggle(ui, &mut on_top),
                                        );
                                    }
                                    1 => {
                                        setting_row(
                                            ui,
                                            ICON_FA_EYE,
                                            tr(T::Overlay),
                                            tr(T::OverlaySub),
                                            56.0,
                                            |ui| toggle(ui, &mut overlay),
                                        );
                                        row_divider(ui);
                                        setting_row(
                                            ui,
                                            ICON_FA_WAND_MAGIC_SPARKLES,
                                            tr(T::Ambient),
                                            tr(T::AmbientSub),
                                            56.0,
                                            |ui| toggle(ui, &mut ambient_on),
                                        );
                                        row_divider(ui);
                                        setting_row(
                                            ui,
                                            ICON_FA_ARROWS_UP_DOWN_LEFT_RIGHT,
                                            tr(T::Parallax),
                                            tr(T::ParallaxSub),
                                            56.0,
                                            |ui| toggle(ui, &mut parallax_on),
                                        );
                                    }
                                    _ => {
                                        setting_row(
                                            ui,
                                            ICON_FA_KEYBOARD,
                                            tr(T::Hotkeys),
                                            tr(T::HotkeysSub),
                                            130.0,
                                            |ui| {
                                                if configure_button(ui, tr(T::Configure)) {
                                                    *page = Page::Clicker;
                                                }
                                            },
                                        );
                                        row_divider(ui);
                                        setting_row(
                                            ui,
                                            ICON_FA_ARROWS_ROTATE,
                                            tr(T::ResetAllSettings),
                                            tr(T::ResetAllSettingsSub),
                                            130.0,
                                            |ui| {
                                                if logo_button(
                                                    ui,
                                                    DANGER_IDLE,
                                                    DANGER_HOVER,
                                                    10,
                                                    tr(T::Reset),
                                                    Some(ICON_FA_TRASH),
                                                    Vec2::new(120.0, BTN_H),
                                                )
                                                .clicked()
                                                {
                                                    s.reset();
                                                    lang_idx = 0;
                                                    autostart_on = false;
                                                    overlay = false;
                                                    on_top = false;
                                                    ambient_on = true;
                                                    parallax_on = true;
                                                    ui.data_mut(|d| {
                                                        d.remove::<bool>(auto_id);
                                                    });
                                                }
                                            },
                                        );
                                    }
                                });
                        });
                    },
                );
            },
        );
    });

    ui.data_mut(|d| d.insert_temp(cat_id, cat));

    ui.add_space(12.0);
    ui.horizontal(|ui| {
        let text = tr(T::AutoSaved);
        let g = ui
            .painter()
            .layout_no_wrap(text.to_owned(), regular(12.0), MUTED);
        let total = 12.0 + 8.0 + g.size().x;
        ui.add_space(((ui.available_width() - total) / 2.0).max(0.0));
        ui.label(
            RichText::new(ICON_FA_CIRCLE_INFO)
                .font(icon_font(12.0))
                .color(MUTED),
        );
        ui.label(RichText::new(text).font(regular(12.0)).color(MUTED));
    });

    if lang_idx != old_lang {
        set_lang(if lang_idx == 1 { Lang::Ru } else { Lang::En });
    }
    if autostart_on != old_auto {
        autostart::set_enabled(autostart_on);
    }
    ui.data_mut(|d| d.insert_temp(auto_id, autostart_on));

    if overlay != old_overlay {
        s.overlay.store(overlay, Relaxed);
    }
    if on_top != old_on_top {
        s.always_on_top.store(on_top, Relaxed);
    }
    if ambient_on != old_ambient {
        s.ambient.store(ambient_on, Relaxed);
    }
    if parallax_on != old_parallax {
        s.parallax.store(parallax_on, Relaxed);
    }
}

fn configure_button(ui: &mut Ui, text: &str) -> bool {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(120.0, BTN_H), egui::Sense::click());
    let bg = if resp.hovered() { WIDGET_HOVER } else { WIDGET };
    ui.painter().rect(
        rect,
        CornerRadius::same(8),
        bg,
        Stroke::new(0.0, CARD_BORDER),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.left_center() + egui::vec2(14.0, 0.0),
        Align2::LEFT_CENTER,
        ICON_FA_KEYBOARD,
        icon_font(14.0),
        MUTED,
    );
    ui.painter().text(
        rect.left_center() + egui::vec2(38.0, 0.0),
        Align2::LEFT_CENTER,
        text,
        regular(14.0),
        TEXT,
    );
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp.clicked()
}
