use super::vulkan;
use crate::prelude::*;
use rust_skia::{
    GrContext,
    C_GrContext_MakeVulkan,
    SkRefCntBase
};

pub type Context = RCHandle<GrContext>;

impl NativeRefCountedBase for GrContext {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl Context {
    pub fn new_vulkan(backend_context: &vulkan::BackendContext) -> Option<Context> {
       Context::from_ptr(unsafe { C_GrContext_MakeVulkan(backend_context.native) })
    }
}
