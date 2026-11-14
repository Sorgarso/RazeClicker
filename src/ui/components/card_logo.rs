use egui::{Align2, Color32, CornerRadius, Rect, Sense, Stroke, StrokeKind, Ui, Vec2, pos2};

use crate::ui::fonts::fonts::icon_font;
use crate::ui::theme::*;

const LOGO_BOX: f32 = 56.0;
const LOGO_R: u8 = 14;

pub fn card_logo(ui: &mut Ui, icon: &str) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(LOGO_BOX), Sense::hover());
    let p = ui.painter();
    let r = CornerRadius::same(LOGO_R);

    p.rect_filled(rect, r, ACCENT.gamma_multiply(0.18));
    p.rect_stroke(
        rect,
        r,
        Stroke::new(1.0, ACCENT.gamma_multiply(0.35)),
        StrokeKind::Inside,
    );
    p.text(
        rect.center(),
        Align2::CENTER_CENTER,
        icon,
        icon_font(20.0),
        ACCENT_HOVER,
    );
}
