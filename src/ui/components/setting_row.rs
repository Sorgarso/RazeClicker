use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::theme::*;
use egui::{
    Align, Align2, Color32, Layout, Rect, Sense, Stroke, StrokeKind, Ui, UiBuilder, pos2, vec2,
};

const SR_H: f32 = 48.0;

pub fn setting_row(
    ui: &mut Ui,
    icon: &str,
    title: &str,
    sub: &str,
    ctrl_w: f32,
    control: impl FnOnce(&mut Ui),
) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), SR_H), Sense::hover());

    let badge = Rect::from_min_size(pos2(rect.left(), rect.center().y - 15.0), vec2(30.0, 30.0));
    let p = ui.painter();
    p.rect_filled(badge, 9, Color32::from_rgba_unmultiplied(222, 163, 195, 30));
    p.rect_stroke(
        badge,
        9,
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(222, 163, 195, 90)),
        StrokeKind::Inside,
    );
    p.text(
        badge.center(),
        Align2::CENTER_CENTER,
        icon,
        icon_font(13.0),
        START_PINK,
    );

    let tx = badge.right() + 12.0;
    let tw = (rect.right() - ctrl_w - 12.0 - tx).max(60.0);
    let g1 = p.layout(title.to_owned(), regular(14.0), TEXT, tw);
    let g2 = p.layout(sub.to_owned(), regular(11.0), MUTED, tw);
    let total = g1.size().y + 2.0 + g2.size().y;
    let y = rect.center().y - total / 2.0;
    let h1 = g1.size().y;
    p.galley(pos2(tx, y), g1, TEXT);
    p.galley(pos2(tx, y + h1 + 2.0), g2, MUTED);

    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::right_to_left(Align::Center)),
    );
    control(&mut child);
}
