use std::sync::Arc;

use crate::{model::settings::Settings, update::Updater};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Clicker,
    Keyboard,
    PickLocation,
    Recorder,
    Settings,
    About,
    Studio,
}

impl Page {
    pub fn window_size(self) -> (f32, f32) {
        match self {
            Page::Studio => (1270.0, 700.0),
            _ => (890.0, 460.0),
        }
    }
}

pub struct UiState {
    pub settings: Arc<Settings>,
    pub updater: Arc<Updater>,
    pub page: Page,
    pub drag: bool,
    pub close: bool,
    pub minimize: bool,
    pub show_dialog: bool,
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
        }
    }
}
