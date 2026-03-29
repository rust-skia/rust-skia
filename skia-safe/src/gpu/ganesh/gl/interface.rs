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
    /// Creates a GL interface from the native platform GL context.
    ///
    /// On `wasm32-unknown-unknown` targets, use [`Interface::new_web_sys()`] instead.
    pub fn new_native() -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_GrGLInterface_MakeNativeInterface() as _ })
    }

    /// Creates a GL interface using a `web_sys` WebGL2 context.
    ///
    /// Before calling this function, the WebGL2 context must be registered and set as the active
    /// context via [`skia_safe::gpu::gl::register_gl_context`] and
    /// [`skia_safe::gpu::gl::set_gl_context`]:
    ///
    /// ```no_run
    /// # #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    /// # {
    /// // gl_ctx: web_sys::WebGl2RenderingContext
    /// let id = skia_safe::gpu::gl::register_gl_context(gl_ctx);
    /// skia_safe::gpu::gl::set_gl_context(id);
    /// let interface = skia_safe::gpu::gl::Interface::new_web_sys();
    /// # }
    /// ```
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub fn new_web_sys() -> Option<Self> {
        use skia_wasm_shims;

        Self::from_ptr(unsafe {
            skia_bindings::C_GrGLInterface_MakeAssembledInterface(
                std::ptr::null_mut(),
                Some(skia_wasm_shims::web_sys_get_proc),
            ) as *mut _
        })
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
