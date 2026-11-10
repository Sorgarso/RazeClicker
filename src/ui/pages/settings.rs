use crate::i18n::{Lang, T, lang, set_lang, tr};
use crate::model::{config, settings::Settings};
use crate::platform::windows::autostart;
use crate::ui::components::{
    card::card, dropdown::dropdown, outlined_button::outlined_button, row_divider::row_divider,
    segmented::segmented, setting_item::setting_item, toggle::toggle,
};
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::state::Page;
use crate::ui::theme::*;
use egui::{ComboBox, CornerRadius, Frame, Margin, RichText, Stroke, Ui, Vec2};
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

pub fn show(ui: &mut Ui, s: &Arc<Settings>, page: &mut Page) {
    let mut lang_idx = if lang() == Lang::Ru { 1 } else { 0 };

    let auto_id = egui::Id::new("autostart_state");
    let mut autostart_on: bool = ui
        .data_mut(|d| d.get_temp(auto_id))
        .unwrap_or_else(autostart::is_enabled);

    let (old_lang, old_auto) = (lang_idx, autostart_on);

    ui.horizontal(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(tr(T::Settings))
                    .font(regular(26.0))
                    .color(TEXT),
            );
            ui.label(RichText::new(tr(T::SettingsSub)).size(14.0).color(MUTED));
        });
    });
    ui.add_space(14.0);

    card(ui, 16, 12, 16, |ui| {
        ui.spacing_mut().item_spacing.y = 0.0;

        setting_item(
            ui,
            ICON_FA_GLOBE,
            tr(T::Language),
            tr(T::LanguageSub),
            |ui| {
                dropdown(ui, "lang", &mut lang_idx, &["English", "Русский"], 160.0);
            },
        );
        row_divider(ui);

        setting_item(
            ui,
            ICON_FA_POWER_OFF,
            tr(T::StartOnWindows),
            tr(T::StartOnWindowsSub),
            |ui| toggle(ui, &mut autostart_on),
        );

        row_divider(ui);

        setting_item(
            ui,
            ICON_FA_ARROWS_ROTATE,
            tr(T::ResetAllSettings),
            tr(T::ResetAllSettingsSub),
            |ui| {
                if outlined_button(
                    ui,
                    DANGER_IDLE,
                    DANGER_HOVER,
                    10,
                    tr(T::Reset),
                    Some(ICON_FA_TRASH),
                    Vec2::new(140.0, 40.0),
                )
                .clicked()
                {
                    s.reset();
                    lang_idx = 0;
                    autostart_on = false;

                    ui.data_mut(|d| {
                        d.remove::<bool>(auto_id);
                    });
                }
            },
        );
    });

    ui.add_space(14.0);

    card(ui, 16, 12, 16, |ui| {
        setting_item(
            ui,
            ICON_FA_KEYBOARD,
            tr(T::Hotkeys),
            tr(T::HotkeysSub),
            |ui| {
                if configure_button(ui, tr(T::Configure)) {
                    *page = Page::Clicker;
                }
            },
        );
    });

    ui.add_space(14.0);

    card(ui, 18, 12, 18, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 14.0;
            ui.label(
                RichText::new(ICON_FA_CIRCLE_INFO)
                    .font(icon_font(18.0))
                    .color(ICON),
            );
            ui.label(RichText::new(tr(T::AutoSaved)).size(13.0).color(MUTED));
        });
    });

    let mut changed = false;
    if lang_idx != old_lang {
        set_lang(if lang_idx == 1 { Lang::Ru } else { Lang::En });
        changed = true;
    }
    if autostart_on != old_auto {
        autostart::set_enabled(autostart_on);
    }
    ui.data_mut(|d| d.insert_temp(auto_id, autostart_on));

    if lang_idx != old_lang {
        set_lang(if lang_idx == 1 { Lang::Ru } else { Lang::En });
    }
    if autostart_on != old_auto {
        autostart::set_enabled(autostart_on);
    }
    ui.data_mut(|d| d.insert_temp(auto_id, autostart_on));
}

fn configure_button(ui: &mut Ui, text: &str) -> bool {
    let font = regular(14.0);
    let galley_w = 130.0;
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(galley_w, 38.0), egui::Sense::click());
    let bg = if resp.hovered() { WIDGET_HOVER } else { WIDGET };
    ui.painter().rect(
        rect,
        CornerRadius::same(8),
        bg,
        Stroke::new(1.0, CARD_BORDER),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.left_center() + egui::vec2(16.0, 0.0),
        egui::Align2::LEFT_CENTER,
        ICON_FA_KEYBOARD,
        icon_font(14.0),
        MUTED,
    );
    ui.painter().text(
        rect.left_center() + egui::vec2(42.0, 0.0),
        egui::Align2::LEFT_CENTER,
        text,
        font,
        TEXT,
    );
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp.clicked()
}
