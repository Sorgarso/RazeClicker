use std::{
    sync::{atomic::Ordering::Relaxed, Arc},
    time::{Duration, Instant},
};

use windows::Win32::UI::WindowsAndMessaging::MrmResourceIndexerHandle;

use crate::{
    engine::hotkey,
    model::{recording::*, settings::Settings},
    platform::windows::{
        hooks::{self, Event, InputEvent, MouseBtn},
        input,
    },
};

const MOVE_MIN: Duration = Duration::from_millis(8);

pub fn begin_record(s: &Arc<Settings>) {
    if s.rec_state.load(Relaxed) != REC_IDLE {
        return;
    }
    s.rec_track.lock().unwrap().clear();
    s.rec_state.store(REC_RECORDING, Relaxed);
}

pub fn stop_record(s: &Arc<Settings>) {
    if s.rec_state.load(Relaxed) == REC_RECORDING {
        s.rec_state.store(REC_IDLE, Relaxed);
    }
}

pub fn stop_play(s: &Arc<Settings>) {
    if s.rec_state.load(Relaxed) == REC_PLAYING {
        s.rec_state.store(REC_IDLE, Relaxed);
    }
}

pub fn is_recording(s: &Arc<Settings>) -> bool {
    s.rec_state.load(Relaxed) == 1
}

pub fn is_playing(s: &Arc<Settings>) -> bool {
    s.rec_state.load(Relaxed) == 2
}

pub struct Recorder {
    start: Option<Instant>,
    last_move: Option<Instant>,
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            start: None,
            last_move: None,
        }
    }

    pub fn handle(&mut self, s: &Settings, e: &Event) {
        if s.rec_state.load(Relaxed) != REC_RECORDING {
            self.start = None;
            self.last_move = None;
            return;
        }

        let action = match e.ev {
            InputEvent::KeyDown(vk) => Action::Key { vk, down: true },
            InputEvent::KeyUp(vk) => Action::Key { vk, down: false },
            InputEvent::MouseDown(b) => Action::Button {
                btn: btn_id(b),
                down: true,
            },
            InputEvent::MouseUp(b) => Action::Button {
                btn: btn_id(b),
                down: false,
            },
            InputEvent::MouseMove(x, y) => {
                if self
                    .last_move
                    .map_or(false, |t| e.at.duration_since(t) < MOVE_MIN)
                {
                    return;
                }
                self.last_move = Some(e.at);
                Action::Move { x, y }
            }
            _ => return,
        };
        let start = *self.start.get_or_insert(e.at);
        let t_us = e.at.duration_since(start).as_micros() as u64;
        s.rec_track.lock().unwrap().push(RecEvent { t_us, action });
    }
}

fn btn_id(b: MouseBtn) -> u8 {
    match b {
        MouseBtn::Left => 0,
        MouseBtn::Right => 1,
        MouseBtn::Middle => 2,
        MouseBtn::X1 => 3,
        MouseBtn::X2 => 4,
    }
}

pub fn play(s: &Arc<Settings>) {
    if is_recording(s) || s.rec_state.load(Relaxed) == REC_PLAYING {
        return;
    }

    let mut idx = 0;
    let play_loop = if s.play_loops.load(Relaxed) == 0 {
        true
    } else {
        false
    };
    let track = s.rec_track.lock().unwrap().clone();

    if track.is_empty() {
        s.rec_state.store(REC_IDLE, Relaxed);
        return;
    }
    let s = Arc::clone(s);
    s.rec_state.store(REC_PLAYING, Relaxed);
    std::thread::spawn(move || {
        loop {
            let base = track[0].t_us;
            let start = Instant::now();
            let mut keys: Vec<u32> = vec![];
            let mut btns: Vec<u8> = vec![];
            for ev in &track {
                if !wait_until(start + Duration::from_micros(ev.t_us - base), &s) {
                    break;
                }
                match ev.action {
                    Action::Move { x, y } => input::mouse_move_abs(x, y),
                    Action::Button { btn, down } => {
                        input::mouse_button(btn, down);
                        if down {
                            btns.push(btn)
                        } else {
                            btns.retain(|b| *b != btn)
                        }
                    }
                    Action::Key { vk, down } => {
                        input::key_event(vk, down);
                        if down {
                            keys.push(vk)
                        } else {
                            keys.retain(|k| *k != vk)
                        }
                    }
                }
            }
            for k in keys {
                input::key_event(k, false);
            }
            for b in btns {
                input::mouse_button(b, false);
            }

            if s.rec_state.load(Relaxed) != REC_PLAYING {
                break;
            }

            if !play_loop {
                break;
            }
        }
        s.rec_state.store(REC_IDLE, Relaxed);
    });
}

fn wait_until(due: Instant, s: &Arc<Settings>) -> bool {
    loop {
        if s.rec_state.load(Relaxed) != REC_PLAYING {
            return false;
        }
        let now = Instant::now();
        if now >= due {
            return true;
        }
        let rem = due - now;
        if rem > Duration::from_millis(3) {
            std::thread::sleep(rem - Duration::from_millis(2));
        } else {
            std::hint::spin_loop();
        }
    }
}
