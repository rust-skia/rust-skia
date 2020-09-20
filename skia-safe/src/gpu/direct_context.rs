#[cfg(feature = "d3d")]
use super::d3d;
#[cfg(feature = "gl")]
use super::gl;
#[cfg(feature = "vulkan")]
use super::vk;
use super::ContextOptions;
use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::{GrContext, GrDirectContext, SkRefCntBase};
use std::ops::{Deref, DerefMut};

pub type DirectContext = RCHandle<GrDirectContext>;

impl NativeRefCountedBase for GrDirectContext {
    type Base = SkRefCntBase;
}

impl Deref for RCHandle<GrDirectContext> {
    type Target = RCHandle<GrContext>;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref(self) }
    }
}

impl DerefMut for RCHandle<GrDirectContext> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_ref_mut(self) }
    }
}

impl RCHandle<GrDirectContext> {
    #[cfg(feature = "gl")]
    pub fn new_gl<'a>(
        interface: impl Into<Option<gl::Interface>>,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        DirectContext::from_ptr(unsafe {
            sb::C_GrDirectContext_MakeGL(
                interface.into().into_ptr_or_null(),
                options.into().native_ptr_or_null(),
            )
        })
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan<'a>(
        backend_context: &vk::BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        unsafe {
            let end_resolving = backend_context.begin_resolving();
            let context = DirectContext::from_ptr(sb::C_GrDirectContext_MakeVulkan(
                backend_context.native.as_ptr() as _,
                options.into().native_ptr_or_null(),
            ));
            drop(end_resolving);
            context
        }
    }

    /// # Safety
    /// This function is unsafe because `device` and `queue` are untyped handles which need to exceed the
    /// lifetime of the context returned.
    #[cfg(feature = "metal")]
    pub unsafe fn new_metal<'a>(
        device: *mut std::ffi::c_void,
        queue: *mut std::ffi::c_void,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        DirectContext::from_ptr(sb::C_GrContext_MakeMetal(
            device,
            queue,
            options.into().native_ptr_or_null(),
        ))
    }

    #[cfg(feature = "d3d")]
    pub unsafe fn new_d3d<'a>(
        backend_context: &d3d::BackendContext,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<DirectContext> {
        DirectContext::from_ptr(sb::C_GrDirectContext_MakeDirect3D(
            backend_context.native(),
            options.into().native_ptr_or_null(),
        ))
    }
}
