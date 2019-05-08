//! SkStream and relatives.
//! This implementation covers the minimal subset to interface with Rust or Rust streams.
//! The policy is to avoid exporting Skia streams and try to cover every use case by
//! using Rust native Streams.

use crate::core::Data;
use crate::prelude::*;
use skia_bindings::{
    C_SkDynamicMemoryWStream_Construct, C_SkDynamicMemoryWStream_detachAsData,
    C_SkDynamicMemoryWStream_detachAsStream, C_SkStream_delete, C_SkWStream_destruct,
    C_SkWStream_write, SkDynamicMemoryWStream, SkStreamAsset,
};

#[repr(transparent)]
pub struct StreamAsset(*mut SkStreamAsset);

impl NativeAccess<SkStreamAsset> for StreamAsset {
    fn native(&self) -> &SkStreamAsset {
        unsafe { &*self.0 }
    }

    fn native_mut(&mut self) -> &mut SkStreamAsset {
        unsafe { &mut *self.0 }
    }
}

impl Drop for StreamAsset {
    fn drop(&mut self) {
        unsafe {
            C_SkStream_delete(&mut self.native_mut()._base._base._base);
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
        StreamAsset(unsafe {
            C_SkDynamicMemoryWStream_detachAsStream(self.native_mut())
        })
    }
}

#[test]
fn detaching_empty_dynamic_memory_w_stream_leads_to_non_null_data() {
    let mut stream = DynamicMemoryWStream::new();
    let data = stream.detach_as_data();
    assert_eq!(0, data.size())
}
