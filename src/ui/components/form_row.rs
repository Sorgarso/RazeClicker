use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align2, Sense, Ui, vec2};

const LABEL_W: f32 = 92.0;

pub fn form_row(ui: &mut Ui, label: &str, content: impl FnOnce(&mut Ui)) {
    form_row_w(ui, label, LABEL_W, content);
}

pub fn form_row_w(ui: &mut Ui, label: &str, label_w: f32, content: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(vec2(label_w, ROW_H), Sense::hover());
        ui.painter().with_clip_rect(rect).text(
            rect.left_center(),
            Align2::LEFT_CENTER,
            label,
            regular(13.0),
            MUTED,
        );
        content(ui);
    });
}
