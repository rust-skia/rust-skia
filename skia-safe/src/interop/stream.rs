//! SkStream and relatives.
//! This implementation covers the minimal subset to interface with Rust streams.
//!
//! Bindings that wrap functions that use Skia stream types, _must_ use Rust streams instead.

use crate::{prelude::*, Data};
use skia_bindings as sb;
use skia_bindings::{SkDynamicMemoryWStream, SkMemoryStream, SkStream, SkStreamAsset, SkWStream};
use std::{ffi, fmt, io, marker::PhantomData, ptr};

/// Trait representing an Skia allocated Stream type with a base class of SkStream.
#[repr(transparent)]
pub struct Stream<N: NativeStreamBase>(ptr::NonNull<N>);
unsafe impl<N: NativeStreamBase> Send for Stream<N> {}

pub trait NativeStreamBase {
    fn as_stream_mut(&mut self) -> &mut SkStream;
}

impl<T: NativeStreamBase> Drop for Stream<T> {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkStream_delete(self.0.as_ptr() as *mut _);
        }
    }
}

impl<N: NativeStreamBase> Stream<N> {
    pub fn from_ptr(ptr: *mut N) -> Option<Stream<N>> {
        ptr::NonNull::new(ptr).map(Stream)
    }
}

pub type StreamAsset = Stream<SkStreamAsset>;
impl NativeBase<SkStream> for SkStreamAsset {}

impl NativeStreamBase for SkStreamAsset {
    fn as_stream_mut(&mut self) -> &mut SkStream {
        self.base_mut()
    }
}

impl NativeAccess<SkStreamAsset> for StreamAsset {
    fn native(&self) -> &SkStreamAsset {
        unsafe { self.0.as_ref() }
    }
    fn native_mut(&mut self) -> &mut SkStreamAsset {
        unsafe { self.0.as_mut() }
    }
}

impl fmt::Debug for StreamAsset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamAsset").finish()
    }
}

#[repr(C)]
pub struct MemoryStream<'a> {
    native: ptr::NonNull<SkMemoryStream>,
    pd: PhantomData<&'a ()>,
}
unsafe impl Send for MemoryStream<'_> {}
impl NativeBase<SkStream> for SkMemoryStream {}

impl NativeStreamBase for SkMemoryStream {
    fn as_stream_mut(&mut self) -> &mut SkStream {
        self.base_mut()
    }
}

impl NativeAccess<SkMemoryStream> for MemoryStream<'_> {
    fn native(&self) -> &SkMemoryStream {
        unsafe { self.native.as_ref() }
    }
    fn native_mut(&mut self) -> &mut SkMemoryStream {
        unsafe { self.native.as_mut() }
    }
}

impl fmt::Debug for MemoryStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryStream")
            .field("offset", &self.native().fOffset)
            .finish()
    }
}

impl MemoryStream<'_> {
    // Create a stream asset that refers the bytes provided.
    pub fn from_bytes(bytes: &[u8]) -> MemoryStream {
        let ptr = unsafe { sb::C_SkMemoryStream_MakeDirect(bytes.as_ptr() as _, bytes.len()) };

        MemoryStream {
            native: ptr::NonNull::new(ptr).unwrap(),
            pd: PhantomData,
        }
    }
}

pub type DynamicMemoryWStream = Handle<SkDynamicMemoryWStream>;

impl NativeBase<SkWStream> for SkDynamicMemoryWStream {}

impl NativeDrop for SkDynamicMemoryWStream {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkWStream_destruct(self.base_mut());
        }
    }
}

impl fmt::Debug for DynamicMemoryWStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DynamicMemoryWStream")
            .field(
                "bytes_written_before_tail",
                &self.native().fBytesWrittenBeforeTail,
            )
            .finish()
    }
}

impl DynamicMemoryWStream {
    pub fn new() -> Self {
        Self::construct(|w_stream| unsafe { sb::C_SkDynamicMemoryWStream_Construct(w_stream) })
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut stream = Self::new();
        stream.write(bytes);
        stream
    }

    pub fn write(&mut self, bytes: &[u8]) -> bool {
        unsafe {
            sb::C_SkWStream_write(
                self.native_mut().base_mut(),
                bytes.as_ptr() as _,
                bytes.len(),
            )
        }
    }

    pub fn detach_as_data(&mut self) -> Data {
        Data::from_ptr(unsafe { sb::C_SkDynamicMemoryWStream_detachAsData(self.native_mut()) })
            .unwrap()
    }

    pub fn detach_as_stream(&mut self) -> StreamAsset {
        StreamAsset::from_ptr(unsafe {
            sb::C_SkDynamicMemoryWStream_detachAsStream(self.native_mut())
        })
        .unwrap()
    }
}

pub struct RustStream<'a> {
    inner: Handle<sb::RustStream>,
    _phantom: PhantomData<&'a mut ()>,
}

impl RustStream<'_> {
    pub fn stream_mut(&mut self) -> &mut SkStream {
        self.inner.native_mut().base_mut()
    }
}

impl NativeBase<SkStream> for sb::RustStream {}

