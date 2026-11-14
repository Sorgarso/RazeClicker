use crate::{
    i18n::{Lang, T, lang, set_lang, tr},
    ui::{
        anim::animate_ease::*,
        components::side_button::side_panel_button,
        fonts::{fa_data::*, fonts::regular},
        images::{self, Img},
        state::{Page, UiState},
        theme::*,
    },
    update::state::UpdateStatus,
};
use egui::{Color32, CornerRadius, Frame, Margin, Rect, RichText, Shape, pos2};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn items() -> [(Page, &'static str, &'static str); 4] {
    [
        (Page::Clicker, ICON_FA_COMPUTER_MOUSE, tr(T::Clicker)),
        (Page::Advanced, ICON_FA_SLIDERS, tr(T::Advanced)),
        (
            Page::PickLocation,
            ICON_FA_MAP_LOCATION_DOT,
            tr(T::PickLocation),
        ),
        (Page::Recorder, ICON_FA_RECORD_VINYL, tr(T::Recorder)),
    ]
}
fn bottom_items() -> [(Page, &'static str, &'static str); 2] {
    [
        (Page::About, ICON_FA_CIRCLE_INFO, tr(T::About)),
        (Page::Settings, ICON_FA_GEAR, tr(T::Settings)),
    ]
}
pub fn show(ui: &mut egui::Ui, st: &mut UiState) {
    egui::Panel::left("side")
        .exact_size(180.0)
        .resizable(false)
        .show_separator_line(false)
        .frame(
            Frame::new()
                .fill(SIDEBAR)
                .inner_margin(Margin::same(16))
                .corner_radius(CornerRadius {
                    nw: 20,
                    sw: 20,
                    ne: 0,
                    se: 0,
                }),
        )
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                images::show(ui, Img::Logo, egui::vec2(90.0, 90.0));
                ui.label(RichText::new("RazeClicker").font(regular(20.0)));
            });
            ui.add_space(12.0);
            ui.spacing_mut().item_spacing.y = 4.0;

            let pill_slot = ui.painter().add(Shape::Noop);
            let mut selected_rect: Option<Rect> = None;

            for (page, icon, name) in items() {
                let resp = side_panel_button(ui, Some(icon), name, st.page == page);
                if st.page == page {
                    selected_rect = Some(resp.rect);
                }
                if resp.clicked() {
                    st.page = page;
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.spacing_mut().item_spacing.y = 4.0;
                for (page, icon, name) in bottom_items() {
                    let resp = side_panel_button(ui, Some(icon), name, st.page == page);
                    if st.page == page {
                        selected_rect = Some(resp.rect);
                    }
                    if resp.clicked() {
                        st.page = page;
                    }
                }
            });

            if let Some(target) = selected_rect {
                let ctx = ui.ctx();
                let id = egui::Id::new("side_pill");
                let top = animate_ease(ctx, id.with("top"), target.min.y, 0.35, ease_out_cubic);
                let bottom =
                    animate_ease(ctx, id.with("bottom"), target.max.y, 0.35, ease_out_cubic);
                let rect = Rect::from_min_max(pos2(target.min.x, top), pos2(target.max.x, bottom));
                ui.painter().set(
                    pill_slot,
                    Shape::rect_filled(rect, CornerRadius::same(8), ACCENT.gamma_multiply(0.25)),
                );
            }
        });
}
