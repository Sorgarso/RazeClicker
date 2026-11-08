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
    cps: AtomicU32,          // f32 как биты
    pub unit: AtomicUsize,   // 0=s 1=m 2=h 3=d
    pub button: AtomicUsize, // 0=Left 1=Right
    pub mode: AtomicUsize,
    pub toggle: AtomicBool,
    pub enabled: AtomicBool,
    pub hotkey: AtomicUsize,    // VK-код
    pub keyButton: AtomicUsize, // VK-код
    pub capturing: AtomicBool,
    pub capturingKeybutton: AtomicBool,
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
}

impl Settings {
    pub fn new() -> Self {
        Self {
            cps: AtomicU32::new(50.0f32.to_bits()),
            unit: AtomicUsize::new(0),
            button: AtomicUsize::new(0),
            mode: AtomicUsize::new(MOUSE_MODE),
            toggle: AtomicBool::new(TOGGLE),
            enabled: AtomicBool::new(false),
            hotkey: AtomicUsize::new(0x73), // F4
            keyButton: AtomicUsize::new(0x46),
            capturing: AtomicBool::new(false),
            capturingKeybutton: AtomicBool::new(false),
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
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
