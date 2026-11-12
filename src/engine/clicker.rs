use std::{
    sync::{Arc, atomic::Ordering::Relaxed},
    time::{Duration, Instant},
};

use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        Media::{timeBeginPeriod, timeEndPeriod},
        System::Threading::{
            CREATE_WAITABLE_TIMER_HIGH_RESOLUTION, CreateWaitableTimerExW, GetCurrentThread,
            SetThreadPriority, SetWaitableTimer, THREAD_PRIORITY_HIGHEST, TIMER_ALL_ACCESS,
            WaitForSingleObject,
        },
    },
    core::PCWSTR,
};

use crate::{
    model::settings::Settings,
    platform::windows::input::{click, click_at, key_press},
};

const SPIN_HR: Duration = Duration::from_micros(400);
const SPIN_FALLBACK: Duration = Duration::from_millis(2);
const MAX_SLEEP: Duration = Duration::from_millis(10);
const MIN_LAG: Duration = Duration::from_millis(50);
const IDLE_SLEEP: Duration = Duration::from_millis(10);

struct HrTimer(HANDLE);

impl HrTimer {
    fn new() -> Option<Self> {
        unsafe {
            CreateWaitableTimerExW(
                None,
                PCWSTR::null(),
                CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
                TIMER_ALL_ACCESS.0,
            )
            .ok()
            .map(Self)
        }
    }

    fn sleep(&self, d: Duration) {
        let due = -((d.as_nanos() / 100) as i64);
        unsafe {
            if SetWaitableTimer(self.0, &due, 0, None, None, false).is_ok() {
                WaitForSingleObject(self.0, 50);
            }
        }
    }
}

impl Drop for HrTimer {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

pub fn start(s: Arc<Settings>) {
    std::thread::Builder::new()
        .name("clicker".into())
        .spawn(move || {
            unsafe {
                let _ = SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_HIGHEST);
            }

            let timer = HrTimer::new();
            let legacy = timer.is_none();
            if legacy {
                unsafe {
                    timeBeginPeriod(1);
                }
            }

            run(&s, &timer);

            if legacy {
                unsafe {
                    timeEndPeriod(1);
                }
            }
        })
        .expect("clicker thread");
}

fn run(s: &Settings, timer: &Option<HrTimer>) {
    let mut running = false;
    let mut start = Instant::now();
    let mut n: u64 = 0;
    let mut interval = Duration::from_secs(1);
    let mut points: Vec<_> = Vec::new();
    let mut idx: usize = 0;

    loop {
        if !s.enabled.load(Relaxed) {
            running = false;
            std::thread::sleep(IDLE_SLEEP);
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
        if !wait_until(deadline, s, timer) {
            running = false;
            continue;
        }

        if !points.is_empty() {
            let p = points[idx % points.len()];
            idx += 1;
            click_at(s.button.load(Relaxed), p.x as i32, p.y as i32);
        } else if s.mode.load(Relaxed) == 0 {
            click(s.button.load(Relaxed));
        } else {
            key_press(s.key_button.load(Relaxed), 10);
        }
        n += 1;

        let lag = Instant::now().saturating_duration_since(deadline);
        if lag > MIN_LAG.max(interval * 5) {
            start = Instant::now();
            n = 0;
        }
    }
}

fn wait_until(deadline: Instant, s: &Settings, timer: &Option<HrTimer>) -> bool {
    let spin = if timer.is_some() {
        SPIN_HR
    } else {
        SPIN_FALLBACK
    };

    loop {
        if !s.enabled.load(Relaxed) {
            return false;
        }
        let now = Instant::now();
        if now >= deadline {
            return true;
        }
        let rem = deadline - now;

        if rem > spin + Duration::from_micros(600) {
            let nap = (rem - spin).min(MAX_SLEEP);
            match timer {
                Some(t) => t.sleep(nap),
                None => std::thread::sleep(nap),
            }
        } else {
            std::hint::spin_loop();
        }
    }
}
