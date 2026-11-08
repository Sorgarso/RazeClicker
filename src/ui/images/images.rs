use crate::ui::assets;
use egui::{ColorImage, Context, Id, Image, TextureHandle, TextureOptions, Ui, Vec2};

#[derive(Clone, Copy)]
pub enum Img {
    Logo,
}

impl Img {
    fn key(self) -> &'static str {
        match self {
            Img::Logo => "logo",
        }
    }

    fn bytes(self) -> &'static [u8] {
        match self {
            Img::Logo => assets::logo::LOGO,
        }
    }
}

pub fn texture(ctx: &Context, img: Img) -> TextureHandle {
    let id = Id::new(("tex", img.key()));
    if let Some(t) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return t;
    }
    let rgba = ::image::load_from_memory(img.bytes())
        .expect("bad image bytes")
        .to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let color = ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
    let tex = ctx.load_texture(img.key(), color, TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
    tex
}

pub fn texture_sized(ctx: &Context, img: Img, target_px: u32) -> TextureHandle {
    let id = Id::new(("tex", img.key(), target_px));
    if let Some(t) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return t;
    }
    let src = ::image::load_from_memory(img.bytes()).expect("bad image bytes");
    let resized = src.resize(
        target_px,
        target_px,
        ::image::imageops::FilterType::Lanczos3,
    );
    let rgba = resized.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let color = ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
    let tex = ctx.load_texture(img.key(), color, TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
    tex
}

pub fn show(ui: &mut Ui, img: Img, size: Vec2) {
    let px = (size.x.max(size.y) * ui.ctx().pixels_per_point()).ceil() as u32;
    let tex = texture_sized(ui.ctx(), img, px);
    ui.add(Image::new(&tex).fit_to_exact_size(size));
}
