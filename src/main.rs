#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod engine;
use std::time::Duration;
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
        let old = exe.with_extension("old");

        std::thread::spawn(move || {
            for _ in 0..20 {
                match std::fs::remove_file(&old) {
                    Ok(_) => break,
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        std::thread::sleep(Duration::from_millis(500));
                    }
                    Err(_) => std::thread::sleep(Duration::from_millis(500)),
                }
            }
        });
    }
}
fn main() {
    setup_panic_hook();

    cleanup_old_version();

    app::run();
}
