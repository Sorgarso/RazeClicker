use auto_launch::AutoLaunchBuilder;

fn launcher() -> Option<auto_launch::AutoLaunch> {
    let exe = std::env::current_exe().ok()?;
    AutoLaunchBuilder::new()
        .set_app_name("RazeClicker")
        .set_app_path(exe.to_str()?)
        .build()
        .ok()
}

pub fn is_enabled() -> bool {
    launcher()
        .and_then(|l| l.is_enabled().ok())
        .unwrap_or(false)
}

pub fn set_enabled(on: bool) {
    if let Some(l) = launcher() {
        let _ = if on { l.enable() } else { l.disable() };
    }
}
