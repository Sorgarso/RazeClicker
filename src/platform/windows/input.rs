use ::windows::Win32::UI::{
    Input::KeyboardAndMouse::*,
    WindowsAndMessaging::{
        GetSystemMetrics, SetCursorPos, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
        SM_YVIRTUALSCREEN,
    },
};
use std::{mem::size_of, time::Duration};

fn send(inputs: &[INPUT]) {
    unsafe {
        SendInput(inputs, size_of::<INPUT>() as i32);
    }
}

fn button_flags(button: usize) -> (MOUSE_EVENT_FLAGS, MOUSE_EVENT_FLAGS) {
    if button == 0 {
        (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP)
    } else {
        (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP)
    }
}

fn normalize(x: i32, y: i32) -> (i32, i32) {
    unsafe {
        let vx = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let vy = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let vw = GetSystemMetrics(SM_CXVIRTUALSCREEN).max(2);
        let vh = GetSystemMetrics(SM_CYVIRTUALSCREEN).max(2);
        (
            ((x - vx) as i64 * 65535 / (vw - 1) as i64) as i32,
            ((y - vy) as i64 * 65535 / (vh - 1) as i64) as i32,
        )
    }
}

fn mouse_input(dx: i32, dy: i32, flags: MOUSE_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx,
                dy,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn key_input(scan: u32, up: bool) -> INPUT {
    let mut flags = KEYEVENTF_SCANCODE;
    if matches!(scan >> 8, 0xE0 | 0xE1) {
        flags |= KEYEVENTF_EXTENDEDKEY;
    }
    if up {
        flags |= KEYEVENTF_KEYUP;
    }
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: (scan & 0xFF) as u16,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

pub fn click(button: usize) {
    let (down, up) = button_flags(button);
    send(&[mouse_input(0, 0, down), mouse_input(0, 0, up)]);
}

pub fn click_at(button: usize, x: i32, y: i32) {
    let (down, up) = button_flags(button);
    let (nx, ny) = normalize(x, y);
    let abs = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK;
    send(&[
        mouse_input(nx, ny, abs),
        mouse_input(nx, ny, abs | down),
        mouse_input(nx, ny, abs | up),
    ]);
}

pub fn key_press(vk: usize, hold_ms: u64) {
    let scan = unsafe { MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC_EX) };

    send(&[key_input(scan, false)]);
    std::thread::sleep(Duration::from_millis(hold_ms));
    send(&[key_input(scan, true)]);
}

pub fn move_mouse(x: i32, y: i32) {
    unsafe {
        let _ = SetCursorPos(x, y);
    }
}

pub fn mouse_move_abs(x: i32, y: i32) {
    let (nx, ny) = normalize(x, y);
    let f = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK;
    send(&[mouse_input(nx, ny, f)]);
}

pub fn mouse_button(btn: u8, down: bool) {
    let (flag, data) = match (btn, down) {
        (0, true) => (MOUSEEVENTF_LEFTDOWN, 0),
        (0, false) => (MOUSEEVENTF_LEFTUP, 0),
        (1, true) => (MOUSEEVENTF_RIGHTDOWN, 0),
        (1, false) => (MOUSEEVENTF_RIGHTUP, 0),
        (2, true) => (MOUSEEVENTF_MIDDLEDOWN, 0),
        (2, false) => (MOUSEEVENTF_MIDDLEUP, 0),
        (n, d) => (
            if d {
                MOUSEEVENTF_XDOWN
            } else {
                MOUSEEVENTF_XUP
            },
            n as u32 - 2,
        ),
    };
    let mut i = mouse_input(0, 0, flag);
    unsafe {
        i.Anonymous.mi.mouseData = data;
    }
    send(&[i]);
}

pub fn key_event(vk: u32, down: bool) {
    let scan = unsafe { MapVirtualKeyW(vk, MAPVK_VK_TO_VSC_EX) };
    send(&[key_input(scan, !down)]);
}
