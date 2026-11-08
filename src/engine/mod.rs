mod clicker;
mod hotkey;
pub mod recorder;

use crate::model::settings::Settings;
use crate::platform::windows::hooks::Event;
use std::sync::{mpsc::Receiver, Arc};

pub fn start(settings: Arc<Settings>, events: Receiver<Event>) {
    hotkey::start(settings.clone(), events);
    clicker::start(settings);
}
