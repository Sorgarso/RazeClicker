use crate::{
    i18n::{T, tr},
    ui::{
        components::{logo_button::logo_button, model::modal},
        fonts::{
            fa_data::*,
            fonts::{icon_font, regular},
        },
        pages,
        shell::sidebar::VERSION,
        state::{Page, UiState},
        theme::*,
    },
    update::state::UpdateStatus,
};
use egui::{Align, Color32, Layout, RichText, vec2};

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
        "added" | "new" => Color32::from_rgb(80, 200, 120),
        "fixed" => Color32::from_rgb(90, 140, 255),
        "changed" | "improved" => Color32::from_rgb(240, 180, 70),
        "removed" => Color32::from_rgb(230, 100, 100),
        _ => ACCENT,
    }
}

pub fn changelog_dialog(ui: &mut egui::Ui, st: &mut UiState) {
    if !st.show_update_dialog || st.updater.get_changelog().is_empty() {
        return;
    }

    let changelog = st.updater.get_changelog().to_string();
    let mut close = false;

    modal(
        ui.ctx(),
        "change_logs_dialog",
        &mut st.show_update_dialog,
        380.0,
        340.0,
        |ui| {
            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.label(
                    RichText::new(tr(T::WhatsNew))
                        .font(regular(22.0))
                        .color(TEXT)
                        .strong(),
                );
                ui.label(
                    RichText::new(format!("{} {}", tr(T::Version), env!("CARGO_PKG_VERSION")))
                        .font(regular(11.0))
                        .color(MUTED),
                );
            });

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(2.0);

            egui::ScrollArea::vertical()
                .max_height(300.0)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    for line in parse_changelog(&changelog) {
                        match line {
                            Line::Heading(h) => {
                                ui.add_space(4.0);
                                ui.label(
                                    RichText::new(h.to_uppercase())
                                        .font(regular(14.0))
                                        .color(heading_color(h))
                                        .strong(),
                                );
                            }
                            Line::Bullet(b) => {
                                ui.horizontal_top(|ui| {
                                    ui.add_space(4.0);
                                    ui.label(RichText::new("•").color(ACCENT));
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

            // ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            //     let btn = egui::Button::new(
            //         RichText::new("Got it")
            //             .font(regular(13.0))
            //             .color(Color32::WHITE),
            //     )
            //     .fill(ACCENT)
            //     .corner_radius(8.0);
            //     if ui.add_sized([ui.available_width(), 30.0], btn).clicked() {
            //         close = true;
            //     }
            // });
        },
    );

    if close {
        st.show_update_dialog = false;
    }
}
