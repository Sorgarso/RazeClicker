use egui::{Color32, Context, CornerRadius, Id, LayerId, Order, Stroke, StrokeKind};

const CORNER: u8 = 20;

pub fn window_border(ctx: &Context) {
    let rect = ctx.content_rect();
    let p = ctx.layer_painter(LayerId::new(Order::Tooltip, Id::new("window_border")));

    p.rect_stroke(
        rect.shrink(0.5),
        CornerRadius::same(CORNER),
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 22)),
        StrokeKind::Inside,
    );
}
