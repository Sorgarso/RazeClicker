use std::ffi::c_void;
use std::mem::size_of;

use windows::Win32::Graphics::Direct3D::Fxc::{D3DCOMPILE_OPTIMIZATION_LEVEL3, D3DCompile};
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::core::{PCSTR, Result, s};

use super::ambient::GpuParams;

const SRC: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/ambient.hlsl"));

pub struct AmbientPass {
    vs: ID3D11VertexShader,
    ps: ID3D11PixelShader,
    cb: ID3D11Buffer,
    rs: ID3D11RasterizerState,
}

fn compile(entry: PCSTR, target: PCSTR) -> Result<ID3DBlob> {
    unsafe {
        let mut code: Option<ID3DBlob> = None;
        let mut err: Option<ID3DBlob> = None;
        let res = D3DCompile(
            SRC.as_ptr() as *const c_void,
            SRC.len(),
            PCSTR::null(),
            None,
            None,
            entry,
            target,
            D3DCOMPILE_OPTIMIZATION_LEVEL3,
            0,
            &mut code,
            Some(&mut err),
        );
        if let Err(e) = res {
            if let Some(err) = err {
                let msg = std::slice::from_raw_parts(
                    err.GetBufferPointer() as *const u8,
                    err.GetBufferSize(),
                );
                eprintln!("ambient.hlsl: {}", String::from_utf8_lossy(msg));
            }
            return Err(e);
        }
        Ok(code.expect("shader blob"))
    }
}

fn bytes(blob: &ID3DBlob) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(blob.GetBufferPointer() as *const u8, blob.GetBufferSize())
    }
}

impl AmbientPass {
    pub fn new(device: &ID3D11Device) -> Result<Self> {
        unsafe {
            let vs_blob = compile(s!("vs_main"), s!("vs_5_0"))?;
            let ps_blob = compile(s!("ps_main"), s!("ps_5_0"))?;

            let mut vs = None;
            device.CreateVertexShader(bytes(&vs_blob), None, Some(&mut vs))?;
            let mut ps = None;
            device.CreatePixelShader(bytes(&ps_blob), None, Some(&mut ps))?;

            let desc = D3D11_BUFFER_DESC {
                ByteWidth: size_of::<GpuParams>() as u32,
                Usage: D3D11_USAGE_DYNAMIC,
                BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
                CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
                ..Default::default()
            };
            let mut cb = None;
            device.CreateBuffer(&desc, None, Some(&mut cb))?;

            let rs_desc = D3D11_RASTERIZER_DESC {
                FillMode: D3D11_FILL_SOLID,
                CullMode: D3D11_CULL_NONE,
                DepthClipEnable: true.into(),
                ..Default::default()
            };
            let mut rs = None;
            device.CreateRasterizerState(&rs_desc, Some(&mut rs))?;

            Ok(Self {
                vs: vs.unwrap(),
                ps: ps.unwrap(),
                cb: cb.unwrap(),
                rs: rs.unwrap(),
            })
        }
    }

    pub fn draw(
        &self,
        ctx: &ID3D11DeviceContext,
        rtv: &ID3D11RenderTargetView,
        w: u32,
        h: u32,
        params: &GpuParams,
    ) {
        unsafe {
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            if ctx
                .Map(&self.cb, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped))
                .is_ok()
            {
                std::ptr::copy_nonoverlapping(
                    params as *const GpuParams as *const u8,
                    mapped.pData as *mut u8,
                    size_of::<GpuParams>(),
                );
                ctx.Unmap(&self.cb, 0);
            }

            ctx.OMSetRenderTargets(Some(&[Some(rtv.clone())]), None);
            ctx.OMSetBlendState(None, None, 0xFFFF_FFFF);
            ctx.RSSetState(Some(&self.rs));
            ctx.RSSetViewports(Some(&[D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: w as f32,
                Height: h as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            }]));
            ctx.IASetInputLayout(None);
            ctx.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            ctx.VSSetShader(Some(&self.vs), None);
            ctx.PSSetShader(Some(&self.ps), None);
            ctx.PSSetConstantBuffers(0, Some(&[Some(self.cb.clone())]));
            ctx.Draw(3, 0);
        }
    }
}
