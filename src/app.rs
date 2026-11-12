use crate::i18n::{self, Lang};
use crate::model::{config, settings::Settings};
use crate::update::Updater;
use crate::{engine, platform, ui};
use std::sync::Arc;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio Runtime");

    let settings = Arc::new(Settings::new());
    config::load().apply(&settings);

    let updater = Arc::new(Updater::new(runtime.handle().clone()));

    if std::env::args().any(|arg| arg == "--update") {
        let task = updater.get_changelogs_github("2.0.0".to_string());

        let updater_clone = updater.clone();

        runtime.spawn(async move {
            match task.await {
                Ok(Ok(changelog)) => {
                    updater_clone.set_changelog(changelog);
                }
                Ok(Err(error)) => eprintln!("Ошибка загрузки: {}", error),
                Err(error) => eprintln!("Ошибка Tokio: {}", error),
            }
        });
    }

    updater.check();
    let (events, hooks) = platform::windows::hooks::start();

    engine::start(settings.clone(), events);

    updater.check();
    ui::run(settings, updater);

    hooks.stop();
}
