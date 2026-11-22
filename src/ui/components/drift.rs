use egui::{Id, Layout, Sense, Ui, UiBuilder, Vec2, vec2};

const MAX_X: f32 = 7.0;
const MAX_Y: f32 = 5.0;
const SMOOTH: f32 = 9.0;

pub fn drift(
    ui: &mut Ui,
    id: &str,
    size: Vec2,
    depth: f32,
    layout: Layout,
    add: impl FnOnce(&mut Ui),
) {
    let ctx = ui.ctx().clone();
    let (base, _) = ui.allocate_exact_size(size, Sense::hover());
    let screen = ctx.content_rect();

    let on = ctx
        .data(|d| d.get_temp::<bool>(Id::new("parallax_on")))
        .unwrap_or(true);

    let target = match ctx.input(|i| i.pointer.hover_pos()) {
        Some(p) if on => {
            let c = screen.center();
            let nx = ((p.x - c.x) / (screen.width() * 0.5)).clamp(-1.0, 1.0);
            let ny = ((p.y - c.y) / (screen.height() * 0.5)).clamp(-1.0, 1.0);
            vec2(nx * MAX_X, ny * MAX_Y) * depth
        }
        _ => Vec2::ZERO,
    };

    let key = Id::new(("drift", id));
    let cur: Vec2 = ctx.data(|d| d.get_temp(key)).unwrap_or(Vec2::ZERO);
    let dt = ctx.input(|i| i.stable_dt).min(0.05);
    let next = cur + (target - cur) * (1.0 - (-SMOOTH * dt).exp());
    ctx.data_mut(|d| d.insert_temp(key, next));

    if (target - next).length() > 0.02 {
        ctx.request_repaint();
    }

    let mut child = ui.new_child(
        UiBuilder::new()
            .id_salt(("drift_child", id))
            .max_rect(base.translate(next))
            .layout(layout),
    );
    add(&mut child);
}
