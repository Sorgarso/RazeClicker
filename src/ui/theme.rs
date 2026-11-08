use egui::{Color32, CornerRadius, Stroke, Theme, Vec2};

pub const BG: Color32 = Color32::from_rgb(11, 12, 17);
pub const SIDEBAR: Color32 = Color32::from_rgb(14, 15, 21);
pub const CARD: Color32 = Color32::from_rgb(18, 20, 28);
pub const CARD_BORDER: Color32 = Color32::from_rgb(33, 36, 48);
pub const WIDGET: Color32 = Color32::from_rgb(22, 24, 33);
pub const WIDGET_HOVER: Color32 = Color32::from_rgb(30, 33, 45);
pub const TEXT: Color32 = Color32::from_rgb(236, 238, 246);
pub const MUTED: Color32 = Color32::from_rgb(132, 138, 160);
pub const ICON: Color32 = Color32::from_rgb(160, 164, 190);
pub const ACCENT: Color32 = Color32::from_rgb(37, 99, 255);
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(70, 124, 255);
pub const ACCENT_DARK: Color32 = Color32::from_rgb(22, 52, 130);
pub const CLOSE_HOVER: Color32 = Color32::from_rgb(232, 65, 85);
pub const CLOSE_IDLE: Color32 = Color32::from_rgb(30, 30, 38);
pub const MIN_IDLE: Color32 = Color32::from_rgb(30, 30, 38);
pub const MIN_HOVER: Color32 = Color32::from_rgb(52, 52, 66);
pub const DANGER_HOVER: Color32 = Color32::from_rgb(232, 65, 85);
pub const DANGER_IDLE: Color32 = Color32::from_rgb(202, 65, 85);
pub const DANGER_FILLED: Color32 = Color32::from_rgb(26, 24, 32);

pub const ROW_H: f32 = 34.0;
pub const LABEL_W: f32 = 96.0;

pub const MARGIN_BASE: i8 = 18;

pub fn apply_theme(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();
    v.panel_fill = BG;
    v.window_fill = CARD;
    v.extreme_bg_color = Color32::from_rgb(9, 10, 14);
    v.faint_bg_color = CARD;
    v.override_text_color = Some(TEXT);
    v.selection.bg_fill = ACCENT.gamma_multiply(0.35);
    v.selection.stroke = Stroke::new(1.0, ACCENT);
    v.window_corner_radius = CornerRadius::same(10);

    let w = &mut v.widgets;
    w.noninteractive.bg_fill = CARD;
    w.noninteractive.bg_stroke = Stroke::new(1.0, CARD_BORDER);
    w.inactive.bg_fill = WIDGET;
    w.inactive.weak_bg_fill = WIDGET;
    w.inactive.bg_stroke = Stroke::new(1.0, CARD_BORDER);
    w.inactive.corner_radius = CornerRadius::same(8);
    w.hovered.bg_fill = WIDGET_HOVER;
    w.hovered.weak_bg_fill = WIDGET_HOVER;
    w.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    w.hovered.corner_radius = CornerRadius::same(8);
    w.active.bg_fill = ACCENT;
    w.active.weak_bg_fill = ACCENT;
    w.active.bg_stroke = Stroke::new(1.0, ACCENT_HOVER);
    w.active.corner_radius = CornerRadius::same(8);
    ctx.set_visuals(v);

    ctx.style_mut_of(Theme::Dark, |s| {
        s.spacing.item_spacing = Vec2::new(10.0, 12.0);
        s.spacing.button_padding = Vec2::new(16.0, 10.0);
        s.spacing.interact_size.y = 36.0;
        s.animation_time = 0.0;
    });
}
