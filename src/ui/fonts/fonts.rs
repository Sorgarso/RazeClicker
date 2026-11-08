use crate::ui::assets::{font_awesome, inter_regular};
use egui::{FontData, FontDefinitions, FontFamily, FontId};
use std::sync::Arc;
pub fn icon_family() -> FontFamily {
    FontFamily::Name("icons".into())
}

pub fn icon_font(size: f32) -> FontId {
    FontId::new(size, icon_family())
}

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "inter_regular".to_owned(),
        Arc::new(FontData::from_static(
            crate::ui::assets::inter_regular::INTER_REGULAR,
        )),
    );

    fonts.font_data.insert(
        "font_awesome".to_owned(),
        Arc::new(FontData::from_static(
            crate::ui::assets::font_awesome::FONT_AWESOME,
        )),
    );

    fonts.families.insert(
        FontFamily::Name("inter_regular".into()),
        vec!["inter_regular".to_owned()],
    );

    fonts
        .families
        .insert(icon_family(), vec!["font_awesome".to_owned()]);

    ctx.set_fonts(fonts);
}

pub fn regular(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("inter_regular".into()))
}
