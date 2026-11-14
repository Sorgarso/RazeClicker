use std::ffi::c_void;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use std::time::{Duration, Instant};

use egui::{Color32, FontFamily, FontId, Frame, Margin, RichText, Stroke, StrokeKind, Theme, Vec2};
use egui_winit::winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, DeviceEvents, EventLoop},
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::{Icon, Window, WindowId, WindowLevel},
};
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    Graphics::Dwm::DwmExtendFrameIntoClientArea,
    UI::{
        Controls::MARGINS,
        WindowsAndMessaging::{
            GWL_EXSTYLE, GetWindowLongPtrW, IMAGE_ICON, LR_DEFAULTSIZE, LR_SHARED, LoadImageW,
            SetWindowLongPtrW, WM_SETICON, WS_EX_LAYERED, WS_EX_NOREDIRECTIONBITMAP,
        },
    },
};
use windows::core::PCWSTR;

use super::dx11::Dx;
use crate::{
    model::{
        config::{self, Config},
        settings::Settings,
    },
    ui::{fonts::fonts::setup_fonts, state::UiState, theme::apply_theme},
    update::Updater,
};

const MAX_FPS: f32 = 60.0;
const DRAG_QUIET: Duration = Duration::from_millis(120);
const CLEAR: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

struct Gfx {
    window: Window,
    dx: Dx,
    state: egui_winit::State,
}

struct Host {
    ctx: egui::Context,
    st: UiState,
    gfx: Option<Gfx>,
    last_move: Option<Instant>,
    last_frame: Instant,
    last_saved: Config,
    dirty_since: Option<Instant>,
    applied_top: Option<bool>,
}
impl ApplicationHandler for Host {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.gfx.is_some() {
            return;
        }

        use egui_winit::winit::platform::windows::WindowAttributesExtWindows;

        let window = el
            .create_window(
                Window::default_attributes()
                    .with_title("RazeClicker")
                    .with_inner_size(LogicalSize::new(890.0, 460.0))
                    .with_resizable(false)
                    .with_decorations(false)
                    .with_transparent(true),
            )
            .expect("window");

        let hwnd = match window.window_handle().unwrap().as_raw() {
            RawWindowHandle::Win32(h) => HWND(h.hwnd.get() as *mut c_void),
            _ => panic!("not win32"),
        };

