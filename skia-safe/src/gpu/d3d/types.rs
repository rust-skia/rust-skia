use super::{ID3D12Resource, D3D12_RESOURCE_STATES, DXGI_FORMAT};
use crate::gpu;
use crate::prelude::*;
use skia_bindings::GrD3DTextureResourceInfo;
use winapi::um::unknwnbase::IUnknown;

pub use wio::com::ComPtr as cp;

pub fn safe_com_add_ref<T: winapi::Interface>(ptr: *mut T) -> *mut T {
    if !ptr.is_null() {
        unsafe { (*(ptr as *mut IUnknown)).AddRef() };
    }
    ptr
}

pub fn safe_com_release<T: winapi::Interface>(ptr: *mut T) {
    if !ptr.is_null() {
        unsafe { (*(ptr as *mut IUnknown)).Release() };
    }
}

impl<T> NativeTransmutable<skia_bindings::gr_cp<T>> for cp<T> {}
#[test]
fn test_cp_layout() {
    cp::<super::ID3D12Device>::test_layout();
}

// TODO: add remaining cp functions to ComPtr via traits (get, reset, retain).

#[repr(C)]
#[derive(Clone)]
pub struct TextureResourceInfo {
    pub resource: cp<ID3D12Resource>,
    pub resource_state: D3D12_RESOURCE_STATES,
    pub format: DXGI_FORMAT,
    pub level_count: u32,
    pub sample_quality_level: std::os::raw::c_uint,
    pub protected: gpu::Protected,
}

impl NativeTransmutable<GrD3DTextureResourceInfo> for TextureResourceInfo {}
#[test]
fn test_texture_resource_info_layout() {
    TextureResourceInfo::test_layout();
}
