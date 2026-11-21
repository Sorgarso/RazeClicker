use crate::ui::components::glow::radial_glow;
use crate::ui::fonts::fonts::icon_font;
use egui::{Align2, Color32, Sense, Stroke, StrokeKind, Ui, Vec2};

pub fn icon_badge(ui: &mut Ui, icon: &str, size: f32, color: Color32) {
    let outer = size + 40.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), outer), Sense::hover());
    let c = rect.center();
    let r = egui::Rect::from_center_size(c, Vec2::splat(size));

    let tint = |a: u8| Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), a);
    let p = ui.painter();

    radial_glow(p, c, size * 0.4, size * 0.4 + 30.0, tint(45));
    p.rect_filled(r, (size * 0.28) as u8, tint(34));
    p.rect_stroke(
        r,
        (size * 0.28) as u8,
        Stroke::new(1.0, tint(120)),
        StrokeKind::Inside,
    );
    p.text(
        c,
        Align2::CENTER_CENTER,
        icon,
        icon_font(size * 0.42),
        color,
    );
}