        unsafe {
            let instance = windows::Win32::System::LibraryLoader::GetModuleHandleW(None)
                .expect("GetModuleHandleW failed");

            let icon = LoadImageW(
                Some(instance.into()),
                PCWSTR(1 as *const u16),
                IMAGE_ICON,
                0,
                0,
                LR_DEFAULTSIZE | LR_SHARED,
            );

            if let Ok(icon) = icon {
                let _ = windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                    hwnd,
                    WM_SETICON,
                    Some(WPARAM(1)),
                    Some(LPARAM(icon.0 as isize)),
                );

                let _ = windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                    hwnd,
                    WM_SETICON,
                    Some(WPARAM(0)),
                    Some(LPARAM(icon.0 as isize)),
                );
            }
        }

        let size = window.inner_size();
        let dx = Dx::new(hwnd, size.width, size.height).expect("DX11 init failed");

        setup_fonts(&self.ctx);
        apply_theme(&self.ctx);
        self.ctx.options_mut(|o| o.reduce_texture_memory = true);

        let state = egui_winit::State::new(
            self.ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        window.request_redraw();
        el.set_control_flow(ControlFlow::Wait);
        self.gfx = Some(Gfx { window, dx, state });
    }
    fn new_events(&mut self, el: &ActiveEventLoop, cause: StartCause) {
        if let StartCause::ResumeTimeReached { .. } = cause {
            if let Some(g) = &self.gfx {
                g.window.request_redraw();
            }

            el.set_control_flow(ControlFlow::Wait);
        }
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Host {
            ctx,
            st,
            gfx,
            last_move,
            last_frame,
            last_saved,
            dirty_since,
            applied_top,
        } = self;
        let Some(g) = gfx.as_mut() else { return };

        if let WindowEvent::Moved(_) = event {
            *last_move = Some(Instant::now());
            return;
        }

        if let WindowEvent::RedrawRequested = event {
            *last_frame = Instant::now();

            if let Some(t) = *last_move {
                if t.elapsed() < DRAG_QUIET {
                    el.set_control_flow(ControlFlow::WaitUntil(t + DRAG_QUIET));
                    return;
                }
                *last_move = None;
            }
            let raw = g.state.take_egui_input(&g.window);
            let full = ctx.run_ui(raw, |ui| crate::ui::shell::draw(ui, st));

            let (renderer_out, platform, viewports) = egui_directx11::split_output(full);
            if let Some(v) = viewports.get(&egui::ViewportId::ROOT) {
                for cmd in &v.commands {
                    if let egui::ViewportCommand::InnerSize(sz) = cmd {
                        let _ = g
                            .window
                            .request_inner_size(LogicalSize::new(sz.x as f64, sz.y as f64));
                    }
                }
            }
            g.state.handle_platform_output(&g.window, platform);

            g.dx.draw(ctx, renderer_out, CLEAR);

            if st.settings.request_minimize.swap(false, Relaxed) {
                st.minimize = true;
            }

            if st.minimize {
                st.minimize = false;
                let _ = g.window.set_minimized(true);
            }

            if st.close {
                el.exit();
                return;
            }

            let top = st.settings.always_on_top.load(Relaxed);
            if *applied_top != Some(top) {
                g.window.set_window_level(if top {
                    WindowLevel::AlwaysOnTop
                } else {
                    WindowLevel::Normal
                });
                *applied_top = Some(top);
            }

            if st.drag {
                st.drag = false;
                let pos = ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default();
                let _ = g.window.drag_window();
                g.state
                    .egui_input_mut()
                    .events
                    .push(egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: egui::Modifiers::NONE,
                    });
                g.window.request_redraw();
            }

            let min_frame = Duration::from_secs_f32(1.0 / MAX_FPS);
            let delay = viewports
                .get(&egui::ViewportId::ROOT)
                .map(|v| v.repaint_delay)
                .unwrap_or(Duration::MAX);

            if delay.is_zero() {
                g.window.request_redraw();
                el.set_control_flow(ControlFlow::Wait);
            } else if delay < Duration::from_secs(3600) {
                el.set_control_flow(ControlFlow::WaitUntil(Instant::now() + delay));
            } else {
                el.set_control_flow(ControlFlow::Wait);
            }
            return;
        }

        let resp = g.state.on_window_event(&g.window, &event);
        if resp.repaint {
            let next = *last_frame + Duration::from_secs_f32(1.0 / MAX_FPS);
            if Instant::now() >= next {
                g.window.request_redraw();
            } else {
                el.set_control_flow(ControlFlow::WaitUntil(next));
            }
            let cur = Config::from_settings(&st.settings);
            if cur != *last_saved {
                match *dirty_since {
                    None => {
                        *dirty_since = Some(Instant::now());
                        ctx.request_repaint_after(Duration::from_secs(1)); // иначе окно уснёт и сейв не сработает
                    }
                    Some(t) if t.elapsed() >= Duration::from_secs(1) => {
                        config::save(&cur);
                        *last_saved = cur;
                        *dirty_since = None;
                    }
                    Some(_) => ctx.request_repaint_after(Duration::from_millis(300)),
                }
            }
        }

        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(s) => {
                g.dx.resize(s.width, s.height);
                g.window.request_redraw();
            }
            _ => {}
        }
    }
}

pub fn run(settings: Arc<Settings>, updater: Arc<Updater>) {
    let event_loop = EventLoop::new().expect("event loop");
    event_loop.listen_device_events(DeviceEvents::Never);
    event_loop.set_control_flow(ControlFlow::Wait);

    let settings_snapshot = Config::from_settings(&settings);
    let mut host = Host {
        ctx: egui::Context::default(),
        st: UiState::new(settings, updater),
        gfx: None,
        last_move: None,
        last_frame: Instant::now(),
        last_saved: settings_snapshot,
        dirty_since: None,
        applied_top: None,
    };
    event_loop.run_app(&mut host).expect("run");
}
