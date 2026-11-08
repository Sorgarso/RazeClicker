use egui_directx11::{Renderer, RendererOutput};
use windows::core::{Interface, Result};
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::DirectComposition::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

pub struct Dx {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    swap: IDXGISwapChain1,
    rtv: Option<ID3D11RenderTargetView>,
    renderer: Renderer,
}
const SUB: u8 = 1;
impl Dx {
    pub fn new(hwnd: HWND, width: u32, height: u32) -> windows::core::Result<Self> {
        unsafe {
            let mut device = None;
            let mut context = None;
            D3D11CreateDevice(
                None::<&IDXGIAdapter>,
                D3D_DRIVER_TYPE_HARDWARE,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_FLAG(0),
                Some(&[D3D_FEATURE_LEVEL_11_0]),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )?;
            let device = device.unwrap();
            let context = context.unwrap();

            let dxgi: IDXGIDevice1 = device.cast()?;
            let dxgi_device: IDXGIDevice = device.cast()?;
            let adapter = dxgi_device.GetAdapter()?;
            let factory: IDXGIFactory2 = adapter.GetParent()?;

            let desc = DXGI_SWAP_CHAIN_DESC1 {
                Width: width.max(1),
                Height: height.max(1),
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 2,
                Scaling: DXGI_SCALING_STRETCH,
                SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
                ..Default::default()
            };
            let swap = factory.CreateSwapChainForHwnd(&device, hwnd, &desc, None, None)?;

            let renderer = Renderer::new(&device)?;
            let mut me = Self {
                device,
                context,
                swap,
                rtv: None,
                renderer,
            };

            me.make_rtv()?;

            Ok(me)
        }
    }

    fn make_rtv(&mut self) -> Result<()> {
        unsafe {
            let back: ID3D11Texture2D = self.swap.GetBuffer(0)?;
            let mut rtv = None;
            self.device
                .CreateRenderTargetView(&back, None, Some(&mut rtv))?;
            self.rtv = rtv;
        }
        Ok(())
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }
        unsafe {
            self.context.OMSetRenderTargets(None, None);
            self.rtv = None;
            let _ = self
                .swap
                .ResizeBuffers(0, w, h, DXGI_FORMAT_UNKNOWN, DXGI_SWAP_CHAIN_FLAG(0));
        }
        let _ = self.make_rtv();
    }

    pub fn draw(
        &mut self,
        egui_ctx: &egui::Context,
        out: RendererOutput,
        clear: [f32; 4],
    ) -> (f32, f32) {
        let Some(rtv) = self.rtv.clone() else {
            return (0.0, 0.0);
        };
        let t0 = std::time::Instant::now();
        unsafe {
            self.context.ClearRenderTargetView(&rtv, &clear);
        }
        let _ = self.renderer.render(&self.context, &rtv, egui_ctx, out);
        let t1 = std::time::Instant::now();
        unsafe {
            let _ = self.swap.Present(1, DXGI_PRESENT(0));
        }
        let t2 = std::time::Instant::now();
        (
            (t1 - t0).as_secs_f32() * 1000.0,
            (t2 - t1).as_secs_f32() * 1000.0,
        )
    }
}
