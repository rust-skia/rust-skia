use std::path::Path;

use skia_safe::{
    gpu::{self, d3d, Budgeted, Protected},
    Canvas, ImageInfo,
};

use crate::{artifact, drivers::DrawingDriver, Driver};

use windows::Win32::Graphics::{
    Direct3D::D3D_FEATURE_LEVEL_11_0,
    Direct3D12::{
        D3D12CreateDevice, ID3D12CommandQueue, ID3D12Device, D3D12_COMMAND_LIST_TYPE_DIRECT,
        D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE,
    },
    Dxgi::{
        CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory4, DXGI_ADAPTER_FLAG,
        DXGI_ADAPTER_FLAG_NONE, DXGI_ADAPTER_FLAG_SOFTWARE,
    },
};

pub struct D3D {
    context: gpu::DirectContext,
}

impl DrawingDriver for D3D {
    const DRIVER: Driver = Driver::D3d;

    fn new() -> Self {
        let factory =
            unsafe { CreateDXGIFactory1::<IDXGIFactory4>() }.expect("Creating DXGI factory");

        let adapter = get_hardware_adapter(&factory).expect("Creating DXGI Adapter");

        let mut device: Option<ID3D12Device> = None;
        unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device) }
            .expect("Creating D3D device");
        let device = device.expect("Creating D3D device");

        let queue = unsafe {
            device.CreateCommandQueue::<ID3D12CommandQueue>(&D3D12_COMMAND_QUEUE_DESC {
                Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                ..Default::default()
            })
        }
        .expect("Creating command queue");

        let backend_context = d3d::BackendContext {
            adapter,
            device,
            queue,
            memory_allocator: None,
            protected_context: Protected::No,
        };

        let context = unsafe { gpu::DirectContext::new_d3d(&backend_context, None) }.unwrap();
        Self { context }
    }

    fn draw_image(
        &mut self,
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&Canvas),
    ) {
        let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
        let mut surface = gpu::surfaces::render_target(
            &mut self.context,
            Budgeted::Yes,
            &image_info,
            None,
            gpu::SurfaceOrigin::BottomLeft,
            None,
            false,
            None,
        )
        .unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, func);
    }

    fn draw_image_256(&mut self, path: &Path, name: &str, func: impl Fn(&Canvas)) {
        self.draw_image((256, 256), path, name, func)
    }
}

fn get_hardware_adapter(factory: &IDXGIFactory4) -> windows::core::Result<IDXGIAdapter1> {
    for i in 0.. {
        let adapter = unsafe { factory.EnumAdapters1(i)? };

        let desc = unsafe { adapter.GetDesc1()? };

        if (DXGI_ADAPTER_FLAG(desc.Flags as i32) & DXGI_ADAPTER_FLAG_SOFTWARE)
            != DXGI_ADAPTER_FLAG_NONE
        {
            // Don't select the Basic Render Driver adapter. If you want a
            // software adapter, pass in "/warp" on the command line.
            continue;
        }

        // Check to see whether the adapter supports Direct3D 12, but don't
        // create the actual device yet.
        if unsafe {
            D3D12CreateDevice(
                &adapter,
                D3D_FEATURE_LEVEL_11_0,
                std::ptr::null_mut::<Option<ID3D12Device>>(),
            )
        }
        .is_ok()
        {
            return Ok(adapter);
        }
    }

    unreachable!()
}
