///! FFI interopability for skia-safe's wrapper types.
///!
///! This module is only meant to be used by external code. Internal code should continue to use the traits in
///! the `prelude` module.
use crate::prelude::*;

/// This trait supports the conversion of a wrapper type into it's native C/C++ and back.
///
/// # Safety
///
/// The native type `N` _should_ be treated as opaque, because its definition may change
/// without adhering to semantic versioning and largely depends on what the tool bindgen
/// is able to generate.
///
/// Converting from a Rust wrapper type to a native type loses the automatic ability to free associated memory.
pub unsafe trait PointerWrapper<N>
where
    Self: Sized,
{
    /// Wraps a native pointer into a wrapper type.
    /// Returns `None` if the pointer is `null`.
    fn wrap(ptr: *mut N) -> Option<Self>;
    /// Unwraps the wrapper type into the native pointer.
    fn unwrap(self) -> *mut N;
    /// Access the wrapped pointer.
    fn inner(&self) -> &N;
    /// Access the wrapped pointer.
    fn inner_mut(&mut self) -> &mut N;
}

/// A trait that supports the conversion from a native value into its Rust wrapper type and back.
///
/// This is implemented for all wrapper types that manage memory in Rust without an pointer indirection.
///
/// # Safety
///
/// The native type `N` _should_ be treated as opaque, because its definition may change
/// without adhering to semantic versioning and largely depends on what the tool bindgen
/// is able to generate.
///
/// Converting from a Rust wrapper type to a native type may lose the automatic ability to free associated memory.
pub unsafe trait ValueWrapper<N> {
    fn wrap(native: N) -> Self;
    fn unwrap(self) -> N;
    fn inner(&self) -> &N;
    fn inner_mut(&mut self) -> &mut N;
}

pub unsafe trait RefWrapper<N> {
    fn wrap_ref(native: &N) -> &Self;
    fn wrap_mut(native: &mut N) -> &mut Self;
    fn inner(&self) -> &N;
    fn inner_mut(&mut self) -> &mut N;
}

//
// Handle<N>
//

unsafe impl<N> ValueWrapper<N> for Handle<N>
where
    N: NativeDrop,
{
    fn wrap(native: N) -> Self
    where
        N: NativeDrop,
    {
        Self::from_native(native)
    }

    fn unwrap(self) -> N {
        self.into_native()
    }

    fn inner(&self) -> &N {
        self.native()
    }

    fn inner_mut(&mut self) -> &mut N {
        self.native_mut()
    }
}

unsafe impl<N> RefWrapper<N> for Handle<N>
where
    N: NativeDrop,
{
    fn wrap_ref(native: &N) -> &Self {
        Self::from_native_ref(native)
    }

    fn wrap_mut(native: &mut N) -> &mut Self {
        Self::from_native_ref_mut(native)
    }

    fn inner(&self) -> &N {
        self.native()
    }

    fn inner_mut(&mut self) -> &mut N {
        self.native_mut()
    }
}

//
// RefHandle<N>
//

unsafe impl<N> PointerWrapper<N> for RefHandle<N>
where
    N: NativeDrop,
{
    fn wrap(ptr: *mut N) -> Option<Self> {
        Self::from_ptr(ptr)
    }

    fn unwrap(self) -> *mut N {
        self.into_ptr()
    }

    fn inner(&self) -> &N {
        self.native()
    }

    fn inner_mut(&mut self) -> &mut N {
        self.native_mut()
    }
}

//
// RCHandle<N>
//

unsafe impl<N> PointerWrapper<N> for RCHandle<N>
where
    N: NativeRefCounted,
{
    fn wrap(ptr: *mut N) -> Option<Self> {
        Self::from_ptr(ptr)
    }

    fn unwrap(self) -> *mut N {
        self.into_ptr()
    }

    fn inner(&self) -> &N {
        self.native()
    }

    fn inner_mut(&mut self) -> &mut N {
        self.native_mut()
    }
}
