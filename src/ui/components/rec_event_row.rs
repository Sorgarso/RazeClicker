use crate::model::key::key_name;
use crate::model::recording::{Action, RecEvent};
use crate::ui::fonts::fonts::regular;
use crate::ui::theme::*;
use egui::{Align2, Color32, CornerRadius, FontFamily, FontId, Sense, Ui, Vec2};
pub fn rec_event_row(ui: &mut Ui, index: usize, event: RecEvent) {
    let height = 42.0;
    let width = ui.available_width();

    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

    let painter = ui.painter();

    let bg = if response.hovered() {
        WIDGET_HOVER
    } else {
        WIDGET
    };

    painter.rect_filled(rect, CornerRadius::same(9), bg);

    let muted = MUTED;
    let text = TEXT;
    let mono = regular(12.0);
    let small = regular(11.0);
    let number_rect =
        egui::Rect::from_min_size(rect.left_top() + Vec2::new(7.0, 8.0), Vec2::new(26.0, 26.0));

    painter.rect_filled(
        number_rect,
        CornerRadius::same(6),
        ACCENT.gamma_multiply(0.18),
    );

    painter.text(
        number_rect.center(),
        Align2::CENTER_CENTER,
        format!("{index:02}"),
        mono.clone(),
        ACCENT_HOVER,
    );

    let mut x = number_rect.right() + 12.0;

    let (title, subtitle, color) = match event.action {
        Action::Key { vk, down } => (
            if down { "KEY DOWN" } else { "KEY UP" },
            format!("{}", key_name(vk as usize)),
            if down {
                Color32::from_rgb(100, 190, 255)
            } else {
                MUTED
            },
        ),

        Action::Button { btn, down } => {
            let name = match btn {
                0 => "LEFT",
                1 => "RIGHT",
                2 => "MIDDLE",
                3 => "X1",
                4 => "X2",
                _ => "UNKNOWN",
            };

            (
                if down { "MOUSE DOWN" } else { "MOUSE UP" },
                name.to_string(),
                if down {
                    Color32::from_rgb(255, 170, 90)
                } else {
                    MUTED
                },
            )
        }

        Action::Move { x, y } => (
            "MOUSE MOVE",
            format!("X {:>5}  Y {:>5}", x, y),
            Color32::from_rgb(120, 220, 160),
        ),
    };

    let badge_width = 85.0;

    let badge_rect = egui::Rect::from_min_size(
        egui::pos2(x, rect.top() + 9.0),
        Vec2::new(badge_width, 24.0),
    );

    painter.rect_filled(
        badge_rect,
        CornerRadius::same(5),
        color.gamma_multiply(0.15),
    );

    painter.text(
        badge_rect.center(),
        Align2::CENTER_CENTER,
        title,
        regular(10.0),
        color,
    );

    x = badge_rect.right() + 10.0;

    painter.text(
        egui::pos2(x, rect.center().y),
        Align2::LEFT_CENTER,
        subtitle,
        mono,
        text,
    );

    let time = format_time(event.t_us);
    painter.text(
        egui::pos2(rect.right() - 8.0, rect.center().y),
        Align2::RIGHT_CENTER,
        time,
        small,
        muted,
    );
}

fn format_time(us: u64) -> String {
    let total_ms = us / 1000;

    if total_ms < 1000 {
        format!("{} ms", total_ms)
    } else if total_ms < 60_000 {
        format!("{:.2} s", us as f64 / 1_000_000.0)
    } else {
        format!(
            "{}:{:02}.{:03}",
            total_ms / 60_000,
            (total_ms / 1000) % 60,
            total_ms % 1000
        )
    }
}
