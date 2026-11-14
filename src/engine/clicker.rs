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

struct Rng(u64);

impl Rng {
    fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E37_79B9_7F4A_7C15);
        Self(seed | 1)
    }

    fn signed(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
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
    let mut next = Instant::now();
    let mut factor = 1.0_f64;
    let mut target = 1.0_f64;
    let mut retarget_at = Instant::now();
    let mut interval = Duration::from_secs(1);
    let mut points: Vec<_> = Vec::new();
    let mut idx: usize = 0;
    let mut rng = Rng::new();
    let mut done: u64 = 0;

    loop {
        if !s.enabled.load(Relaxed) | !s.enable_clicker.load(Relaxed) {
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
            next = start;
            n = 0;
            factor = 1.0;
            target = 1.0;
            retarget_at = start;
            running = true;
            done = 0;
        }

        let pct = if s.jitter_enabled.load(Relaxed) {
            s.jitter.load(Relaxed).min(45)
        } else {
            0
        };
        let deadline = if pct == 0 {
            start + interval.mul_f64(n as f64)
        } else {
            next
        };

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
        done += 1;

        if s.click_limit_enabled.load(Relaxed) {
            let limit = s.click_limit.load(Relaxed) as u64;
            if limit > 0 && done >= limit {
                s.enabled.store(false, Relaxed);
                running = false;
                continue;
            }
        }

        if pct > 0 {
            let p = pct as f64 / 100.0;
            let now = Instant::now();

            if now >= retarget_at {
                target = 1.0 + p * rng.signed();
                let ms = 300.0 + 700.0 * rng.signed().abs();
                retarget_at = now + Duration::from_millis(ms as u64);
            }

            let dt = (interval.as_secs_f64() * factor) / 0.3;
            factor += (target - factor) * dt.min(1.0);

            next += interval.mul_f64(factor);
        }

        let ref_t = if pct == 0 {
            start + interval.mul_f64((n - 1) as f64)
        } else {
            next
        };
        let lag = Instant::now().saturating_duration_since(ref_t);
        if lag > MIN_LAG.max(interval * 5) {
            start = Instant::now();
            next = start;
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
