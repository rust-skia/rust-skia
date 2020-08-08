use crate::artifact;
use crate::drivers::DrawingDriver;
use skia_safe::{
    gpu,
    gpu::{d3d, d3d::cp, Protected},
    Budgeted, Canvas, ImageInfo, Surface,
};
use std::path::Path;
use std::ptr;
use winapi::{
    shared::{
        dxgi::{CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory1},
        winerror::S_OK,
    },
    um::{
        d3d12::{
            D3D12CreateDevice, ID3D12CommandQueue, ID3D12Device, D3D12_COMMAND_LIST_TYPE_DIRECT,
            D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE,
            D3D12_COMMAND_QUEUE_PRIORITY_NORMAL,
        },
        d3dcommon::D3D_FEATURE_LEVEL_12_0,
    },
    Interface,
};
use wio::com::ComPtr;

pub struct D3D {
    context: gpu::Context,
}

impl DrawingDriver for D3D {
    const NAME: &'static str = "d3d";

    fn new() -> Self {
        let factory = {
            let mut factory: *mut IDXGIFactory1 = ptr::null_mut();
            let r = unsafe {
                CreateDXGIFactory1(&IDXGIFactory1::uuidof(), &mut factory as *mut _ as _)
            };
            if r != S_OK {
                panic!("failed to create DXGI factory");
            }
            unsafe { ComPtr::from_raw(factory) }
        };

        let adapter = {
            let mut adapter: *mut IDXGIAdapter1 = ptr::null_mut();
            let r = unsafe { factory.EnumAdapters1(0, &mut adapter as *mut _ as _) };
            if r != S_OK {
                panic!("failed to create DXGI adapter");
            }
            unsafe { ComPtr::from_raw(adapter) }
        };

        let device = {
            let mut device: *mut ID3D12Device = ptr::null_mut();
            let r = unsafe {
                D3D12CreateDevice(
                    ptr::null_mut(),
                    D3D_FEATURE_LEVEL_12_0,
                    &ID3D12Device::uuidof(),
                    &mut device as *mut _ as _,
                )
            };
            if r != S_OK {
                panic!("failed to create D3D device")
            }
            unsafe { ComPtr::from_raw(device) }
        };

        let queue = {
            let mut queue: *mut ID3D12CommandQueue = ptr::null_mut();
            let desc = D3D12_COMMAND_QUEUE_DESC {
                Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL as _,
                Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                NodeMask: 0,
            };
            let r = unsafe {
                device.CreateCommandQueue(
                    &desc,
                    &ID3D12CommandQueue::uuidof(),
                    &mut queue as *mut _ as _,
                )
            };
            if r != S_OK {
                panic!("failed to create D3D device")
            }
            unsafe { ComPtr::from_raw(queue) }
        };

        let backend_context = d3d::BackendContext {
            adapter: cp::from_ptr(adapter.into_raw() as _),
            device: cp::from_ptr(device.into_raw() as _),
            queue: cp::from_ptr(queue.into_raw() as _),
            protected_context: Protected::No,
        };

        let context = unsafe { gpu::Context::new_d3d(&backend_context) }.unwrap();
        Self { context }
    }

    fn draw_image(
        &mut self,
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&mut Canvas),
    ) {
        let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
        let mut surface = Surface::new_render_target(
            &mut self.context,
            Budgeted::Yes,
            &image_info,
            None,
            gpu::SurfaceOrigin::BottomLeft,
            None,
            false,
        )
        .unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, func);
    }
    fn draw_image_256(&mut self, path: &Path, name: &str, func: impl Fn(&mut Canvas)) {
        self.draw_image((256, 256), path, name, func)
    }
}
