use crate::i18n::{self, Lang};
use crate::model::{config, settings::Settings};
use crate::update::Updater;
use crate::{engine, platform, ui};
use std::sync::Arc;

pub fn run() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio Runtime");

    let settings = Arc::new(Settings::new());
    config::load().apply(&settings);

    let updater = Arc::new(Updater::new(runtime.handle().clone()));

    let (events, hooks) = platform::windows::hooks::start();

    engine::start(settings.clone(), events);

    updater.check();
    ui::run(settings, updater);

    hooks.stop();
}
