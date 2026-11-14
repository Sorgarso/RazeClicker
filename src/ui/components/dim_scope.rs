use egui::Ui;

use crate::ui::anim::animate_ease::*;

pub fn dim_scope(ui: &mut Ui, salt: &str, enabled: bool, add: impl FnOnce(&mut Ui)) {
    let ctx = ui.ctx().clone();
    let id = ui.id().with(("dim", salt));

    let target = if enabled { 1.0 } else { 0.0 };
    let t = animate_ease(&ctx, id, target, 0.3, ease_out_cubic);

    ui.add_enabled_ui(enabled, |ui| {
        ui.set_opacity(0.5 + 0.5 * t);
        add(ui);
    });
}
