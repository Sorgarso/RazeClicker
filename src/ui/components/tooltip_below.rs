use crate::ui::fonts::fonts::regular;
use crate::ui::theme::TEXT;
use egui::{Area, Frame, Id, Order, RichText};

pub fn tooltip_below(ui: &egui::Ui, resp: &egui::Response, text: &str) {
    if !resp.contains_pointer() {
        return;
    }

    let pos = resp.rect.center_bottom() + egui::vec2(0.0, 6.0);

    Area::new(Id::new(("tip", resp.id)))
        .order(Order::Tooltip)
        .pivot(egui::Align2::CENTER_TOP)
        .fixed_pos(pos)
        .constrain(true)
        .interactable(false)
        .show(ui.ctx(), |ui| {
            Frame::popup(ui.style()).show(ui, |ui| {
                ui.label(RichText::new(text).font(regular(12.0)).color(TEXT));
            });
        });
}
