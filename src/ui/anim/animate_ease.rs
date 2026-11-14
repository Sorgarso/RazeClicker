use egui::{Context, Id};

pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

pub fn ease_out_quint(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(5)
}

pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

pub fn animate_ease(ctx: &Context, id: Id, target: f32, dur: f32, ease: fn(f32) -> f32) -> f32 {
    let now = ctx.input(|i| i.time);
    let (mut from, mut to, mut start) = ctx
        .data_mut(|d| d.get_temp::<(f32, f32, f64)>(id))
        .unwrap_or((target, target, now));

    let cur = |from: f32, to: f32, start: f64| {
        let t = (((now - start) as f32) / dur).clamp(0.0, 1.0);
        from + (to - from) * ease(t)
    };

    if (to - target).abs() > 0.01 {
        from = cur(from, to, start);
        to = target;
        start = now;
    }

    let t = (((now - start) as f32) / dur).clamp(0.0, 1.0);
    let v = from + (to - from) * ease(t);

    ctx.data_mut(|d| d.insert_temp(id, (from, to, start)));
    if t < 1.0 {
        ctx.request_repaint();
    }
    v
}
