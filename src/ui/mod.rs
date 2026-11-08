pub mod assets;
pub mod backend;
pub mod components;
pub mod fonts;
pub mod images;
pub mod pages;
pub mod shell;
pub mod state;
pub mod theme;

use std::sync::Arc;

use crate::{model::settings::Settings, update::Updater};
pub fn run(settings: Arc<Settings>, updater: Arc<Updater>) {
    backend::window::run(settings, updater);
}
