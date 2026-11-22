use crate::ui::components::glow::radial_glow;
use crate::ui::components::spaced::spaced;
use crate::ui::fonts::fa_data::*;
use crate::ui::fonts::fonts::{icon_font, regular};
use crate::ui::theme::*;
use egui::{Align2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2};
use std::sync::Arc;
use std::time::Duration;

fn hint_text(ui: &Ui, hint: &str) -> Arc<str> {
    let id = ui.id().with("hint_spaced");
    if let Some((src, out)) = ui.data(|d| d.get_temp::<(Arc<str>, Arc<str>)>(id)) {
        if &*src == hint {
            return out;
        }
    }
    let out: Arc<str> = spaced(&hint.to_uppercase()).into();
    ui.data_mut(|d| d.insert_temp(id, (Arc::<str>::from(hint), out.clone())));
    out
}

pub fn start_button(
    ui: &mut Ui,
    running: bool,
    label: &str,
    hint: &str,
    diameter: f32,
) -> Response {
    let outer = diameter + 60.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(outer), Sense::hover());
    let c = rect.center();

    let resp = ui.interact(
        Rect::from_center_size(c, Vec2::splat(diameter)),
        ui.id().with("start_btn"),
        Sense::click(),
    );

    let hint = hint_text(ui, hint);
    let ctx = ui.ctx();
    let hover_t = ctx.animate_bool_with_time(resp.id.with("hover"), resp.hovered(), 0.2);
    let press_t =
        ctx.animate_bool_with_time(resp.id.with("press"), resp.is_pointer_button_down_on(), 0.1);
    let time = ui.input(|i| i.time);

    let r = diameter / 2.0 - press_t * 2.0;
    let scale = r / (diameter / 2.0);

    let rgba = |a: u8| Color32::from_rgba_unmultiplied(222, 163, 195, a);
    let p = ui.painter();

    let glow_a = (if running { 35.0 } else { 10.0 }) + hover_t * 25.0;
    radial_glow(p, c, r, r + 40.0, rgba(glow_a as u8));

    for (i, a) in [10u8, 16, 24].iter().enumerate() {
        p.circle_stroke(
            c,
            diameter / 2.0 + 28.0 - i as f32 * 9.0,
            Stroke::new(1.0, rgba(*a)),
        );
    }

    let fill = START_PINK.lerp_to_gamma(Color32::WHITE, 0.12 * hover_t);
    p.circle_filled(c, r, fill);
    p.circle_stroke(
        c,
        r,
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 40)),
    );

    let icon = if running { ICON_FA_STOP } else { ICON_FA_PLAY };
    p.text(
        c + Vec2::new(0.0, -26.0 * scale),
        Align2::CENTER_CENTER,
        icon,
        icon_font(15.0 * scale),
        START_PINK_DARK,
    );
    p.text(
        c,
        Align2::CENTER_CENTER,
        label,
        regular(21.0 * scale),
        START_PINK_DARK,
    );
    p.text(
        c + Vec2::new(0.0, 26.0 * scale),
        Align2::CENTER_CENTER,
        &*hint,
        regular(9.0 * scale),
        MUTED,
    );

    if resp.hovered() {
        ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
