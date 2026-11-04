//! `SkStream` and relatives.
//! This implementation covers the minimal subset to interface with Rust streams.
//!
//! Bindings that wrap functions that use Skia stream types _must_ use Rust streams instead.

use std::{ffi, fmt, io, marker::PhantomData, mem, ptr};

use skia_bindings::{
    self as sb, SkDynamicMemoryWStream, SkMemoryStream, SkStream, SkStreamAsset, SkWStream,
};

use crate::{Data, prelude::*};

/// Trait representing an Skia allocated Stream type with a base class of [`Stream`].
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

impl NativeAccess for StreamAsset {
    type Native = SkStreamAsset;

    fn native(&self) -> &SkStreamAsset {
        unsafe { self.0.as_ref() }
    }
}

impl NativeAccessMut for StreamAsset {
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

impl Drop for MemoryStream<'_> {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkStream_delete(self.native_mut().as_stream_mut());
        }
    }
}

impl NativeAccess for MemoryStream<'_> {
    type Native = SkMemoryStream;

    fn native(&self) -> &SkMemoryStream {
        unsafe { self.native.as_ref() }
    }
}

impl NativeAccessMut for MemoryStream<'_> {
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
    #[allow(unused)]
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

#[allow(unused)]
pub struct RustStream<'a> {
    // This can't be a handle, because we need to be able to create a "deletable" C++ SkStream*.
    inner: RefHandle<sb::RustStream>,
    _phantom: PhantomData<&'a mut ()>,
}

#[allow(unused)]
impl RustStream<'_> {
    /// Returns a pointer to the stream for C++ calls that read from it.
    pub fn stream_ptr(&mut self) -> *mut SkStream {
        self.inner.native_ptr().cast()
    }

    pub fn into_native(mut self) -> *mut SkStream {
        let stream = self.inner.native_ptr().cast();
        mem::forget(self.inner);
        stream
    }
}

impl NativeBase<SkStream> for sb::RustStream {}

impl NativeDelete for sb::RustStream {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_RustStream_delete(ptr) }
    }
}

#[allow(unused)]
impl<'a> RustStream<'a> {
    pub fn new_seekable<T: io::Read + io::Seek>(val: &'a mut T) -> Self {
        Self {
            inner: RefHandle::from_ptr(unsafe {
                sb::C_RustStream_new(
                    val as *mut T as *mut ffi::c_void,
                    usize::MAX,
                    Some(read_trampoline::<T>),
                    Some(seek_start_trampoline::<T>),
                    Some(seek_current_trampoline::<T>),
                )
            })
            .unwrap(),
            _phantom: PhantomData,
        }
    }

    pub fn new<T: io::Read>(val: &'a mut T) -> Self {
        Self {
            inner: RefHandle::from_ptr(unsafe {
                sb::C_RustStream_new(
                    val as *mut T as *mut ffi::c_void,
                    usize::MAX,
                    Some(read_trampoline::<T>),
                    None,
                    None,
                )
            })
            .unwrap(),
            _phantom: PhantomData,
        }
    }
}

unsafe extern "C" fn read_trampoline<T>(
    val: *mut ffi::c_void,
    buf: *mut ffi::c_void,
    count: usize,
) -> usize
where
    T: io::Read,
{
    unsafe {
        let val: &mut T = &mut *(val as *mut _);

        if buf.is_null() {
            const BUF_SIZE: usize = 128;

            let mut buf = [0; BUF_SIZE];

            let mut out_bytes = 0;
            let mut count = count;

            while count > 0 {
                // A read error ends the skip like the end of the stream, because retrying an
                // error that persists never returns (`skip_stops_at_read_error`).
                let bytes = match val.read(&mut buf[..count.min(BUF_SIZE)]) {
                    Ok(0) | Err(_) => break,
                    Ok(bytes) => bytes,
                };

                count -= bytes;
                out_bytes += bytes;
            }

            out_bytes
        } else {
            let buf: &mut [u8] = std::slice::from_raw_parts_mut(buf as _, count as _);

            val.read(buf).unwrap_or(0)
        }
    }
}

unsafe extern "C" fn seek_start_trampoline<T: io::Seek>(val: *mut ffi::c_void, pos: usize) -> bool {
    unsafe {
        let val: &mut T = &mut *(val as *mut _);
        val.seek(io::SeekFrom::Start(pos as _)).is_ok()
    }
}

