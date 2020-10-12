use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::GrGLExtensions;
use std::ffi::CString;

pub type Extensions = Handle<GrGLExtensions>;
unsafe impl Send for Extensions {}
unsafe impl Sync for Extensions {}

impl NativeDrop for GrGLExtensions {
    fn drop(&mut self) {
        unsafe { sb::C_GrGLExtensions_destruct(self) }
    }
}

impl NativeClone for GrGLExtensions {
    fn clone(&self) -> Self {
        unsafe { sb::GrGLExtensions::new(self) }
    }
}

impl Handle<GrGLExtensions> {
    // TODO: support new() / init?

    pub fn is_initialized(&self) -> bool {
        self.native().fInitialized
    }

    pub fn has(&self, extension: impl AsRef<str>) -> bool {
        let extension = CString::new(extension.as_ref()).unwrap();
        unsafe { self.native().has(extension.as_ptr()) }
    }

    pub fn remove(&mut self, extension: impl AsRef<str>) -> bool {
        let extension = CString::new(extension.as_ref()).unwrap();
        unsafe { self.native_mut().remove(extension.as_ptr()) }
    }

    pub fn add(&mut self, extension: impl AsRef<str>) {
        let extension = CString::new(extension.as_ref()).unwrap();
        unsafe { self.native_mut().add(extension.as_ptr()) }
    }

    pub fn reset(&mut self) {
        unsafe { sb::C_GrGLExtensions_reset(self.native_mut()) }
    }

    // TODO: dumpJSON()?
}
