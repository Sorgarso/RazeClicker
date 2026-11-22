use crate::ui::theme::BG;
use egui::{Id, Rect, Ui};
use std::f32::consts::PI;
use std::sync::Mutex;
use std::time::Duration;

const STRENGTH: f32 = 1.0;
const SIZE: f32 = 1.0;
const SPEED: f32 = 1.0;
pub const PARTICLES: usize = 12;
const FRAME_MS: u64 = 66;
const IDLE_MS: u64 = 120;
const CORNER: f32 = 20.0;
const FADE_SEC: f32 = 1.4;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GpuParams {
    rect: [f32; 4],
    misc: [f32; 4],
    bg: [f32; 4],
    ring_center: [f32; 4],
    glow: [[f32; 4]; 4],
    ring: [[f32; 4]; 3],
    part: [[f32; 4]; PARTICLES],
}

impl GpuParams {
    const ZERO: Self = Self {
        rect: [0.0; 4],
        misc: [0.0; 4],
        bg: [0.0; 4],
        ring_center: [0.0; 4],
        glow: [[0.0; 4]; 4],
        ring: [[0.0; 4]; 3],
        part: [[0.0; 4]; PARTICLES],
    };

    fn base(r: Rect, ppp: f32) -> Self {
        Self {
            rect: [r.left(), r.top(), r.width(), r.height()],
            misc: [ppp, CORNER, 0.0, 0.0],
            bg: [
                BG.r() as f32 / 255.0,
                BG.g() as f32 / 255.0,
                BG.b() as f32 / 255.0,
                0.0,
            ],
            ..Self::ZERO
        }
    }
}

static PARAMS: Mutex<GpuParams> = Mutex::new(GpuParams::ZERO);

pub fn params() -> GpuParams {
    *PARAMS.lock().unwrap_or_else(|e| e.into_inner())
}

fn a01(a: f32, vis: f32) -> f32 {
    (a * STRENGTH * vis / 255.0).clamp(0.0, 1.0)
}

fn rnd(i: usize, salt: f32) -> f32 {
    ((i as f32 * 12.9898 + salt * 78.233).sin() * 43758.547)
        .fract()
        .abs()
}

#[allow(clippy::too_many_arguments)]
fn glow(
    r: Rect,
    t: f32,
    k: f32,
    vis: f32,
    bx: f32,
    by: f32,
    ax: f32,
    ay: f32,
    fx: f32,
    fy: f32,
    ph: f32,
    rad: f32,
    al: f32,
) -> [f32; 4] {
    let (w, h) = (r.width(), r.height());
    let x = r.left() + w * (bx + ax * (t * fx + ph).sin());
    let y = r.top() + h * (by + ay * (t * fy + ph * 1.7).cos());
    let pulse = 1.0 + 0.18 * (t * 0.6 + ph).sin();
    [x, y, rad * SIZE * pulse, a01(al * (1.0 + 0.6 * k), vis)]
}

fn fill(p: &mut GpuParams, r: Rect, t: f32, k: f32, vis: f32) {
    let (w, h) = (r.width(), r.height());
    let c = r.center();

    p.glow = [
        glow(
            r, t, k, vis, 0.5, 0.0, 0.22, 0.08, 0.21, 0.17, 0.0, 430.0, 26.0,
        ), // pink
        glow(
            r, t, k, vis, 0.5, 0.45, 0.32, 0.22, 0.15, 0.23, 2.1, 360.0, 13.0,
        ), // violet
        glow(
            r, t, k, vis, 0.5, 0.5, 0.38, 0.20, 0.12, 0.18, 1.3, 300.0, 9.0,
        ), // rose
        glow(
            r, t, k, vis, 0.5, 1.0, 0.28, 0.06, 0.19, 0.13, 4.0, 380.0, 16.0,
        ), // orange
    ];

    p.ring_center = [c.x, c.y - 20.0, 0.0, 0.0];
    for i in 0..3 {
        let ph = (t * 0.1 + i as f32 / 3.0).fract();
        let fade = (1.0 - ph).powi(2);
        p.ring[i] = [
            70.0 + ph * 540.0,
            a01((7.0 + 16.0 * k) * fade, vis),
            0.0,
            0.0,
        ];
    }

    let travel = h - 60.0;
    for i in 0..PARTICLES {
        let x0 = rnd(i, 1.0);
        let y0 = rnd(i, 2.0);
        let spd = 7.0 + rnd(i, 3.0) * 16.0;
        let sz = 0.8 + rnd(i, 4.0) * 1.6;
        let ph = rnd(i, 5.0) * 6.28;

        let y = r.bottom() - 30.0 - ((y0 * travel + t * spd) % travel);
        let x = r.left() + 30.0 + x0 * (w - 60.0) + 16.0 * (t * 0.3 + ph).sin();

        let life = (r.bottom() - 30.0 - y) / travel;
        let edge = (life * PI).sin();
        let tw = 0.55 + 0.45 * (t * (0.8 + rnd(i, 6.0)) + ph).sin();
        let a = edge * tw * (60.0 + 60.0 * k);

        p.part[i] = [x, y, sz, a01(a, vis)];
    }
}

pub fn ambient(ui: &Ui, enabled: bool, running: bool) {
    let ctx = ui.ctx();
    let r = ui.max_rect();
    let mut p = GpuParams::base(r, ctx.pixels_per_point());

    let vis = ctx.animate_bool_with_time(Id::new("ambient_vis"), enabled, FADE_SEC);
    if vis > 0.001 {
        let focused = ctx.input(|i| i.focused);
        let ms = if focused { FRAME_MS } else { IDLE_MS };
        ctx.request_repaint_after(Duration::from_millis(ms));

        let t = ui.input(|i| i.time) as f32 * SPEED;
        let k = ctx.animate_bool_with_time(Id::new("ambient_run"), running, 1.2);
        fill(&mut p, r, t, k, vis);
    }

    *PARAMS.lock().unwrap_or_else(|e| e.into_inner()) = p;
}
