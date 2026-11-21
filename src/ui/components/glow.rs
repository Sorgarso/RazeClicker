use egui::{Color32, Mesh, Painter, Pos2, Shape, vec2};
use std::f32::consts::TAU;

pub fn radial_glow(p: &Painter, center: Pos2, inner_r: f32, outer_r: f32, color: Color32) {
    radial_glow_n(p, center, inner_r, outer_r, color, 40);
}

pub fn radial_glow_n(
    p: &Painter,
    center: Pos2,
    inner_r: f32,
    outer_r: f32,
    color: Color32,
    n: usize,
) {
    p.add(Shape::mesh(glow_mesh(center, inner_r, outer_r, color, n)));
}

pub fn glow_mesh(center: Pos2, inner_r: f32, outer_r: f32, color: Color32, n: usize) -> Mesh {
    let mut mesh = Mesh::default();

    let (sn, cs) = (TAU / n as f32).sin_cos();
    let mut dir = vec2(1.0, 0.0);
    let mut rotate = |d: egui::Vec2| vec2(d.x * cs - d.y * sn, d.x * sn + d.y * cs);

    mesh.colored_vertex(center, color);

    if inner_r <= 2.0 {
        mesh.vertices.reserve(n);
        mesh.indices.reserve(n * 3);
        for _ in 0..n {
            mesh.colored_vertex(center + dir * outer_r, Color32::TRANSPARENT);
            dir = rotate(dir);
        }
        for i in 0..n {
            let j = (i + 1) % n;
            mesh.add_triangle(0, 1 + i as u32, 1 + j as u32);
        }
        return mesh;
    }

    mesh.vertices.reserve(2 * n);
    mesh.indices.reserve(n * 9);
    for _ in 0..n {
        mesh.colored_vertex(center + dir * inner_r, color);
        mesh.colored_vertex(center + dir * outer_r, Color32::TRANSPARENT);
        dir = rotate(dir);
    }
    for i in 0..n {
        let j = (i + 1) % n;
        let (a, c) = (1 + 2 * i as u32, 2 + 2 * i as u32);
        let (b, d) = (1 + 2 * j as u32, 2 + 2 * j as u32);
        mesh.add_triangle(0, a, b);
        mesh.add_triangle(a, c, d);
        mesh.add_triangle(a, d, b);
    }
    mesh
}
