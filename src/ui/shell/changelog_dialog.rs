use crate::{
    i18n::{T, tr},
    ui::{
        components::{icon_badge::icon_badge, model::modal},
        fonts::{fa_data::*, fonts::regular},
        state::UiState,
        theme::*,
    },
};
use egui::{Color32, RichText, Sense, vec2};

enum Line<'a> {
    Heading(&'a str),
    Bullet(&'a str),
    Text(&'a str),
}

fn parse_changelog(src: &str) -> Vec<Line<'_>> {
    src.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.chars().all(|c| c == '-' || c == '=' || c == '_'))
        .map(|l| {
            if let Some(h) = l
                .trim_start_matches('#')
                .strip_prefix(' ')
                .filter(|_| l.starts_with('#'))
            {
                Line::Heading(h.trim())
            } else if let Some(b) = l.strip_prefix("- ").or_else(|| l.strip_prefix("* ")) {
                Line::Bullet(b.trim())
            } else {
                Line::Text(l)
            }
        })
        .collect()
}

fn heading_color(name: &str) -> Color32 {
    match name.to_lowercase().as_str() {
        "added" | "new" => Color32::from_rgb(110, 210, 150),
        "fixed" => Color32::from_rgb(120, 160, 255),
        "changed" | "improved" => Color32::from_rgb(240, 190, 100),
        "removed" => Color32::from_rgb(240, 120, 120),
        _ => START_PINK,
    }
}

pub fn changelog_dialog(ui: &mut egui::Ui, st: &mut UiState) {
    if !st.show_update_dialog || st.updater.get_changelog().is_empty() {
        return;
    }

    let changelog = st.updater.get_changelog().to_string();

    modal(
        ui.ctx(),
        "change_logs_dialog",
        &mut st.show_update_dialog,
        380.0,
        340.0,
        |ui| {
            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;

                let (r, _) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::hover());
                let p = ui.painter();
                p.rect_filled(r, 10, Color32::from_rgba_unmultiplied(222, 163, 195, 34));
                p.rect_stroke(
                    r,
                    10,
                    egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(222, 163, 195, 120)),
                    egui::StrokeKind::Inside,
                );
                p.text(
                    r.center(),
                    egui::Align2::CENTER_CENTER,
                    ICON_FA_STAR,
                    crate::ui::fonts::fonts::icon_font(15.0),
                    START_PINK,
                );

                ui.add_space(8.0);
                ui.label(
                    RichText::new(tr(T::WhatsNew))
                        .font(regular(20.0))
                        .color(TEXT),
                );
                ui.add_space(2.0);
                ui.label(
                    RichText::new(format!("{} {}", tr(T::Version), env!("CARGO_PKG_VERSION")))
                        .font(regular(11.0))
                        .color(MUTED),
                );
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .max_height(260.0)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 6.0;

                    for line in parse_changelog(&changelog) {
                        match line {
                            Line::Heading(h) => {
                                let col = heading_color(h);
                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    let (r, _) =
                                        ui.allocate_exact_size(vec2(8.0, 14.0), Sense::hover());
                                    ui.painter().circle_filled(r.center(), 3.0, col);
                                    ui.label(
                                        RichText::new(h.to_uppercase())
                                            .font(regular(12.0))
                                            .color(col),
                                    );
                                });
                            }
                            Line::Bullet(b) => {
                                ui.horizontal_top(|ui| {
                                    ui.add_space(14.0);
                                    let (r, _) =
                                        ui.allocate_exact_size(vec2(8.0, 16.0), Sense::hover());
                                    ui.painter().circle_filled(r.center(), 2.0, START_PINK);
                                    ui.add(
                                        egui::Label::new(
                                            RichText::new(b).font(regular(13.0)).color(TEXT),
                                        )
                                        .wrap(),
                                    );
                                });
                            }
                            Line::Text(t) => {
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(t).font(regular(13.0)).color(MUTED),
                                    )
                                    .wrap(),
                                );
                            }
                        }
                    }
                });
        },
    );
}
