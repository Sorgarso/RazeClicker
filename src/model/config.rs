use super::settings::Settings;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(default)]
pub struct Config {
    pub version: u32,
    pub lang: u8,

    pub cps: f32,
    pub unit: usize,
    pub button: usize,
    pub mode: usize,
    pub toggle: bool,
    pub hotkey: usize,
    pub key_button: usize,
    pub pick_location_enabled: bool,
    pub points: Vec<(f32, f32)>,

    pub start_hotkey: usize,
    pub play_hotkey: usize,
    pub play_loops: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            lang: 0,
            cps: 50.0,
            unit: 0,
            button: 0,
            mode: 0,
            toggle: false,
            hotkey: 0x73,
            key_button: 0x20,
            pick_location_enabled: false,
            points: vec![],
            start_hotkey: 0x75,
            play_hotkey: 0x74,
            play_loops: 1,
        }
    }
}

impl Config {
    pub fn from_settings(s: &Settings) -> Self {
        Self {
            version: 1,
            lang: if crate::i18n::lang() == crate::i18n::Lang::Ru {
                1
            } else {
                0
            },
            cps: s.cps(),
            unit: s.unit.load(Relaxed),
            button: s.button.load(Relaxed),
            mode: s.mode.load(Relaxed),
            toggle: s.toggle.load(Relaxed),
            hotkey: s.hotkey.load(Relaxed),
            key_button: s.key_button.load(Relaxed),
            pick_location_enabled: s.pick_location_enabled.load(Relaxed),
            points: s.pos.lock().unwrap().iter().map(|p| (p.x, p.y)).collect(),
            start_hotkey: s.start_hotkey.load(Relaxed),
            play_hotkey: s.play_hotkey.load(Relaxed),
            play_loops: s.play_loops.load(Relaxed),
        }
    }

    pub fn apply(&self, s: &Settings) {
        use crate::i18n::{Lang, set_lang};
        set_lang(if self.lang == 1 { Lang::Ru } else { Lang::En });
        s.set_cps(self.cps);
        s.unit.store(self.unit, Relaxed);
        s.button.store(self.button, Relaxed);
        s.mode.store(self.mode, Relaxed);
        s.toggle.store(self.toggle, Relaxed);
        s.hotkey.store(self.hotkey, Relaxed);
        s.key_button.store(self.key_button, Relaxed);
        s.pick_location_enabled
            .store(self.pick_location_enabled, Relaxed);
        *s.pos.lock().unwrap() = self.points.iter().map(|&(x, y)| egui::pos2(x, y)).collect();
        s.start_hotkey.store(self.start_hotkey, Relaxed);
        s.play_hotkey.store(self.play_hotkey, Relaxed);
        s.play_loops.store(self.play_loops, Relaxed);
    }
}

fn path() -> PathBuf {
    let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    p.push("RazeClicker");
    let _ = std::fs::create_dir_all(&p);
    p.push("config.json");
    p
}

pub fn load() -> Config {
    std::fs::read_to_string(path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(c: &Config) {
    println!("saving to {:?}", path());
    if let Ok(s) = serde_json::to_string_pretty(c) {
        let tmp = path().with_extension("tmp");
        if std::fs::write(&tmp, s).is_ok() {
            let _ = std::fs::rename(tmp, path());
        }
    }
}
