use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::icon_font;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align2, Color32, CornerRadius, FontFamily, FontId, Pos2, Response, Sense, Ui, Vec2};

pub fn logo_button(
    ui: &mut egui::Ui,
    color_bg: Color32,
    color_hover: Color32,
    corner_radius: u8,
    text: &str,
    icon: Option<&str>,
    size: Vec2,
) -> egui::Response {
    let desired = size;
    let (rect, resp) = ui.allocate_exact_size(desired, egui::Sense::click());

    let ctx = ui.ctx();

    let hover_t = ctx.animate_bool_with_time(resp.id.with("hover"), resp.hovered(), 0.20);
    let press_t = ctx.animate_bool_with_time(
        resp.id.with("press"),
        resp.is_pointer_button_down_on(),
        0.10,
    );

    let hovered = resp.contains_pointer();
    let t = ui.ctx().animate_bool_with_time(resp.id, hovered, 0.2);

    let bg = color_bg.lerp_to_gamma(color_hover, t);

    let r = rect.shrink(press_t * 1.5);
    ui.painter().rect_filled(r, corner_radius, bg);

    let text_galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), regular(14.0), Color32::WHITE);

    let icon_width = if icon.is_some() { 16.0 } else { 0.0 };
    let spacing = if icon.is_some() { 8.0 } else { 0.0 };

    let total_width = icon_width + spacing + text_galley.size().x;
    let start_x = r.center().x - total_width / 2.0;

    if let Some(icon) = icon {
        ui.painter().text(
            Pos2::new(start_x, r.center().y),
            Align2::LEFT_CENTER,
            icon,
            icon_font(16.0),
            TEXT,
        );
    }

    let scale = r.height() / rect.height();
    ui.painter().text(
        Pos2::new(start_x + icon_width + spacing, r.center().y),
        Align2::LEFT_CENTER,
        text,
        regular(14.0 * scale),
        Color32::WHITE,
    );

    // ui.painter().text(
    //     Pos2::new(start_x + icon_width + spacing, r.center().y),
    //     Align2::LEFT_CENTER,
    //     text,
    //     regular(14.0),
    //     Color32::WHITE,
    // );

    if hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