impl NativeDrop for sb::RustStream {
    fn drop(&mut self) {
        unsafe { sb::C_RustStream_destruct(self) }
    }
}

impl<'a> RustStream<'a> {
    pub fn new<T: io::Read>(val: &'a mut T) -> Self {
        unsafe extern "C" fn read_trampoline<T>(
            val: *mut ffi::c_void,
            buf: *mut ffi::c_void,
            count: usize,
        ) -> usize
        where
            T: io::Read,
        {
            let val: &mut T = &mut *(val as *mut _);

            if buf.is_null() {
                const BUF_SIZE: usize = 128;

                let mut buf = [0; BUF_SIZE];

                let mut out_bytes = 0;
                let mut count = count;

                // This is OK because we just abort if it panics anyway, we don't try
                // to continue at all.
                let val = std::panic::AssertUnwindSafe(val);

                let out_bytes = match std::panic::catch_unwind(move || {
                    while count > 0 {
                        let bytes = match val.0.read(&mut buf[..count.min(BUF_SIZE)]) {
                            Ok(0) => break,
                            Ok(bytes) => bytes,
                            Err(_) => 0,
                        };

                        count -= bytes;
                        out_bytes += bytes;
                    }

                    out_bytes
                }) {
                    Ok(res) => res,
                    Err(_) => {
                        println!("Panic in FFI callback for `SkStream::read`");
                        std::process::abort();
                    }
                };

                out_bytes
            } else {
                let buf: &mut [u8] = std::slice::from_raw_parts_mut(buf as _, count as _);

                val.read(buf).unwrap_or(0)
            }
        }

        let (length, seek_start, seek_current): (
            usize,
            Option<unsafe extern "C" fn(_, _) -> _>,
            Option<unsafe extern "C" fn(_, _) -> _>,
        );

        #[cfg(feature = "nightly")]
        {
            trait MaybeSeek {
                fn maybe_seek(&mut self, from: io::SeekFrom) -> Option<u64>;
            }

            impl<T> MaybeSeek for T {
                default fn maybe_seek(&mut self, _: io::SeekFrom) -> Option<u64> {
                    None
                }
            }

            impl<T> MaybeSeek for T
            where
                T: io::Seek,
            {
                fn maybe_seek(&mut self, from: io::SeekFrom) -> Option<u64> {
                    self.seek(from).ok()
                }
            }

            unsafe extern "C" fn seek_start_trampoline<T: MaybeSeek>(
                val: *mut ffi::c_void,
                pos: usize,
            ) -> bool {
                let val: &mut T = &mut *(val as *mut _);

                // This is OK because we just abort if it panics anyway, we don't try
                // to continue at all.
                let val = std::panic::AssertUnwindSafe(val);

                match std::panic::catch_unwind(move || {
                    val.0.maybe_seek(io::SeekFrom::Start(pos as _)).is_some()
                }) {
                    Ok(res) => res,
                    Err(_) => {
                        println!("Panic in FFI callback for `SkStream::seek`");
                        std::process::abort();
                    }
                }
            }

            unsafe extern "C" fn seek_current_trampoline<T: MaybeSeek>(
                val: *mut ffi::c_void,
                offset: libc::c_long,
            ) -> bool {
                let val: &mut T = &mut *(val as *mut _);

                // This is OK because we just abort if it panics anyway, we don't try
                // to continue at all.
                let val = std::panic::AssertUnwindSafe(val);

                match std::panic::catch_unwind(move || {
                    val.0
                        .maybe_seek(io::SeekFrom::Current(offset as _))
                        .is_some()
                }) {
                    Ok(res) => res,
                    Err(_) => {
                        println!("Panic in FFI callback for `SkStream::move`");
                        std::process::abort();
                    }
                }
            }

            length = if let Some(cur) = val.maybe_seek(io::SeekFrom::Current(0)) {
                let length = val.maybe_seek(io::SeekFrom::End(0)).unwrap();

                val.maybe_seek(io::SeekFrom::Start(cur));

                length as usize
            } else {
                std::usize::MAX
            };

            seek_start = Some(seek_start_trampoline::<T>);
            seek_current = Some(seek_current_trampoline::<T>);
        }

        #[cfg(not(feature = "nightly"))]
        {
            length = usize::MAX;
            seek_start = None;
            seek_current = None;
        }

        RustStream {
            inner: Handle::construct(|ptr| unsafe {
                sb::C_RustStream_construct(
                    ptr,
                    val as *mut T as *mut ffi::c_void,
                    length,
                    Some(read_trampoline::<T>),
                    seek_start,
                    seek_current,
                );
            }),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MemoryStream, RustStream};
    use crate::interop::DynamicMemoryWStream;

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

    #[test]
    fn read_from_rust_stream() {
        let mut data: &[u8] = &[12u8, 13u8, 14u8];
        let mut stream = RustStream::new(&mut data);

        let mut first_byte = 0i8;
        unsafe {
            stream.stream_mut().readS8(&mut first_byte);
        }
        assert_eq!(first_byte, 12i8)
    }
}


pub struct RustStream<'a> {
    inner: Handle<sb::RustStream>,
    _phantom: PhantomData<&'a mut ()>,
}

