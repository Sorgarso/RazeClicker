use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::icon_font;
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align2, Color32, CornerRadius, FontFamily, FontId, Response, Sense, Ui, Vec2};

pub fn side_panel_button(ui: &mut Ui, icon: Option<&str>, text: &str, selected: bool) -> Response {
    let size = Vec2::new(ui.available_width(), 38.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());

    let t = ui
        .ctx()
        .animate_bool_with_time(resp.id, resp.hovered(), 0.15);
    let s = ui
        .ctx()
        .animate_bool_with_time(resp.id.with("sel"), selected, 0.15);

    let bg = Color32::TRANSPARENT
        .lerp_to_gamma(Color32::from_rgb(30, 30, 38), t)
        .lerp_to_gamma(ACCENT.gamma_multiply(0.25), s);
    ui.painter().rect_filled(rect, CornerRadius::same(8), bg);

    let text_color = MUTED.lerp_to_gamma(TEXT, s.max(t));

    let mut text_x = 16.0;

    if let Some(icon) = icon {
        ui.painter().text(
            rect.left_center() + Vec2::new(18.0, 0.0),
            Align2::LEFT_CENTER,
            icon,
            icon_font(16.0),
            text_color,
        );
        text_x = 48.0;
    }

    ui.painter().text(
        rect.left_center() + Vec2::new(text_x, 0.0),
        Align2::LEFT_CENTER,
        text,
        regular(15.0),
        text_color,
    );

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
