use std::{fmt, marker::PhantomData, ptr};

use skia_bindings::{self as sb, SkRecorder};

use crate::prelude::NativeAccess;

pub type Type = skia_bindings::SkRecorder_Type;
variant_name!(Type::CPU);

pub struct Recorder<'a> {
    ptr: ptr::NonNull<SkRecorder>,
    _owned_by: PhantomData<&'a mut ()>,
    delete_it: bool,
}

impl Drop for Recorder<'_> {
    fn drop(&mut self) {
        if self.delete_it {
            unsafe { sb::C_SkRecorder_delete(self.native_mut()) }
        }
    }
}

impl fmt::Debug for Recorder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Recorder")
            .field("type", &self.ty())
            .finish()
    }
}

impl NativeAccess for Recorder<'_> {
    type Native = SkRecorder;

    fn native(&self) -> &SkRecorder {
        unsafe { self.ptr.as_ref() }
    }

    fn native_mut(&mut self) -> &mut SkRecorder {
        unsafe { self.ptr.as_mut() }
    }
}

impl Recorder<'_> {
    pub fn ty(&self) -> Type {
        let mut ty = Type::CPU;
        unsafe {
            sb::C_SkRecorder_type(self.native(), &mut ty);
        }
        ty
    }
}