impl RustStream<'_> {
    pub fn stream_mut(&mut self) -> &mut SkStream {
        self.inner.native_mut().base_mut()
    }
}

impl NativeBase<SkStream> for sb::RustStream {}

impl NativeDrop for sb::RustStream {
    fn drop(&mut self) {}
}

impl<'a> RustStream<'a> {
    pub fn new<T: io::Read>(val: &'a mut T) -> Self {
        unsafe extern "C" fn read_trampoline<T>(
            val: *mut ffi::c_void,
            buf: *mut ffi::c_void,
            count: usize,
        ) -> usize
            where
                T: io::Read,
        {
            let val: &mut T = &mut *(val as *mut _);

            if buf.is_null() {
                const BUF_SIZE: usize = 128;

                let mut buf = [0; BUF_SIZE];

                let mut out_bytes = 0;
                let mut count = count;

                // This is OK because we just abort if it panics anyway, we don't try
                // to continue at all.
                let val = std::panic::AssertUnwindSafe(val);

                let out_bytes = match std::panic::catch_unwind(move || {
                    while count > 0 {
                        let bytes = match val.0.read(&mut buf[..count.min(BUF_SIZE)]) {
                            Ok(0) => break,
                            Ok(bytes) => bytes,
                            Err(_) => 0,
                        };

                        count -= bytes;
                        out_bytes += bytes;
                    }

                    out_bytes
                }) {
                    Ok(res) => res,
                    Err(_) => {
                        println!("Panic in FFI callback for `SkStream::read`");
                        std::process::abort();
                    }
                };

                out_bytes
            } else {
                let buf: &mut [u8] = std::slice::from_raw_parts_mut(buf as _, count as _);

                match val.read(buf) {
                    Ok(bytes) => bytes,
                    Err(_) => 0,
                }
            }
        }

        let (length, seek_start, seek_current): (
            usize,
            Option<unsafe extern "C" fn(_, _) -> _>,
            Option<unsafe extern "C" fn(_, _) -> _>,
        );

        #[cfg(feature = "nightly")]
            {
                trait MaybeSeek {
                    fn maybe_seek(&mut self, from: io::SeekFrom) -> Option<u64>;
                }

                impl<T> MaybeSeek for T {
                    default fn maybe_seek(&mut self, _: io::SeekFrom) -> Option<u64> {
                        None
                    }
                }

                impl<T> MaybeSeek for T
                    where
                        T: io::Seek,
                {
                    fn maybe_seek(&mut self, from: io::SeekFrom) -> Option<u64> {
                        self.seek(from).ok()
                    }
                }

                unsafe extern "C" fn seek_start_trampoline<T: MaybeSeek>(
                    val: *mut ffi::c_void,
                    pos: usize,
                ) -> bool {
                    let val: &mut T = &mut *(val as *mut _);

                    // This is OK because we just abort if it panics anyway, we don't try
                    // to continue at all.
                    let val = std::panic::AssertUnwindSafe(val);

                    match std::panic::catch_unwind(move || {
                        val.0.maybe_seek(io::SeekFrom::Start(pos as _)).is_some()
                    }) {
                        Ok(res) => res,
                        Err(_) => {
                            println!("Panic in FFI callback for `SkStream::seek`");
                            std::process::abort();
                        }
                    }
                }

                unsafe extern "C" fn seek_current_trampoline<T: MaybeSeek>(
                    val: *mut ffi::c_void,
                    offset: libc::c_long,
                ) -> bool {
                    let val: &mut T = &mut *(val as *mut _);

                    // This is OK because we just abort if it panics anyway, we don't try
                    // to continue at all.
                    let val = std::panic::AssertUnwindSafe(val);

                    match std::panic::catch_unwind(move || {
                        val.0
                            .maybe_seek(io::SeekFrom::Current(offset as _))
                            .is_some()
                    }) {
                        Ok(res) => res,
                        Err(_) => {
                            println!("Panic in FFI callback for `SkStream::move`");
                            std::process::abort();
                        }
                    }
                }

                length = if let Some(cur) = val.maybe_seek(io::SeekFrom::Current(0)) {
                    let length = val.maybe_seek(io::SeekFrom::End(0)).unwrap();

                    val.maybe_seek(io::SeekFrom::Start(cur));

                    length as usize
                } else {
                    std::usize::MAX
                };

                seek_start = Some(seek_start_trampoline::<T>);
                seek_current = Some(seek_current_trampoline::<T>);
            }

        #[cfg(not(feature = "nightly"))]
            {
                length = usize::MAX;
                seek_start = None;
                seek_current = None;
            }

        RustStream {
            inner: Handle::construct(|ptr| unsafe {
                sb::C_RustStream_construct(
                    ptr,
                    val as *mut T as *mut ffi::c_void,
                    length,
                    Some(read_trampoline::<T>),
                    seek_start,
                    seek_current,
                );
            }),
            _phantom: PhantomData,
        }
    }
} 