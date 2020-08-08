use super::{ID3D12Resource, D3D12_RESOURCE_STATES, DXGI_FORMAT};
use crate::gpu;
use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::{AsIUnknown, GrD3DTextureResourceInfo};

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cp<T: AsIUnknown>(*mut T);

impl<T: AsIUnknown> NativeTransmutable<skia_bindings::gr_cp<T>> for cp<T> {}
#[test]
fn test_cp_layout() {
    cp::<super::ID3D12Device>::test_layout();
}

impl<T: AsIUnknown> Drop for cp<T> {
    fn drop(&mut self) {
        unsafe { sb::C_IUnknown_Release(self.0 as _) }
    }
}

impl<T: AsIUnknown> Clone for cp<T> {
    fn clone(&self) -> Self {
        Self::new(self.0)
    }
}

impl<T: AsIUnknown> cp<T> {
    /// Creates a new smart pointer for D3D types.
    ///
    /// Asserts if ptr is `null`, increases the reference count.
    pub fn new(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        unsafe { sb::C_IUnknown_AddRef(ptr as _) };
        cp(ptr)
    }
}

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
