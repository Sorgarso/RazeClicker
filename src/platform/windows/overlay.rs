use ::windows::Win32::Foundation::*;
use ::windows::Win32::Graphics::Gdi::*;
use ::windows::Win32::System::LibraryLoader::GetModuleHandleW;
use ::windows::Win32::UI::WindowsAndMessaging::*;
use ::windows::core::w;
use std::sync::{Arc, atomic::Ordering::Relaxed};
use std::time::Duration;

use crate::model::settings::Settings;

const OFFSET_X: i32 = 10;
const OFFSET_Y: i32 = 10;
const OVL: i32 = 48;
const RADIUS: f32 = 10.0;
const LOGO_PX: u32 = 42;
const MARGIN: i32 = 14;

const BG: [f32; 4] = [11.0, 12.0, 17.0, 0.94];
const BORDER: [f32; 4] = [45.0, 50.0, 70.0, 1.0];

unsafe extern "system" fn wnd_proc(h: HWND, m: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    DefWindowProcW(h, m, w, l)
}

type Px = [f32; 4];

fn premul(c: [f32; 4], coverage: f32) -> Px {
    let a = c[3] * coverage;
    [c[0] / 255.0 * a, c[1] / 255.0 * a, c[2] / 255.0 * a, a]
}

fn over(dst: Px, src: Px) -> Px {
    let k = 1.0 - src[3];
    [
        src[0] + dst[0] * k,
        src[1] + dst[1] * k,
        src[2] + dst[2] * k,
        src[3] + dst[3] * k,
    ]
}

fn sdf_round_rect(x: f32, y: f32, half: f32, r: f32) -> f32 {
    let qx = (x - half).abs() - (half - r);
    let qy = (y - half).abs() - (half - r);
    let outside = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt();
    outside + qx.max(qy).min(0.0) - r
}

fn render_overlay(size: u32) -> Vec<u8> {
    let logo = ::image::load_from_memory(crate::ui::assets::logo::LOGO)
        .expect("bad logo bytes")
        .resize(LOGO_PX, LOGO_PX, ::image::imageops::FilterType::Lanczos3)
        .to_rgba8();
    let (lw, lh) = (logo.width(), logo.height());
    let (ox, oy) = ((size - lw) / 2, (size - lh) / 2);

    let half = size as f32 / 2.0;
    let mut out = vec![0u8; (size * size * 4) as usize];

    for y in 0..size {
        for x in 0..size {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let d = sdf_round_rect(fx, fy, half, RADIUS);

            let outer = (0.5 - d).clamp(0.0, 1.0);
            let inner = (0.5 - (d + 1.0)).clamp(0.0, 1.0);

            let mut px: Px = premul(BG, outer);
            px = over(px, premul(BG, inner));

            if x >= ox && x < ox + lw && y >= oy && y < oy + lh {
                let p = logo.get_pixel(x - ox, y - oy).0;
                let a = p[3] as f32 / 255.0;
                let l: Px = [
                    p[0] as f32 / 255.0 * a,
                    p[1] as f32 / 255.0 * a,
                    p[2] as f32 / 255.0 * a,
                    a,
                ];
                px = over(px, l);
            }

            let i = ((y * size + x) * 4) as usize;
            out[i] = (px[2] * 255.0).round() as u8;
            out[i + 1] = (px[1] * 255.0).round() as u8;
            out[i + 2] = (px[0] * 255.0).round() as u8;
            out[i + 3] = (px[3] * 255.0).round() as u8;
        }
    }
    out
}

pub fn start(s: Arc<Settings>) {
    let _ = std::thread::Builder::new()
        .name("overlay".into())
        .spawn(move || unsafe { run(&s) });
}

unsafe fn run(s: &Settings) {
    let Ok(hinst) = GetModuleHandleW(None) else {
        return;
    };
    let class = w!("RazeClickerOverlay");

    let wc = WNDCLASSW {
        lpfnWndProc: Some(wnd_proc),
        hInstance: hinst.into(),
        lpszClassName: class,
        ..Default::default()
    };
    RegisterClassW(&wc);

    let hwnd = match CreateWindowExW(
        WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
        class,
        w!(""),
        WS_POPUP,
        0,
        0,
        OVL,
        OVL,
        None,
        None,
        Some(hinst.into()),
        None,
    ) {
        Ok(h) => h,
        Err(_) => return,
    };

    let screen = GetDC(None);
    let mem = CreateCompatibleDC(Some(screen));
    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: OVL,
            biHeight: -OVL,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
    let Ok(bmp) = CreateDIBSection(Some(mem), &bmi, DIB_RGB_COLORS, &mut bits, None, 0) else {
        return;
    };
    SelectObject(mem, bmp.into());
    let px = render_overlay(OVL as u32);
    std::ptr::copy_nonoverlapping(px.as_ptr(), bits as *mut u8, px.len());

    let blend = BLENDFUNCTION {
        BlendOp: AC_SRC_OVER as u8,
        BlendFlags: 0,
        SourceConstantAlpha: 255,
        AlphaFormat: AC_SRC_ALPHA as u8,
    };
    let size = SIZE { cx: OVL, cy: OVL };
    let src = POINT { x: 0, y: 0 };
    let _ = UpdateLayeredWindow(
        hwnd,
        Some(screen),
        None,
        Some(&size),
        Some(mem),
        Some(&src),
        COLORREF(0),
        Some(&blend),
        ULW_ALPHA,
    );

    let mut shown = false;
    let mut shown = false;
    loop {
        let want =
            s.enabled.load(Relaxed) && s.overlay.load(Relaxed) && s.enable_clicker.load(Relaxed);
        if want != shown {
            if want {
                let sw = GetSystemMetrics(SM_CXSCREEN);
                let sh = GetSystemMetrics(SM_CYSCREEN);

                let (x, y) = (sw - OVL - MARGIN - OFFSET_X, MARGIN + OFFSET_Y);

                let _ = SetWindowPos(
                    hwnd,
                    Some(HWND_TOPMOST),
                    x,
                    y,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
                );
            } else {
                let _ = ShowWindow(hwnd, SW_HIDE);
            }
            shown = want;
        }

        let mut msg = MSG::default();
        while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
            let _ = DispatchMessageW(&msg);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
