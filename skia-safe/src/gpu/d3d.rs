use crate::prelude::*;
use skia_bindings as sb;

mod backend_context;
pub use backend_context::*;

mod types;
pub use types::*;

// re-export D3D types we use

pub use sb::AsIUnknown;
pub use sb::GrD3DResourceStateEnum as ResourceStateEnum;
pub use sb::ID3D12CommandQueue;
pub use sb::ID3D12Device;
pub use sb::ID3D12Resource;
pub use sb::IDXGIAdapter1;
pub use sb::D3D12_RESOURCE_STATES;
pub use sb::DXGI_FORMAT;

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct cp<T: AsIUnknown>(*mut T);

impl<T: AsIUnknown> NativeTransmutable<skia_bindings::gr_cp<T>> for cp<T> {}
#[test]
fn test_cp_layout() {
    cp::<ID3D12Device>::test_layout();
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
