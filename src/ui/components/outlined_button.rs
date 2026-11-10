use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::icon_font;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{
    Align2, Color32, CornerRadius, FontFamily, FontId, Pos2, Response, Sense, Stroke, StrokeKind,
    Ui, Vec2,
};

pub fn outlined_button(
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

    let hovered = resp.contains_pointer();
    let t = ui.ctx().animate_bool_with_time(resp.id, hovered, 0.2);

    let border = color_bg.lerp_to_gamma(color_hover, t);

    ui.painter().rect_filled(rect, corner_radius, DANGER_FILLED);
    ui.painter().rect_stroke(
        rect,
        corner_radius,
        Stroke::new(1.0, border),
        StrokeKind::Inside,
    );
    let text_galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), regular(14.0), Color32::WHITE);

    let icon_width = if icon.is_some() { 16.0 } else { 0.0 };
    let spacing = if icon.is_some() { 8.0 } else { 0.0 };

    let total_width = icon_width + spacing + text_galley.size().x;
    let start_x = rect.center().x - total_width / 2.0;

    if let Some(icon) = icon {
        ui.painter().text(
            Pos2::new(start_x, rect.center().y),
            Align2::LEFT_CENTER,
            icon,
            icon_font(16.0),
            TEXT,
        );
    }

    ui.painter().text(
        Pos2::new(start_x + icon_width + spacing, rect.center().y),
        Align2::LEFT_CENTER,
        text,
        regular(14.0),
        Color32::WHITE,
    );

    if hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
