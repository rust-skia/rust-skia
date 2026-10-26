#![allow(unused)]
use crate::prelude::safer;
use skia_bindings as sb;
use std::ffi::c_void;
use std::marker::PhantomData;

/// A sink / receiver for array data that is copied from C++ to a Rust [`Vec`].
#[derive(Debug)]
pub struct VecSink<'a, T> {
    sink: sb::VecSink<T>,
    pd: PhantomData<&'a mut Vec<T>>,
}

impl<'a, T: 'static> VecSink<'a, T> {
    /// Create a new sink that calls back into the closure given.
    pub fn new<F: FnMut(&[T])>(v: &'a mut F) -> Self {
        VecSink {
            sink: sb::VecSink {
                data: (v as *mut F).cast(),
                set_fn: Some(Self::set_fn::<F>),
                _phantom_0: PhantomData,
            },
            pd: PhantomData,
        }
    }

    pub fn new_mut<F: FnMut(&mut [T])>(v: &'a mut F) -> Self {
        VecSink {
            sink: sb::VecSink {
                data: (v as *mut F).cast(),
                set_fn: Some(Self::set_fn_mut::<F>),
                _phantom_0: PhantomData,
            },
            pd: PhantomData,
        }
    }

    pub fn native_mut(&mut self) -> &mut sb::VecSink<T> {
        &mut self.sink
    }

    /// `data` is the `&mut F` erased in `new`, the only place that pairs it with this function.
    unsafe extern "C" fn set_fn<F: FnMut(&[T])>(ptr: *mut T, len: usize, data: *mut c_void) {
        unsafe {
            let rust_fn = &mut *data.cast::<F>();
            (rust_fn)(safer::from_raw_parts(ptr, len));
        }
    }

    /// `data` is the `&mut F` erased in `new_mut`, the only place that pairs it with this function.
    unsafe extern "C" fn set_fn_mut<F: FnMut(&mut [T])>(
        ptr: *mut T,
        len: usize,
        data: *mut c_void,
    ) {
        unsafe {
            let rust_fn = &mut *data.cast::<F>();
            (rust_fn)(safer::from_raw_parts_mut(ptr, len));
        }
    }
}

#[derive(Debug)]
pub struct Sink<'a, T> {
    sink: sb::Sink<T>,
    pd: PhantomData<&'a mut T>,
}

impl<'a, T: 'static> Sink<'a, T> {
    /// Create a new sink that calls back into the closure given.
    pub fn new<F: FnMut(&T)>(v: &'a mut F) -> Self {
        Sink {
            sink: sb::Sink {
                data: (v as *mut F).cast(),
                set_fn: Some(Self::set_fn::<F>),
                _phantom_0: PhantomData,
            },
            pd: PhantomData,
        }
    }

    pub fn native_mut(&mut self) -> &mut sb::Sink<T> {
        &mut self.sink
    }

    /// `data` is the `&mut F` erased in `new`, the only place that pairs it with this function.
    unsafe extern "C" fn set_fn<F: FnMut(&T)>(ptr: *const T, data: *mut c_void) {
        unsafe {
            let rust_fn = &mut *data.cast::<F>();
            (rust_fn)(&*ptr);
        }
    }
}
