use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        OnceLock,
    },
    thread::JoinHandle,
    time::Instant,
};

use ::windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::{LibraryLoader::GetModuleHandleW, Threading::GetCurrentThreadId},
    UI::WindowsAndMessaging::*,
};
static PICK_LOCATION: AtomicBool = AtomicBool::new(false);

pub fn start_pick_location() {
    PICK_LOCATION.store(true, Ordering::SeqCst);
}

pub struct Hooks {
    thread_id: u32,
    thread: Option<JoinHandle<()>>,
}

impl Hooks {
    pub fn stop(mut self) {
        unsafe {
            let _ = PostThreadMessageW(self.thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
        }

        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    KeyDown(u32),
    KeyUp(u32),
    MouseDown(MouseBtn),
    MouseUp(MouseBtn),
    MouseMove(i32, i32),
    PickedLocation(i32, i32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseBtn {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

pub struct Event {
    pub at: Instant,
    pub ev: InputEvent,
}

static TX: OnceLock<Sender<Event>> = OnceLock::new();

fn send(ev: InputEvent) {
    if let Some(tx) = TX.get() {
        let _ = tx.send(Event {
            at: Instant::now(),
            ev,
        });
    }
}

unsafe extern "system" fn kb_proc(code: i32, w: WPARAM, l: LPARAM) -> LRESULT {
    if code >= 0 {
        let k = &*(l.0 as *const KBDLLHOOKSTRUCT);
        if k.flags.0 & LLKHF_INJECTED.0 == 0 {
            match w.0 as u32 {
                WM_KEYDOWN | WM_SYSKEYDOWN => send(InputEvent::KeyDown(k.vkCode)),
                WM_KEYUP | WM_SYSKEYUP => send(InputEvent::KeyUp(k.vkCode)),
                _ => {}
            }
        }
    }
    CallNextHookEx(None, code, w, l)
}

unsafe extern "system" fn mouse_proc(code: i32, w: WPARAM, l: LPARAM) -> LRESULT {
    if code >= 0 {
        let m = &*(l.0 as *const MSLLHOOKSTRUCT);

        let (x, y) = (m.pt.x, m.pt.y);
        if m.flags & LLMHF_INJECTED == 0 {
            match w.0 as u32 {
                WM_LBUTTONDOWN => {
                    let picking = PICK_LOCATION.swap(false, Ordering::SeqCst);

                    if picking {
                        send(InputEvent::PickedLocation(x, y));
                    } else {
                        send(InputEvent::MouseDown(MouseBtn::Left));
                    }
                }
                WM_LBUTTONUP => send(InputEvent::MouseUp(MouseBtn::Left)),
                WM_RBUTTONDOWN => send(InputEvent::MouseDown(MouseBtn::Right)),
                WM_RBUTTONUP => send(InputEvent::MouseUp(MouseBtn::Right)),
                WM_MBUTTONDOWN => send(InputEvent::MouseDown(MouseBtn::Middle)),
                WM_MBUTTONUP => send(InputEvent::MouseUp(MouseBtn::Middle)),
                WM_MOUSEMOVE => send(InputEvent::MouseMove(m.pt.x, m.pt.y)),
                WM_XBUTTONDOWN => {
                    let btn = (m.mouseData >> 16) as u16;

                    match btn {
                        1 => send(InputEvent::MouseDown(MouseBtn::X1)),
                        2 => send(InputEvent::MouseDown(MouseBtn::X2)),
                        _ => {}
                    }
                }

                WM_XBUTTONUP => {
                    let btn = (m.mouseData >> 16) as u16;

                    match btn {
                        1 => send(InputEvent::MouseUp(MouseBtn::X1)),
                        2 => send(InputEvent::MouseUp(MouseBtn::X2)),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    CallNextHookEx(None, code, w, l)
}

pub fn start() -> (Receiver<Event>, Hooks) {
    let (tx, rx) = channel();
    let _ = TX.set(tx);

    let (ready_tx, ready_rx) = channel();

    let thread = std::thread::spawn(move || unsafe {
        let thread_id = GetCurrentThreadId();

        let hmod = match GetModuleHandleW(None) {
            Ok(hmod) => hmod,
            Err(_) => return,
        };

        let kb = match SetWindowsHookExW(WH_KEYBOARD_LL, Some(kb_proc), Some(hmod.into()), 0) {
            Ok(hook) => hook,
            Err(_) => return,
        };

        let ms = match SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), Some(hmod.into()), 0) {
            Ok(hook) => hook,
            Err(_) => {
                let _ = UnhookWindowsHookEx(kb);
                return;
            }
        };

        let _ = ready_tx.send(thread_id);

        let mut msg = MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let _ = UnhookWindowsHookEx(kb);
        let _ = UnhookWindowsHookEx(ms);
    });

    let thread_id = ready_rx.recv().expect("Hook thread failed to start");

    (
        rx,
        Hooks {
            thread_id,
            thread: Some(thread),
        },
    )
}
