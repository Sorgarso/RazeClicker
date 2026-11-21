use crate::ui::components::glow::{radial_glow, radial_glow_n};
use crate::ui::theme::BG;
use egui::{Color32, Id, Painter, Pos2, Rect, Stroke, Ui, pos2};
use std::f32::consts::PI;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

const STRENGTH: f32 = 1.0;
const SIZE: f32 = 1.0;
const SPEED: f32 = 1.0;
const PARTICLES: usize = 12;
const FRAME_MS: u64 = 66;
const IDLE_MS: u64 = 120;
const CORNER: f32 = 20.0;
const STRIPS: usize = 8;
const FADE_SEC: f32 = 1.4;

static VIS: AtomicU32 = AtomicU32::new(0);

type Rgb = (u8, u8, u8);
const PINK: Rgb = (222, 163, 195);
const ORANGE: Rgb = (214, 140, 110);
const VIOLET: Rgb = (165, 135, 225);
const ROSE: Rgb = (240, 130, 170);

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Full,
    Blobs,
}

fn tint(c: Rgb, a: f32) -> Color32 {
    let vis = f32::from_bits(VIS.load(Ordering::Relaxed));
    Color32::from_rgba_unmultiplied(c.0, c.1, c.2, (a * STRENGTH * vis).clamp(0.0, 255.0) as u8)
}

fn rnd(i: usize, salt: f32) -> f32 {
    ((i as f32 * 12.9898 + salt * 78.233).sin() * 43758.547)
        .fract()
        .abs()
}

fn inset(d: f32) -> f32 {
    let d = d.clamp(0.0, CORNER);
    CORNER
        - (CORNER * CORNER - (CORNER - d) * (CORNER - d))
            .max(0.0)
            .sqrt()
}

struct Glow {
    pos: Pos2,
    rad: f32,
    color: Color32,
}

struct Scene {
    pink: Glow,
    violet: Glow,
    rose: Glow,
    orange: Glow,
}

#[allow(clippy::too_many_arguments)]
fn make_glow(
    r: Rect,
    t: f32,
    k: f32,
    color: Rgb,
    bx: f32,
    by: f32,
    ax: f32,
    ay: f32,
    fx: f32,
    fy: f32,
    ph: f32,
    rad: f32,
    al: f32,
) -> Glow {
    let (w, h) = (r.width(), r.height());
    let pos = pos2(
        r.left() + w * (bx + ax * (t * fx + ph).sin()),
        r.top() + h * (by + ay * (t * fy + ph * 1.7).cos()),
    );
    let pulse = 1.0 + 0.18 * (t * 0.6 + ph).sin();

    Glow {
        pos,
        rad: rad * SIZE * pulse,
        color: tint(color, al * (1.0 + 0.6 * k)),
    }
}

fn make_scene(r: Rect, t: f32, k: f32) -> Scene {
    Scene {
        pink: make_glow(
            r, t, k, PINK, 0.5, 0.0, 0.22, 0.08, 0.21, 0.17, 0.0, 430.0, 26.0,
        ),
        violet: make_glow(
            r, t, k, VIOLET, 0.5, 0.45, 0.32, 0.22, 0.15, 0.23, 2.1, 360.0, 13.0,
        ),
        rose: make_glow(
            r, t, k, ROSE, 0.5, 0.5, 0.38, 0.20, 0.12, 0.18, 1.3, 300.0, 9.0,
        ),
        orange: make_glow(
            r, t, k, ORANGE, 0.5, 1.0, 0.28, 0.06, 0.19, 0.13, 4.0, 380.0, 16.0,
        ),
    }
}

fn put(p: &Painter, zone: Rect, g: &Glow) {
    let dx = (zone.left() - g.pos.x).max(0.0).max(g.pos.x - zone.right());
    let dy = (zone.top() - g.pos.y).max(0.0).max(g.pos.y - zone.bottom());
    if dx * dx + dy * dy < g.rad * g.rad {
        radial_glow(p, g.pos, 1.0, g.rad, g.color);
    }
}

