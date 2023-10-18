use std::{ffi, path::Path, ptr};

use skia_safe::{
    gpu::{self, d3d, Budgeted, Protected},
    Canvas, ImageInfo,
};
use winapi::{
    shared::{
        dxgi,
        guiddef::GUID,
        winerror::{HRESULT, S_OK},
    },
    um::{d3d12, d3dcommon},
    Interface,
};
use wio::com::ComPtr;

use crate::{artifact, drivers::DrawingDriver, Driver};

pub struct D3D {
    context: gpu::DirectContext,
}

impl DrawingDriver for D3D {
    const DRIVER: Driver = Driver::D3d;

    fn new() -> Self {
        let factory: ComPtr<dxgi::IDXGIFactory1> =
            resolve_interface(|iid, ptr| unsafe { dxgi::CreateDXGIFactory1(iid, ptr) })
                .expect_ok("Creating DXGI factory");

        let adapter = resolve_specific(|ptr| unsafe { factory.EnumAdapters1(0, ptr) })
            .expect_ok("Creating DXGI Adapter");

        let device: ComPtr<d3d12::ID3D12Device> = resolve_interface(|iid, ptr| unsafe {
            d3d12::D3D12CreateDevice(
                adapter.as_raw() as _,
                d3dcommon::D3D_FEATURE_LEVEL_11_0,
                iid,
                ptr,
            )
        })
        .expect_ok("Creating D3D device");

        let queue: ComPtr<d3d12::ID3D12CommandQueue> = {
            let desc = d3d12::D3D12_COMMAND_QUEUE_DESC {
                Type: d3d12::D3D12_COMMAND_LIST_TYPE_DIRECT,
                Priority: d3d12::D3D12_COMMAND_QUEUE_PRIORITY_NORMAL as _,
                Flags: d3d12::D3D12_COMMAND_QUEUE_FLAG_NONE,
                NodeMask: 0,
            };

            resolve_interface(|iid, ptr| unsafe { device.CreateCommandQueue(&desc, iid, ptr) })
                .expect_ok("Creating command queue")
        };

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
        func: impl Fn(&mut Canvas),
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
        )
        .unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, func);
    }

    fn draw_image_256(&mut self, path: &Path, name: &str, func: impl Fn(&mut Canvas)) {
        self.draw_image((256, 256), path, name, func)
    }
}

fn resolve_interface<T: Interface>(
    f: impl FnOnce(&GUID, &mut *mut ffi::c_void) -> HRESULT,
) -> Result<ComPtr<T>, HRESULT> {
    let mut ptr: *mut ffi::c_void = ptr::null_mut();
    let r = f(&T::uuidof(), &mut ptr);
    if r == S_OK {
        Ok(unsafe { ComPtr::from_raw(ptr as *mut T) })
    } else {
        Err(r)
    }
}

fn resolve_specific<T: Interface>(
    f: impl FnOnce(&mut *mut T) -> HRESULT,
) -> Result<ComPtr<T>, HRESULT> {
    let mut ptr: *mut T = ptr::null_mut();
    let r = f(&mut ptr);
    if r == S_OK {
        Ok(unsafe { ComPtr::from_raw(ptr) })
    } else {
        Err(r)
    }
}

trait ExpectOk<T> {
    fn expect_ok(self, msg: &str) -> T;
}

impl<T> ExpectOk<T> for Result<T, HRESULT> {
    fn expect_ok(self, msg: &str) -> T {
        match self {
            Ok(r) => r,
            Err(hr) => panic!("{msg} failed. {hr:x}"),
        }
    }
}
