use crate::ui::fonts::fonts::icon_font;
use egui::{Color32, FontFamily, FontId, Vec2};

pub fn font_button(
    ui: &mut egui::Ui,
    color_bg: Color32,
    color_hover: Color32,
    corner_radius: u8,
    text: &str,
    size: Vec2,
) -> egui::Response {
    let desired = size;
    let (rect, resp) = ui.allocate_exact_size(desired, egui::Sense::click());

    let hovered = resp.contains_pointer();
    let t = ui.ctx().animate_bool_with_time(resp.id, hovered, 0.2);

    let bg = color_bg.lerp_to_gamma(color_hover, t);

    ui.painter().rect_filled(rect, corner_radius, bg);

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        icon_font(14.0),
        Color32::WHITE,
    );
    if hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
