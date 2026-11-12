use crate::ui::components::header_button::close_button::close_button;
use crate::ui::theme::*;
use egui::{
    Align2, Area, Color32, CornerRadius, Frame, Id, Key, Margin, Order, Sense, Shadow, Stroke, Ui,
    vec2,
};

pub fn modal(
    ctx: &egui::Context,
    id: &str,
    open: &mut bool,
    width: f32,
    height: f32,
    add_contents: impl FnOnce(&mut Ui),
) {
    if !*open {
        return;
    }
    let base = Id::new(id);
    let screen = ctx.content_rect();

    let t = ctx.animate_bool_with_time(base.with("fade"), true, 0.15);

    Area::new(base.with("dim"))
        .order(Order::Foreground)
        .fixed_pos(screen.min)
        .interactable(true)
        .show(ctx, |ui| {
            let (rect, resp) = ui.allocate_exact_size(screen.size(), Sense::click_and_drag());
            ui.painter().rect_filled(
                rect,
                CornerRadius::same(20),
                Color32::from_black_alpha((185.0 * t) as u8),
            );
        });

    Area::new(base.with("win"))
        .order(Order::Tooltip)
        .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
        .show(ctx, |ui| {
            Frame::new()
                .fill(CARD)
                .stroke(Stroke::new(1.0, CARD_BORDER))
                .corner_radius(CornerRadius::same(12))
                .inner_margin(Margin::same(18))
                .shadow(Shadow {
                    offset: [0, 16],
                    blur: 48,
                    spread: 0,
                    color: Color32::from_black_alpha(160),
                })
                .show(ui, |ui| {
                    ui.set_width(width);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if close_button(ui).clicked() {
                            *open = false;
                        }
                    });

                    ui.vertical_centered(|ui| {
                        add_contents(ui);
                    });
                });
        });
}
