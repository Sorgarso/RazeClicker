use crate::i18n::{lang, set_lang, tr, Lang, T};
use crate::ui::fonts::fa_data::ICON_FA_KEYBOARD;
use crate::ui::fonts::fonts::icon_font;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align2, CornerRadius, FontFamily, FontId, Sense, Stroke, StrokeKind, Ui, Vec2};

pub fn hotkey_input(ui: &mut Ui, key_text: &str, capturing: bool) -> bool {
    let btn_w = 46.0;
    let field_w = (ui.available_width() - btn_w - ui.spacing().item_spacing.x).max(60.0);

    let (rect, _) = ui.allocate_exact_size(Vec2::new(field_w, ROW_H), Sense::hover());
    let stroke = if capturing {
        Stroke::new(1.5, ACCENT)
    } else {
        Stroke::new(1.0, CARD_BORDER)
    };
    ui.painter().rect(
        rect,
        CornerRadius::same(8),
        WIDGET,
        stroke,
        StrokeKind::Inside,
    );
    let (text, color) = if capturing {
        (tr(T::PressKey), MUTED)
    } else {
        (key_text, TEXT)
    };
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        regular(15.0),
        color,
    );

    let (brect, resp) = ui.allocate_exact_size(Vec2::new(btn_w, ROW_H), Sense::click());
    let bg = if capturing || resp.hovered() {
        WIDGET_HOVER
    } else {
        WIDGET
    };
    ui.painter().rect(
        brect,
        CornerRadius::same(8),
        bg,
        Stroke::new(1.0, CARD_BORDER),
        StrokeKind::Inside,
    );
    ui.painter().text(
        brect.center(),
        Align2::CENTER_CENTER,
        ICON_FA_KEYBOARD,
        icon_font(16.0),
        if capturing { ACCENT } else { MUTED },
    );
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp.clicked()
}
