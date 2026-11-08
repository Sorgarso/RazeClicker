use crate::ui::{fonts::fonts::regular, theme::*};
use egui::{Align2, Color32, CornerRadius, FontFamily, FontId, Sense, Ui, Vec2};

pub fn segmented(ui: &mut Ui, value: &mut usize, items: &[&str], item_w: f32) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        ui.horizontal(|ui| {
            for (i, name) in items.iter().enumerate() {
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(item_w, ROW_H), Sense::click());
                let selected = *value == i;
                let bg = if selected {
                    ACCENT
                } else if resp.hovered() {
                    WIDGET_HOVER
                } else {
                    WIDGET
                };
                ui.painter().rect_filled(rect, CornerRadius::same(7), bg);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    name,
                    regular(14.0),
                    if selected { Color32::WHITE } else { MUTED },
                );
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.clicked() {
                    *value = i;
                }
            }
        });
    });
}
