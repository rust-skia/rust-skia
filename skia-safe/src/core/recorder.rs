//! The [`Recorder`] trait for recording drawing commands, implemented by the borrowed
//! [`RecorderRef`] and by the [`OwnedRecorder`] that [`Context::make_recorder`] creates.

use std::{cell::UnsafeCell, fmt, marker::PhantomData};

use crate::{Context, prelude::*};
use skia_bindings::{self as sb, SkRecorder};

pub type Type = skia_bindings::SkRecorder_Type;
variant_name!(Type::CPU);

pub trait Recorder: sealed::AsRecorderRef + fmt::Debug {
    fn ty(&self) -> Type;
    // TODO: Wrap `cpuRecorder()`. Its type `skcpu::Recorder` is private since m159.
}

pub(crate) mod sealed {
    pub trait AsRecorderRef {
        fn as_recorder_ref(&mut self) -> &mut super::RecorderRef;
    }
}

#[repr(transparent)]
pub struct RecorderRef(UnsafeCell<SkRecorder>);

impl NativeAccess for RecorderRef {
    type Native = SkRecorder;

    fn native(&self) -> &SkRecorder {
        unsafe { &*self.0.get() }
    }
}

impl NativeAccessMut for RecorderRef {
    fn native_mut(&mut self) -> &mut SkRecorder {
        unsafe { &mut (*self.0.get()) }
    }
}

impl fmt::Debug for RecorderRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Recorder")
            .field("type", &self.ty())
            .finish()
    }
}

impl RecorderRef {
    pub(crate) fn from_ref_mut(native: &mut SkRecorder) -> &mut Self {
        unsafe { transmute_ref_mut(native) }
    }
}

impl Recorder for RecorderRef {
    fn ty(&self) -> Type {
        recorder_type(self.native())
    }
}

impl sealed::AsRecorderRef for RecorderRef {
    fn as_recorder_ref(&mut self) -> &mut RecorderRef {
        self
    }
}

/// A recorder that [`Context::make_recorder`] created, deleted when this value drops.
///
/// It borrows the [`Context`], because a GPU recorder records into the context's GPU backend.
pub struct OwnedRecorder<'a> {
    recorder: RefHandle<SkRecorder>,
    context: PhantomData<&'a Context>,
}

impl NativeDelete for SkRecorder {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_SkRecorder_delete(ptr) }
    }
}

impl fmt::Debug for OwnedRecorder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedRecorder")
            .field("type", &self.ty())
            .finish()
    }
}

impl<'a> OwnedRecorder<'a> {
    /// Takes ownership of a recorder that `context` created, or returns `None` when `ptr` is null.
    pub(crate) fn from_ptr(ptr: *mut SkRecorder, _context: &'a Context) -> Option<Self> {
        RefHandle::from_ptr(ptr).map(|recorder| Self {
            recorder,
            context: PhantomData,
        })
    }
}

impl Recorder for OwnedRecorder<'_> {
    fn ty(&self) -> Type {
        recorder_type(self.recorder.native())
    }
}

impl sealed::AsRecorderRef for OwnedRecorder<'_> {
    fn as_recorder_ref(&mut self) -> &mut RecorderRef {
        RecorderRef::from_ref_mut(unsafe { &mut *self.recorder.native_ptr() })
    }
}

fn recorder_type(recorder: &SkRecorder) -> Type {
    let mut ty = Type::CPU;
    unsafe {
        sb::C_SkRecorder_type(recorder, &mut ty);
    }
    ty
}

#[cfg(test)]
mod tests {
    use super::{Recorder, RecorderRef};

    #[allow(deref_nullptr)]
    fn _passing_the_different_kinds_of_recorder_compiles() {
        test(None);
        test(Some(borrowed_recorder()));

        fn test(_recorder: Option<&mut dyn Recorder>) {}

        fn borrowed_recorder() -> &'static mut RecorderRef {
            unsafe { &mut *std::ptr::null_mut() }
        }
    }
}
