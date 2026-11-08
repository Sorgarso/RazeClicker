use crate::ui::theme::*;
use egui::{
    vec2, Align2, Area, CornerRadius, FontFamily, FontId, Frame, Key, Margin, Order, Sense, Stroke,
    StrokeKind, Ui,
};

pub fn dropdown(ui: &mut Ui, id: &str, value: &mut usize, items: &[&str], width: f32) -> bool {
    let open_id = ui.make_persistent_id(id);
    let mut open = ui.data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);
    let mut changed = false;

    let (rect, resp) = ui.allocate_exact_size(vec2(width, ROW_H + 4.0), Sense::click());
    if resp.clicked() {
        open = !open;
    }

    let bg = if open || resp.hovered() {
        WIDGET_HOVER
    } else {
        WIDGET
    };
    let border = if open { ACCENT } else { CARD_BORDER };
    ui.painter().rect(
        rect,
        CornerRadius::same(10),
        bg,
        Stroke::new(1.0, border),
        StrokeKind::Inside,
    );
    ui.painter().text(
        rect.left_center() + vec2(14.0, 0.0),
        Align2::LEFT_CENTER,
        items[(*value).min(items.len() - 1)],
        FontId::new(15.0, FontFamily::Proportional),
        TEXT,
    );

    let c = rect.right_center() - vec2(18.0, 0.0);
    let dy = if open { -2.5 } else { 2.5 };
    let s = Stroke::new(1.6, MUTED);
    ui.painter()
        .line_segment([c + vec2(-5.0, -dy), c + vec2(0.0, dy)], s);
    ui.painter()
        .line_segment([c + vec2(0.0, dy), c + vec2(5.0, -dy)], s);

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    if open {
        let area = Area::new(open_id.with("popup"))
            .order(Order::Foreground)
            .fixed_pos(rect.left_bottom() + vec2(0.0, 4.0))
            .show(ui.ctx(), |ui| {
                Frame::new()
                    .fill(CARD)
                    .stroke(Stroke::new(1.0, CARD_BORDER))
                    .corner_radius(CornerRadius::same(10))
                    .inner_margin(Margin::same(4))
                    .show(ui, |ui| {
                        ui.set_width(width - 8.0);
                        ui.spacing_mut().item_spacing.y = 2.0;

                        for (i, name) in items.iter().enumerate() {
                            let (r, rr) =
                                ui.allocate_exact_size(vec2(width - 8.0, 34.0), Sense::click());
                            let selected = *value == i;

                            let bg = if rr.hovered() {
                                WIDGET_HOVER
                            } else if selected {
                                ACCENT.gamma_multiply(0.15)
                            } else {
                                egui::Color32::TRANSPARENT
                            };
                            ui.painter().rect_filled(r, CornerRadius::same(7), bg);
                            ui.painter().text(
                                r.left_center() + vec2(12.0, 0.0),
                                Align2::LEFT_CENTER,
                                name,
                                FontId::new(15.0, FontFamily::Proportional),
                                if selected { ACCENT } else { TEXT },
                            );
                            if rr.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }
                            if rr.clicked() {
                                *value = i;
                                changed = true;
                            }
                        }
                    });
            });

        let popup_rect = area.response.rect;
        let outside = ui.input(|i| {
            i.pointer.any_pressed()
                && i.pointer
                    .interact_pos()
                    .map_or(false, |p| !popup_rect.contains(p) && !rect.contains(p))
        });
        if changed || outside || ui.input(|i| i.key_pressed(Key::Escape)) {
            open = false;
        }
    }

    ui.data_mut(|d| d.insert_temp(open_id, open));
    changed
}
