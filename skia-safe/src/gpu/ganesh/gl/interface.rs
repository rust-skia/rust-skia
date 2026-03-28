use crate::{gpu::gl::Extensions, prelude::*};
use skia_bindings::{self as sb, GrGLInterface, SkRefCntBase};
use std::{ffi::c_void, fmt, os::raw};

pub type Interface = RCHandle<GrGLInterface>;
require_type_equality!(sb::GrGLInterface_INHERITED, sb::SkRefCnt);

impl NativeRefCountedBase for GrGLInterface {
    type Base = SkRefCntBase;
}

impl fmt::Debug for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interface")
            .field("extensions", &self.extensions())
            .finish()
    }
}

impl Interface {
    pub fn new_native() -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_GrGLInterface_MakeNativeInterface() as _ })
    }

    /// Create a GL interface backed by a WebGL2 rendering context via `web-sys`.
    ///
    /// This is the recommended entry-point for GPU-accelerated Skia on
    /// `wasm32-unknown-unknown` (i.e. without Emscripten).  Pass a
    /// `WebGl2RenderingContext` obtained from the browser; it is stored in a
    /// thread-local and used for every subsequent GL call dispatched through
    /// this interface.
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    /// # {
    /// use web_sys::WebGl2RenderingContext;
    /// use skia_safe::gpu::gl::Interface;
    ///
    /// let ctx: WebGl2RenderingContext = /* obtain from canvas */ unimplemented!();
    /// let interface = Interface::new_web_sys(ctx).expect("WebGL2 interface creation failed");
    /// # }
    /// ```
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub fn new_web_sys(ctx: web_sys::WebGl2RenderingContext) -> Option<Self> {
        super::web_sys_interface::make(ctx)
    }

    /// Like `new_web_sys` but also returns an integer handle for use with
    /// `make_web_sys_current` when multiple HTML canvases are in use.
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub fn new_web_sys_identified(ctx: web_sys::WebGl2RenderingContext) -> Option<(u32, Self)> {
        super::web_sys_interface::make_identified(ctx)
    }

    /// Make context `id` (returned by `new_web_sys_identified`) the active
    /// context on this thread.  All GL calls dispatched through any
    /// `Interface` will use it.
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub fn make_web_sys_current(id: u32) {
        super::web_sys_interface::make_current(id);
    }

    /// Free the context registered under `id`.  The associated `Interface`
    /// must not be used again until a different context is made current.
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub fn drop_web_sys_context(id: u32) {
        super::web_sys_interface::drop_context(id);
    }

    pub fn new_load_with<F>(load_fn: F) -> Option<Self>
    where
        F: FnMut(&str) -> *const c_void,
    {
        Self::from_ptr(unsafe {
            sb::C_GrGLInterface_MakeAssembledInterface(
                &load_fn as *const _ as *mut c_void,
                Some(gl_get_proc_fn_wrapper::<F>),
            ) as _
        })
    }

    pub fn new_load_with_cstr<F>(load_fn: F) -> Option<Self>
    where
        F: FnMut(&std::ffi::CStr) -> *const c_void,
    {
        Self::from_ptr(unsafe {
            sb::C_GrGLInterface_MakeAssembledInterface(
                &load_fn as *const _ as *mut c_void,
                Some(gl_get_proc_fn_wrapper_cstr::<F>),
            ) as _
        })
    }

    pub fn validate(&self) -> bool {
        unsafe { self.native().validate() }
    }

    pub fn extensions(&self) -> &Extensions {
        Extensions::from_native_ref(unsafe {
            &*sb::C_GrGLInterface_extensions(self.native_mut_force())
        })
    }

    pub fn extensions_mut(&mut self) -> &mut Extensions {
        Extensions::from_native_ref_mut(unsafe {
            &mut *sb::C_GrGLInterface_extensions(self.native_mut())
        })
    }

    pub fn has_extension(&self, extension: impl AsRef<str>) -> bool {
        self.extensions().has(extension)
    }
}

unsafe extern "C" fn gl_get_proc_fn_wrapper<F>(
    ctx: *mut c_void,
    name: *const raw::c_char,
) -> *const c_void
where
    F: FnMut(&str) -> *const c_void,
{
    (*(ctx as *mut F))(std::ffi::CStr::from_ptr(name).to_str().unwrap())
}

unsafe extern "C" fn gl_get_proc_fn_wrapper_cstr<F>(
    ctx: *mut c_void,
    name: *const raw::c_char,
) -> *const c_void
where
    F: FnMut(&std::ffi::CStr) -> *const c_void,
{
    (*(ctx as *mut F))(std::ffi::CStr::from_ptr(name))
}
