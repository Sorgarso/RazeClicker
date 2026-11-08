use std::sync::{Arc, atomic::Ordering::Relaxed, mpsc::Receiver};

use crate::{
    engine::recorder,
    model::settings::{Settings, TOGGLE},
    platform::windows::hooks::{Event, InputEvent, MouseBtn},
};

const VK_ESCAPE: u32 = 0x1B;

pub fn start(s: Arc<Settings>, rx: Receiver<Event>) {
    let track = s.rec_track.lock().unwrap().clone();
    std::thread::spawn(move || {
        let mut held = false;

        let mut rec = super::recorder::Recorder::new();
        while let Ok(e) = rx.recv() {
            rec.handle(&s, &e);
            match e.ev {
                InputEvent::KeyDown(vk) if vk as usize == s.start_hotkey.load(Relaxed) => {
                    if recorder::is_recording(&s) {
                        recorder::stop_record(&s);
                    } else if recorder::is_playing(&s) {
                        recorder::stop_play(&s);
                    } else {
                        recorder::begin_record(&s);
                    }
                }
                InputEvent::KeyDown(vk) if vk as usize == s.play_hotkey.load(Relaxed) => {
                    if recorder::is_recording(&s) {
                        recorder::stop_record(&s);
                    } else if recorder::is_playing(&s) {
                        recorder::stop_play(&s);
                    } else {
                        recorder::play(&s);
                    }
                }
                InputEvent::PickedLocation(x, y) => {
                    let mut positions = s.pos.lock().unwrap();
                    positions.push(egui::Pos2::new(x as f32, y as f32));
                }
                InputEvent::KeyDown(vk) => {
                    let targets = [
                        (&s.capturing, &s.hotkey),
                        (&s.capturingKeybutton, &s.keyButton),
                        (&s.capturing_play_hotkey, &s.play_hotkey),
                        (&s.capturing_start_hotkey, &s.start_hotkey),
                    ];

                    if let Some((flag, slot)) = targets.iter().find(|(f, _)| f.load(Relaxed)) {
                        if vk != VK_ESCAPE {
                            slot.store(vk as usize, Relaxed);
                        }
                        flag.store(false, Relaxed);
                        held = true;
                        continue;
                    }

                    if vk as usize == s.hotkey.load(Relaxed) && !held {
                        held = true;
                        if s.toggle.load(Relaxed) {
                            s.enabled.fetch_xor(true, Relaxed);
                        } else {
                            s.enabled.store(true, Relaxed);
                        }
                    }
                }
                InputEvent::KeyUp(vk) => {
                    if vk as usize == s.hotkey.load(Relaxed) {
                        held = false;
                        if s.toggle.load(Relaxed) == false {
                            s.enabled.store(false, Relaxed);
                        }
                    }
                }

                InputEvent::MouseDown(btn) => {
                    let vk = match btn {
                        MouseBtn::Left => 0x01,
                        MouseBtn::Right => 0x02,
                        MouseBtn::Middle => 0x04,
                        MouseBtn::X1 => 0x05,
                        MouseBtn::X2 => 0x06,
                    };

                    if s.capturing.load(Relaxed) {
                        s.hotkey.store(vk, Relaxed);
                        s.capturing.store(false, Relaxed);
                        held = true;
                        continue;
                    }

                    if vk == s.hotkey.load(Relaxed) && !held {
                        held = true;
                        if s.toggle.load(Relaxed) == false {
                            s.enabled.store(true, Relaxed);
                        } else {
                            s.enabled.fetch_xor(true, Relaxed);
                        }
                    }
                }

                InputEvent::MouseUp(btn) => {
                    let vk = match btn {
                        MouseBtn::Left => 0x01,
                        MouseBtn::Right => 0x02,
                        MouseBtn::Middle => 0x04,
                        MouseBtn::X1 => 0x05,
                        MouseBtn::X2 => 0x06,
                    };

                    if vk == s.hotkey.load(Relaxed) {
                        held = false;
                        if s.toggle.load(Relaxed) == false {
                            s.enabled.store(false, Relaxed);
                        }
                    }
                }

                _ => {}
            }
        }
    });
}
fn activate(s: &Settings) {
    if s.mode.load(Relaxed) != 0 {
        s.enabled.store(true, Relaxed); // Hold
    } else {
        s.enabled.fetch_xor(true, Relaxed); // Toggle
    }
}
