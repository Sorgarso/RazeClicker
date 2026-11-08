#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod engine;
mod i18n;
mod model;
mod platform;
mod ui;
mod update;

use std::{fs::OpenOptions, io::Write, panic};

fn setup_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

        let message = format!("[{timestamp}] PANIC: {info}\n");

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("panic.log")
        {
            let _ = file.write_all(message.as_bytes());
        }
    }));
}
fn cleanup_old_version() {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(name) = exe.file_name() {
            let name = name.to_string_lossy();

            let old_exe = exe.with_file_name(format!("{}.old", name));
            let _ = std::fs::remove_file(old_exe);

            let old = exe.with_file_name(format!(
                "{}.old",
                name.strip_suffix(".exe").unwrap_or(&name)
            ));
            let _ = std::fs::remove_file(old);
        }
    }
}
fn main() {
    setup_panic_hook();

    cleanup_old_version();

    app::run();
}
