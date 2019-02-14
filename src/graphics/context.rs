use rust_skia::{GrContext, C_GrContext_MakeVulkan};
use super::vulkan;
use crate::prelude::*;

#[derive(Clone)]
pub struct Context {
    pub(crate) inner: RefCounted<Inner>
}

impl Native<*mut GrContext> for Context {
    fn native(&self) -> *mut GrContext {
        self.inner.0
    }
}

#[derive(Clone)]
pub (crate) struct Inner(*mut GrContext);

impl RefCount for Inner {
    fn refer(&self) {
        unsafe { (*self.0)._base._base.ref_() }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe { (*self.0)._base._base.unref(); }
    }
}

impl Context {
    pub fn new_vulkan(backend_context: &vulkan::BackendContext) -> Option<Context> {
       unsafe { C_GrContext_MakeVulkan(backend_context.native) }
           .to_option()
           .map(|native| Context{ inner: Inner(native).into() })
    }
}
