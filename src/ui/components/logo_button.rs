use crate::ui::fonts::fonts::icon_font;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align2, Color32, Pos2, Vec2};

pub fn logo_button(
    ui: &mut egui::Ui,
    color_bg: Color32,
    color_hover: Color32,
    corner_radius: u8,
    text: &str,
    icon: Option<&str>,
    size: Vec2,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    let ctx = ui.ctx();
    let press_t = ctx.animate_bool_with_time(
        resp.id.with("press"),
        resp.is_pointer_button_down_on(),
        0.10,
    );

    let hovered = resp.contains_pointer();
    let t = ctx.animate_bool_with_time(resp.id, hovered, 0.2);

    let bg = color_bg.lerp_to_gamma(color_hover, t);

    let r = rect.shrink(press_t * 1.5);
    ui.painter().rect_filled(r, corner_radius, bg);

    let scale = r.height() / rect.height();
    let text_galley =
        ui.painter()
            .layout_no_wrap(text.to_owned(), regular(14.0 * scale), ON_ACCENT);

    let icon_width = if icon.is_some() { 16.0 } else { 0.0 };
    let spacing = if icon.is_some() { 8.0 } else { 0.0 };

    let total_width = icon_width + spacing + text_galley.size().x;
    let start_x = r.center().x - total_width / 2.0;

    if let Some(icon) = icon {
        ui.painter().text(
            Pos2::new(start_x, r.center().y),
            Align2::LEFT_CENTER,
            icon,
            icon_font(16.0 * scale),
            ON_ACCENT,
        );
    }

    ui.painter().text(
        Pos2::new(start_x + icon_width + spacing, r.center().y),
        Align2::LEFT_CENTER,
        text,
        regular(14.0 * scale),
        ON_ACCENT,
    );

    if hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
