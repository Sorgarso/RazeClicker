use egui::{Context, Id, Ui};

use crate::ui::{anim::animate_ease::ease_out_cubic, state::Page};

pub fn page_started(ctx: &Context, page: Page) {
    let id = Id::new("page_start");
    let now = ctx.input(|i| i.time);
    let cur: Option<(Page, f64)> = ctx.data(|d| d.get_temp(id));

    match cur {
        Some((p, _)) if p == page => {}
        Some(_) => {
            ctx.data_mut(|d| {
                d.insert_temp(id, (page, now));
            });
        }
        None => {
            ctx.data_mut(|d| {
                d.insert_temp(id, (page, now - 10.0));
            });
        }
    }
}

pub fn reveal(ctx: &Context, delay: f32, dur: f32) -> f32 {
    let start = ctx
        .data(|d| d.get_temp::<(Page, f64)>(Id::new("page_start")))
        .map(|(_, s)| s)
        .unwrap_or(0.0);
    let now = ctx.input(|i| i.time);
    let t = (((now - start) as f32 - delay) / dur).clamp(0.0, 1.0);
    if t < 1.0 {
        ctx.request_repaint();
    }
    ease_out_cubic(t)
}

pub fn staggered(ui: &mut Ui, idx: usize, add: impl FnOnce(&mut Ui)) {
    let t = reveal(ui.ctx(), idx as f32 * 0.07, 0.35);
    ui.scope(|ui| {
        ui.set_opacity(t);
        ui.add_space((1.0 - t) * 12.0);
        add(ui);
    });
}