pub fn ambient(ui: &Ui, enabled: bool, running: bool) {
    let ctx = ui.ctx();
    let r = ui.max_rect();

    ui.painter()
        .with_clip_rect(r)
        .rect_filled(r, CORNER as u8, BG);

    let vis = ctx.animate_bool_with_time(Id::new("ambient_vis"), enabled, FADE_SEC);
    if vis <= 0.001 {
        return;
    }
    VIS.store(vis.to_bits(), Ordering::Relaxed);

    let focused = ctx.input(|i| i.focused);
    let ms = if focused { FRAME_MS } else { IDLE_MS };

    let t = ui.input(|i| i.time) as f32 * SPEED;
    let k = ctx.animate_bool_with_time(Id::new("ambient_run"), running, 1.2);

    let scene = make_scene(r, t, k);

    let draw = |zone: Rect, mode: Mode| {
        paint(
            &ui.painter().with_clip_rect(zone),
            zone,
            r,
            t,
            k,
            &scene,
            mode,
        );
    };

    draw(
        Rect::from_min_max(
            pos2(r.left() + CORNER, r.top()),
            pos2(r.right() - CORNER, r.bottom()),
        ),
        Mode::Full,
    );
    draw(
        Rect::from_min_max(
            pos2(r.left(), r.top() + CORNER),
            pos2(r.left() + CORNER, r.bottom() - CORNER),
        ),
        Mode::Blobs,
    );
    draw(
        Rect::from_min_max(
            pos2(r.right() - CORNER, r.top() + CORNER),
            pos2(r.right(), r.bottom() - CORNER),
        ),
        Mode::Blobs,
    );

    for i in 0..STRIPS {
        let d0 = CORNER * i as f32 / STRIPS as f32;
        let d1 = CORNER * (i + 1) as f32 / STRIPS as f32;
        let ins = inset((d0 + d1) * 0.5) + 0.5;

        for (y0, y1) in [
            (r.top() + d0, r.top() + d1),
            (r.bottom() - d1, r.bottom() - d0),
        ] {
            draw(
                Rect::from_min_max(pos2(r.left() + ins, y0), pos2(r.left() + CORNER, y1)),
                Mode::Blobs,
            );
            draw(
                Rect::from_min_max(pos2(r.right() - CORNER, y0), pos2(r.right() - ins, y1)),
                Mode::Blobs,
            );
        }
    }
}

fn paint(p: &Painter, zone: Rect, r: Rect, t: f32, k: f32, sc: &Scene, mode: Mode) {
    let (w, h) = (r.width(), r.height());
    let c = r.center();

    put(p, zone, &sc.pink);
    put(p, zone, &sc.violet);
    put(p, zone, &sc.rose);
    put(p, zone, &sc.orange);

    if mode != Mode::Full {
        return;
    }

    for i in 0..3 {
        let ph = (t * 0.1 + i as f32 / 3.0).fract();
        let fade = (1.0 - ph).powi(2);
        p.circle_stroke(
            pos2(c.x, c.y - 20.0),
            70.0 + ph * 540.0,
            Stroke::new(1.2, tint(PINK, (7.0 + 16.0 * k) * fade)),
        );
    }

    let travel = h - 60.0;
    for i in 0..PARTICLES {
        let x0 = rnd(i, 1.0);
        let y0 = rnd(i, 2.0);
        let spd = 7.0 + rnd(i, 3.0) * 16.0;
        let sz = 0.8 + rnd(i, 4.0) * 1.6;
        let ph = rnd(i, 5.0) * std::f32::consts::TAU;

        let y = r.bottom() - 30.0 - ((y0 * travel + t * spd) % travel);
        let x = r.left() + 30.0 + x0 * (w - 60.0) + 16.0 * (t * 0.3 + ph).sin();

        let life = (r.bottom() - 30.0 - y) / travel;
        let edge = (life * PI).sin();
        let tw = 0.55 + 0.45 * (t * (0.8 + rnd(i, 6.0)) + ph).sin();
        let color = if i % 4 == 0 { ORANGE } else { PINK };
        let a = edge * tw * (60.0 + 60.0 * k);

        if i % 8 == 0 {
            radial_glow_n(
                p,
                pos2(x, y),
                1.0,
                10.0 + sz * 7.0,
                tint(color, a * 0.28),
                12,
            );
        }
        p.circle_filled(pos2(x, y), sz, tint(color, a));
    }
}
