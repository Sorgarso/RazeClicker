use egui::{Color32, CornerRadius, Stroke, Theme, Vec2};

pub const BG: Color32 = Color32::from_rgb(15, 14, 19);
pub const CARD: Color32 = Color32::from_rgb(23, 21, 28);
pub const CARD_BORDER: Color32 = Color32::from_rgb(40, 36, 47);
pub const WIDGET: Color32 = Color32::from_rgb(29, 26, 35);
pub const WIDGET_HOVER: Color32 = Color32::from_rgb(40, 35, 46);

pub const TEXT: Color32 = Color32::from_rgb(240, 236, 244);
pub const MUTED: Color32 = Color32::from_rgb(138, 128, 150);
pub const ICON: Color32 = Color32::from_rgb(172, 160, 184);

pub const ACCENT: Color32 = Color32::from_rgb(217, 163, 191);
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(230, 182, 206);

pub const ON_ACCENT: Color32 = Color32::from_rgb(38, 24, 34);

pub const START_PINK: egui::Color32 = egui::Color32::from_rgb(222, 163, 195);
pub const START_PINK_DARK: egui::Color32 = egui::Color32::from_rgb(38, 28, 40);
pub const EYEBROW_ORANGE: egui::Color32 = egui::Color32::from_rgb(214, 140, 110);

pub const CLOSE_HOVER: Color32 = Color32::from_rgb(232, 90, 110);
pub const CLOSE_IDLE: Color32 = Color32::TRANSPARENT;

pub const MIN_IDLE: Color32 = Color32::TRANSPARENT;
pub const MIN_HOVER: Color32 = Color32::from_rgb(46, 41, 53);
// сделать прозрачность
pub const DANGER_HOVER: Color32 = Color32::from_rgb(232, 90, 110);
pub const DANGER_IDLE: Color32 = Color32::from_rgb(202, 80, 102);
pub const DANGER_FILLED: Color32 = Color32::from_rgb(34, 24, 30);

pub const ROW_H: f32 = 34.0;

pub const MARGIN_BASE: i8 = 18;

pub fn apply_theme(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();
    v.panel_fill = BG;
    v.window_fill = CARD;
    v.extreme_bg_color = Color32::from_rgb(11, 10, 14);
    v.faint_bg_color = CARD;
    v.override_text_color = Some(TEXT);
    v.selection.bg_fill = ACCENT.gamma_multiply(0.30);
    v.selection.stroke = Stroke::new(1.0, ACCENT);
    v.window_corner_radius = CornerRadius::same(14);

    let w = &mut v.widgets;
    w.noninteractive.bg_fill = CARD;
    w.noninteractive.bg_stroke = Stroke::new(1.0, CARD_BORDER);
    w.inactive.bg_fill = WIDGET;
    w.inactive.weak_bg_fill = WIDGET;
    w.inactive.bg_stroke = Stroke::new(1.0, CARD_BORDER);
    w.inactive.corner_radius = CornerRadius::same(10);
    w.hovered.bg_fill = WIDGET_HOVER;
    w.hovered.weak_bg_fill = WIDGET_HOVER;
    w.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    w.hovered.corner_radius = CornerRadius::same(10);
    w.active.bg_fill = ACCENT;
    w.active.weak_bg_fill = ACCENT;
    w.active.bg_stroke = Stroke::new(1.0, ACCENT_HOVER);
    w.active.fg_stroke = Stroke::new(1.0, ON_ACCENT);
    w.active.corner_radius = CornerRadius::same(10);
    ctx.set_visuals(v);

    ctx.style_mut_of(Theme::Dark, |s| {
        s.spacing.item_spacing = Vec2::new(10.0, 12.0);
        s.spacing.button_padding = Vec2::new(16.0, 10.0);
        s.spacing.interact_size.y = 36.0;
        s.animation_time = 0.0;
    });
}
