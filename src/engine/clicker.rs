use std::{
    sync::{Arc, atomic::Ordering::Relaxed},
    time::{Duration, Instant},
};

use windows::Win32::{
    Media::timeBeginPeriod,
    System::Threading::{GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_HIGHEST},
};

use crate::{
    model::settings::Settings,
    platform::windows::input::{click, click_at, key_press, move_mouse},
};
const SPIN_MARGIN: Duration = Duration::from_millis(2);
const MAX_SLEEP: Duration = Duration::from_millis(10);
const MIN_LAG: Duration = Duration::from_millis(50);

pub fn start(s: Arc<Settings>) {
    std::thread::Builder::new()
        .name("clicker".into())
        .spawn(move || {
            unsafe {
                timeBeginPeriod(1);
                let _ = SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_HIGHEST);
            }
            run(&s);
        })
        .expect("clicker thread");
}

fn run(s: &Settings) {
    let mut running = false;
    let mut start = Instant::now();
    let mut n: u64 = 0;
    let mut interval = Duration::from_secs(1);
    let mut points: Vec<_> = Vec::new();
    let mut idx: usize = 0;

    loop {
        if !s.enabled.load(Relaxed) {
            running = false;
            std::thread::sleep(Duration::from_millis(1));
            continue;
        }

        let cur_interval = s.click_interval();
        if !running || cur_interval != interval {
            interval = cur_interval;
            points = if s.pick_location_enabled.load(Relaxed) {
                s.pos.lock().unwrap().clone()
            } else {
                Vec::new()
            };
            idx = 0;
            start = Instant::now();
            n = 0;
            running = true;
        }

        let deadline = start + interval.mul_f64(n as f64);
        if !wait_until(deadline, s) {
            running = false;
            continue;
        }

        if !points.is_empty() {
            let p = points[idx % points.len()];
            idx += 1;
            click_at(s.button.load(Relaxed), p.x as i32, p.y as i32);
        } else {
            let mode = s.mode.load(Relaxed);

            if mode == 0 {
                click(s.button.load(Relaxed));
            } else {
                key_press(s.key_button.load(Relaxed), 10);
            }
        }
        n += 1;

        let lag = Instant::now().saturating_duration_since(deadline);
        if lag > MIN_LAG.max(interval * 5) {
            start = Instant::now();
            n = 0;
        }
    }
}

fn wait_until(deadline: Instant, s: &Settings) -> bool {
    loop {
        if !s.enabled.load(Relaxed) {
            return false;
        }
        let now = Instant::now();
        if now >= deadline {
            return true;
        }
        let rem = deadline - now;
        if rem > SPIN_MARGIN + Duration::from_millis(1) {
            std::thread::sleep((rem - SPIN_MARGIN).min(MAX_SLEEP));
        } else {
            std::hint::spin_loop();
        }
    }
}
