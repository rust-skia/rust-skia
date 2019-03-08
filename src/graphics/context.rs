#[cfg(feature = "vulkan")]
use super::vulkan;
use crate::prelude::*;
use skia_bindings::{
    GrContext,
    SkRefCntBase
};
#[cfg(feature = "vulkan")]
use skia_bindings::C_GrContext_MakeVulkan;

pub type Context = RCHandle<GrContext>;

impl NativeRefCountedBase for GrContext {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl Context {
    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(backend_context: &vulkan::BackendContext) -> Option<Context> {
       Context::from_ptr(unsafe { C_GrContext_MakeVulkan(backend_context.native) })
    }
}
