use std::{
    sync::{
        Mutex,
        atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicUsize, Ordering::Relaxed},
    },
    time::Duration,
};

use egui::{Pos2, Vec2};

use crate::{
    model::recording::{REC_IDLE, RecEvent},
    platform::windows::hooks::Hooks,
};
pub const MOUSE_MODE: usize = 0;
pub const KEYBOARD_MODE: usize = 1;

pub const TOGGLE: bool = false;

pub struct Settings {
    cps: AtomicU32, // f32 как биты
    pub enable_clicker: AtomicBool,
    pub unit: AtomicUsize,   // 0=s 1=m 2=h 3=d
    pub button: AtomicUsize, // 0=Left 1=Right
    pub mode: AtomicUsize,
    pub toggle: AtomicBool,
    pub enabled: AtomicBool,
    pub hotkey: AtomicUsize,     // VK-код
    pub key_button: AtomicUsize, // VK-код
    pub capturing: AtomicBool,
    pub capturing_key_button: AtomicBool,
    pub pos: Mutex<Vec<Pos2>>,
    pub pick_location_enabled: AtomicBool,
    pub request_minimize: AtomicBool,
    pub hooks: Mutex<Option<Hooks>>,
    pub rec_state: AtomicU8, // REC_IDLE / RECORDING / PLAYING
    pub rec_track: Mutex<Vec<RecEvent>>,
    pub play_loops: AtomicU32,     // 0 = бесконечно
    pub play_hotkey: AtomicUsize,  // VK-код
    pub start_hotkey: AtomicUsize, // VK-код
    pub capturing_start_hotkey: AtomicBool,
    pub capturing_play_hotkey: AtomicBool,
    pub jitter: AtomicUsize,
    pub jitter_enabled: AtomicBool,
    pub overlay: AtomicBool,
    pub always_on_top: AtomicBool,
    pub click_limit_enabled: AtomicBool,
    pub click_limit: AtomicU32,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            cps: AtomicU32::new(50.0f32.to_bits()),
            enable_clicker: AtomicBool::new(true),
            unit: AtomicUsize::new(0),
            button: AtomicUsize::new(0),
            mode: AtomicUsize::new(MOUSE_MODE),
            toggle: AtomicBool::new(TOGGLE),
            enabled: AtomicBool::new(false),
            hotkey: AtomicUsize::new(0x73), // F4
            key_button: AtomicUsize::new(0x46),
            capturing: AtomicBool::new(false),
            capturing_key_button: AtomicBool::new(false),
            pos: Mutex::new(Vec::new()),
            pick_location_enabled: AtomicBool::new(false),
            request_minimize: AtomicBool::new(false),
            hooks: Mutex::new(None),
            rec_state: AtomicU8::new(REC_IDLE),
            rec_track: Mutex::new(Vec::new()),
            play_loops: AtomicU32::new(1),
            play_hotkey: AtomicUsize::new(0x75),
            start_hotkey: AtomicUsize::new(0x74),
            capturing_start_hotkey: AtomicBool::new(false),
            capturing_play_hotkey: AtomicBool::new(false),
            jitter: AtomicUsize::new(0),
            jitter_enabled: AtomicBool::new(false),
            overlay: AtomicBool::new(false),
            always_on_top: AtomicBool::new(false),
            click_limit_enabled: AtomicBool::new(false),
            click_limit: AtomicU32::new(100),
        }
    }

    pub fn cps(&self) -> f32 {
        f32::from_bits(self.cps.load(Relaxed))
    }
    pub fn set_cps(&self, v: f32) {
        self.cps.store(v.to_bits(), Relaxed);
    }

    pub fn click_interval(&self) -> Duration {
        let cps = self.cps().max(0.0001);
        let per_sec = match self.unit.load(Relaxed) {
            0 => cps,
            1 => cps / 60.0,
            2 => cps / 3600.0,
            _ => cps / 86400.0,
        };
        Duration::from_secs_f32(1.0 / per_sec.max(0.00001))
    }

    pub fn reset(&self) {
        self.set_cps(50.0);
        self.unit.store(0, Relaxed);
        self.button.store(0, Relaxed);
        self.mode.store(MOUSE_MODE, Relaxed);
        self.toggle.store(TOGGLE, Relaxed);
        self.enabled.store(false, Relaxed);

        self.hotkey.store(0x73, Relaxed); // F4
        self.key_button.store(0x46, Relaxed); // F
        self.start_hotkey.store(0x74, Relaxed); // F5
        self.play_hotkey.store(0x75, Relaxed); // F6

        self.capturing.store(false, Relaxed);
        self.capturing_key_button.store(false, Relaxed);
        self.capturing_start_hotkey.store(false, Relaxed);
        self.capturing_play_hotkey.store(false, Relaxed);

        self.pos.lock().unwrap().clear();
        self.pick_location_enabled.store(false, Relaxed);

        self.request_minimize.store(false, Relaxed);

        self.rec_state.store(REC_IDLE, Relaxed);
        self.rec_track.lock().unwrap().clear();
        self.play_loops.store(1, Relaxed);
        self.jitter.store(0, Relaxed);
        self.jitter_enabled.store(false, Relaxed);
        self.overlay.store(false, Relaxed);
        self.always_on_top.store(false, Relaxed);
        self.enable_clicker.store(false, Relaxed);
        self.click_limit_enabled.store(false, Relaxed);
        self.click_limit.store(100, Relaxed);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