unsafe extern "C" fn seek_current_trampoline<T: io::Seek>(
    val: *mut ffi::c_void,
    offset: ffi::c_long,
) -> bool {
    unsafe {
        let val: &mut T = &mut *(val as *mut _);
        val.seek(io::SeekFrom::Current(offset as _)).is_ok()
    }
}

#[allow(unused)]
pub struct RustWStream<'a> {
    /// The native stream that C++ writes into.
    ///
    /// C++ may keep a pointer to the stream. For example, a PDF document writes into its stream
    /// until the document is closed. The stream lives on the heap, so its address stays the same
    /// when this `RustWStream` moves. It is held through a raw pointer instead of a `Box`, because
    /// Rust assumes that nothing else changes the contents of a `Box` while the value that owns
    /// it is passed to a function. C++ writes to the stream during such calls, for example when
    /// `Document::begin_page()` writes the PDF header, and with a `Box` that would be undefined
    /// behavior.
    inner: ptr::NonNull<Handle<sb::RustWStream>>,
    _phantom: PhantomData<&'a mut ()>,
}

impl Drop for RustWStream<'_> {
    /// Freeing the stream here, and not through a field's drop, keeps the writer borrowed for
    /// `'a` until the C++ stream and any document that writes into it are gone.
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.inner.as_ptr()) })
    }
}

#[allow(unused)]
impl RustWStream<'_> {
    /// Returns a pointer to the stream that C++ may keep and write through.
    pub fn stream_ptr(&mut self) -> *mut SkWStream {
        self.inner.as_ptr().cast()
    }
}

impl NativeBase<SkWStream> for sb::RustWStream {}

impl NativeDrop for sb::RustWStream {
    fn drop(&mut self) {
        unsafe { sb::C_RustWStream_destruct(self) }
    }
}

impl<'a> RustWStream<'a> {
    pub fn new<T: io::Write>(writer: &'a mut T) -> Self {
        let inner = Box::new(Handle::construct(|ptr| unsafe {
            sb::C_RustWStream_construct(
                ptr,
                writer as *mut T as *mut ffi::c_void,
                Some(write_trampoline::<T>),
                Some(flush_trampoline::<T>),
            );
        }));
        return RustWStream {
            inner: ptr::NonNull::new(Box::into_raw(inner)).unwrap(),
            _phantom: PhantomData,
        };

        unsafe extern "C" fn write_trampoline<T: io::Write>(
            val: *mut ffi::c_void,
            buf: *const ffi::c_void,
            count: usize,
        ) -> bool {
            unsafe {
                if count == 0 {
                    return true;
                }
                let buf: &[u8] = std::slice::from_raw_parts(buf as _, count as _);
                let val: &mut T = &mut *(val as *mut _);

                let mut written = 0;
                while written != count {
                    match val.write(&buf[written..]) {
                        Ok(res) if res != 0 => {
                            written += res;
                        }
                        _ => return false,
                    }
                }
                true
            }
        }

        unsafe extern "C" fn flush_trampoline<T: io::Write>(val: *mut ffi::c_void) {
            unsafe {
                let val: &mut T = &mut *(val as *mut _);
                // Not sure what could be done to handle a flush() error.
                // Idea: use a with_stream method on the RustWStream that takes a closure, stores
                // the flush() result and then return a result from with_stream.
                let _flush_result_ignored = val.flush();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io, ptr};

    use skia_bindings as sb;

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
            skia_bindings::SkStream_readS8(stream.stream_ptr(), &mut first_byte);
        }
        assert_eq!(first_byte, 12i8)
    }

    /// Skipping reads into a null buffer. A read error must end the skip, because retrying an
    /// error that persists never returns.
    #[test]
    fn skip_stops_at_read_error() {
        let mut reader = FailingReader { reads: 0 };
        let mut stream = RustStream::new(&mut reader);
        let skipped = unsafe { sb::C_SkStream_read(stream.stream_ptr(), ptr::null_mut(), 10) };
        drop(stream);
        assert_eq!(skipped, 0);
        assert_eq!(reader.reads, 1);
    }

    /// Fails its first read and reports the end of the stream afterwards, so that a skip that
    /// retries after the error still returns and the test fails instead of hanging.
    struct FailingReader {
        reads: usize,
    }

    impl io::Read for FailingReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            self.reads += 1;
            if self.reads == 1 {
                Err(io::Error::other("read failed"))
            } else {
                Ok(0)
            }
        }
    }
}
