use std::sync::Arc;

use crate::{model::settings::Settings, update::Updater};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Clicker,
    Advanced,
    PickLocation,
    Recorder,
    Settings,
    About,
}

pub struct UiState {
    pub settings: Arc<Settings>,
    pub updater: Arc<Updater>,
    pub page: Page,
    pub drag: bool,
    pub close: bool,
    pub minimize: bool,
    pub show_dialog: bool,
    pub show_update_dialog: bool,
    pub last_sent_size: Option<egui::Vec2>,
}

impl UiState {
    pub fn new(settings: Arc<Settings>, updater: Arc<Updater>) -> Self {
        Self {
            settings,
            updater,
            page: Page::default(),
            drag: false,
            close: false,
            minimize: false,
            show_dialog: true,
            show_update_dialog: std::env::args().any(|arg| arg == "--update"),
            last_sent_size: None,
        }
    }
}
