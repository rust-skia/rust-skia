//! SkStream and relatives.
//! This implementation covers the minimal subset to interface with Rust or Rust streams.
//! The policy is to avoid exporting Skia streams and try to cover every use case by
//! using Rust native Streams.

use crate::core::Data;
use crate::prelude::*;
use skia_bindings::{
    C_SkDynamicMemoryWStream_Construct, C_SkDynamicMemoryWStream_detachAsData,
    C_SkDynamicMemoryWStream_detachAsStream, C_SkMemoryStream_MakeDirect, C_SkStream_delete,
    C_SkWStream_destruct, C_SkWStream_write, SkDynamicMemoryWStream, SkMemoryStream, SkStream,
    SkStreamAsset,
};
use std::marker::PhantomData;
use std::ptr;

/// Trait representing an Skia allocated Stream type with a base class of SkStream.
pub struct Stream<N: NativeStreamBase>(*mut N);

pub trait NativeStreamBase {
    fn as_stream_mut(&mut self) -> &mut SkStream;
}

impl<T: NativeStreamBase> Drop for Stream<T> {
    fn drop(&mut self) {
        unsafe {
            C_SkStream_delete(self.0 as _);
        }
    }
}

impl<N: NativeStreamBase> Stream<N> {
    pub fn from_ptr(ptr: *mut N) -> Stream<N> {
        assert_ne!(ptr, ptr::null_mut());
        Stream(ptr)
    }
}

pub type StreamAsset = Stream<SkStreamAsset>;

impl NativeStreamBase for SkStreamAsset {
    fn as_stream_mut(&mut self) -> &mut SkStream {
        &mut self._base._base._base
    }
}

impl NativeAccess<SkStreamAsset> for Stream<SkStreamAsset> {
    fn native(&self) -> &SkStreamAsset {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut SkStreamAsset {
        unsafe { &mut *self.0 }
    }
}

#[repr(C)]
pub struct MemoryStream<'a> {
    native: *mut SkMemoryStream,
    pd: PhantomData<&'a ()>,
}

impl NativeStreamBase for SkMemoryStream {
    fn as_stream_mut(&mut self) -> &mut SkStream {
        &mut self._base._base._base._base._base
    }
}

impl<'a> NativeAccess<SkMemoryStream> for MemoryStream<'a> {
    fn native(&self) -> &SkMemoryStream {
        unsafe { &*self.native }
    }
    fn native_mut(&mut self) -> &mut SkMemoryStream {
        unsafe { &mut *self.native }
    }
}

impl<'a> MemoryStream<'a> {
    // Create a stream asset that refers the bytes provided.
    pub fn from_bytes<'bytes>(bytes: &'bytes [u8]) -> MemoryStream<'bytes> {
        let ptr = unsafe { C_SkMemoryStream_MakeDirect(bytes.as_ptr() as _, bytes.len()) };

        MemoryStream {
            native: ptr,
            pd: PhantomData,
        }
    }
}

pub type DynamicMemoryWStream = Handle<SkDynamicMemoryWStream>;

impl NativeDrop for SkDynamicMemoryWStream {
    fn drop(&mut self) {
        unsafe {
            C_SkWStream_destruct(&mut self._base);
        }
    }
}

impl Handle<SkDynamicMemoryWStream> {
    pub fn new() -> Self {
        Self::construct_c(C_SkDynamicMemoryWStream_Construct)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut stream = Self::new();
        stream.write(bytes);
        stream
    }

    pub fn write(&mut self, bytes: &[u8]) -> bool {
        unsafe {
            C_SkWStream_write(
                &mut self.native_mut()._base,
                bytes.as_ptr() as _,
                bytes.len(),
            )
        }
    }

    pub fn detach_as_data(&mut self) -> Data {
        Data::from_ptr(unsafe { C_SkDynamicMemoryWStream_detachAsData(self.native_mut()) }).unwrap()
    }

    pub fn detach_as_stream(&mut self) -> StreamAsset {
        StreamAsset::from_ptr(unsafe { C_SkDynamicMemoryWStream_detachAsStream(self.native_mut()) })
    }
}

#[test]
fn detaching_empty_dynamic_memory_w_stream_leads_to_non_null_data() {
    let mut stream = DynamicMemoryWStream::new();
    let data = stream.detach_as_data();
    assert_eq!(0, data.size())
}

#[test]
fn memory_stream_from_bytes() {
    let stream = MemoryStream::from_bytes(&[1, 2, 3]);
    drop(stream);
}
