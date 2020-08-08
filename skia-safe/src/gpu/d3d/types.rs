use super::{ID3D12Resource, D3D12_RESOURCE_STATES, DXGI_FORMAT};
use crate::gpu;
use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::GrD3DTextureResourceInfo;
use std::ptr;

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cp<T>(*mut T);

pub fn safe_com_add_ref<T>(obj: *mut T) -> *mut T {
    if !obj.is_null() {
        unsafe { sb::C_IUnknown_AddRef(obj as _) }
    }
    obj
}

pub fn safe_com_release<T>(obj: *mut T) {
    if !obj.is_null() {
        unsafe { sb::C_IUnknown_Release(obj as _) }
    }
}

impl<T> NativeTransmutable<skia_bindings::gr_cp<T>> for cp<T> {}
#[test]
fn test_cp_layout() {
    cp::<super::ID3D12Device>::test_layout();
}

impl<T> Drop for cp<T> {
    fn drop(&mut self) {
        safe_com_release(self.0);
    }
}

impl<T> Clone for cp<T> {
    fn clone(&self) -> Self {
        Self::from_ptr(safe_com_add_ref(self.0))
    }
}

impl<T> Default for cp<T> {
    fn default() -> Self {
        Self::from_ptr(ptr::null_mut())
    }
}

impl<T> PartialEq for cp<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for cp<T> {}

impl<T> cp<T> {
    pub fn from_ptr(ptr: *mut T) -> Self {
        cp(ptr)
    }

    pub fn get(&self) -> *mut T {
        self.0
    }

    pub fn reset(&mut self, object: *mut T) {
        let old = self.0;
        self.0 = object;
        safe_com_release(old);
    }

    pub fn retain(&mut self, object: *mut T) {
        if self.0 != object {
            self.reset(safe_com_add_ref(object))
        }
    }

    #[must_use]
    pub fn release(&mut self) -> *mut T {
        let obj = self.0;
        self.0 = ptr::null_mut();
        obj
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
