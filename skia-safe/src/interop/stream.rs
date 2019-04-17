/// SkStream and relatives.
/// This implementation covers the minimal subset to interface with Rust or Rust streams
/// The policy is to avoid exporting Skia streams and try to cover every use case by
/// using Rust native Streams.

use crate::prelude::*;
use crate::core::Data;
use skia_bindings::{SkDynamicMemoryWStream, C_SkDynamicMemoryWStream_detachAsData, C_SkDynamicMemoryWStream_destruct, C_SkDynamicMemoryWStream_Construct};
use std::mem;

pub type DynamicMemoryWStream = Handle<SkDynamicMemoryWStream>;

impl NativeDrop for SkDynamicMemoryWStream {
    fn drop(&mut self) {
        unsafe {
            C_SkDynamicMemoryWStream_destruct(self)
        }
    }
}

impl Handle<SkDynamicMemoryWStream> {
    pub fn new() -> Self {
        Handle::from_native(unsafe {
            // does not link under linux:
            // SkDynamicMemoryWStream::new()
            let mut stream = mem::uninitialized();
            C_SkDynamicMemoryWStream_Construct(&mut stream);
            stream
        })
    }

    pub fn detach_as_data(&mut self) -> Data {
        Data::from_ptr(unsafe {
            C_SkDynamicMemoryWStream_detachAsData(self.native_mut())
        }).unwrap()
    }
}

#[test]
fn detaching_empty_dynamic_memory_w_stream_leads_to_non_null_data() {
    let mut stream = DynamicMemoryWStream::new();
    let data = stream.detach_as_data();
    assert_eq!(0, data.size())
}
