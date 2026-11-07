//! The C runtime functions that Emscripten's JS runtime or its malloc would otherwise supply, for
//! Skia compiled by the Emscripten SDK and linked into a `wasm32-unknown-unknown` module.
//!
//! `build.rs` links Emscripten's libc and libc++, whose remaining imports these functions resolve.
//! An application that supplies them itself sets `SKIA_WASM_NO_LIBC`.

use std::alloc::{Layout, alloc, alloc_zeroed, dealloc};
use std::ffi::{c_int, c_void};
use std::ptr::null_mut;

/// Every block starts with a header: its layout size and the offset of the user pointer.
const HEADER: usize = 16;

unsafe fn allocate(size: usize, align: usize, zeroed: bool) -> *mut c_void {
    let offset = align.max(HEADER);
    let Ok(layout) = Layout::from_size_align(size + offset, offset) else {
        return null_mut();
    };
    let base = unsafe {
        if zeroed {
            alloc_zeroed(layout)
        } else {
            alloc(layout)
        }
    };
    if base.is_null() {
        return null_mut();
    }
    unsafe {
        let user = base.add(offset);
        let header = user.sub(HEADER) as *mut usize;
        header.write(layout.size());
        header.add(1).write(offset);
        user.cast()
    }
}

/// The layout and base pointer of the block whose user pointer is `ptr`.
unsafe fn block(ptr: *mut c_void) -> (*mut u8, Layout) {
    unsafe {
        let header = (ptr as *mut u8).sub(HEADER) as *const usize;
        let (size, offset) = (header.read(), header.add(1).read());
        (
            (ptr as *mut u8).sub(offset),
            Layout::from_size_align_unchecked(size, offset),
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    unsafe { allocate(size, HEADER, false) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn calloc(count: usize, size: usize) -> *mut c_void {
    match count.checked_mul(size) {
        Some(total) => unsafe { allocate(total, HEADER, true) },
        None => null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            let (base, layout) = block(ptr);
            dealloc(base, layout);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    if ptr.is_null() {
        return unsafe { malloc(size) };
    }
    unsafe {
        let (_, layout) = block(ptr);
        let old_size = layout.size() - layout.align();
        let new = allocate(size, layout.align(), false);
        if !new.is_null() {
            std::ptr::copy_nonoverlapping(ptr as *const u8, new as *mut u8, old_size.min(size));
            free(ptr);
        }
        new
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aligned_alloc(align: usize, size: usize) -> *mut c_void {
    unsafe { allocate(size, align, false) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_memalign(out: *mut *mut c_void, align: usize, size: usize) -> c_int {
    let ptr = unsafe { allocate(size, align, false) };
    if ptr.is_null() {
        return 48; // ENOMEM in Emscripten's musl.
    }
    unsafe { out.write(ptr) };
    0
}

/// libpng and FreeType recover from errors with `longjmp`. Without unwinding, every `setjmp`
/// returns once, and `longjmp` aborts.
#[unsafe(no_mangle)]
pub extern "C" fn setjmp(_env: *mut c_void) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn longjmp(_env: *mut c_void, _value: c_int) -> ! {
    panic!("longjmp called: a C library hit an error it recovers from with longjmp");
}

// Emscripten's libc calls its allocator under these names, too.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libc_malloc(size: usize) -> *mut c_void {
    unsafe { malloc(size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emscripten_builtin_malloc(size: usize) -> *mut c_void {
    unsafe { malloc(size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libc_calloc(count: usize, size: usize) -> *mut c_void {
    unsafe { calloc(count, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libc_free(ptr: *mut c_void) {
    unsafe { free(ptr) }
}

// The functions that Emscripten's JS runtime implements for its libc.

#[unsafe(no_mangle)]
pub extern "C" fn _abort_js() -> ! {
    panic!("abort() called");
}

/// The clock of `clock_gettime(CLOCK_MONOTONIC)`, in milliseconds.
#[unsafe(no_mangle)]
pub extern "C" fn emscripten_get_now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map_or(0.0, |p| p.now())
}

/// Reports UTC without daylight saving time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _tzset_js(
    timezone: *mut i32,
    daylight: *mut c_int,
    std_name: *mut u8,
    dst_name: *mut u8,
) {
    unsafe {
        timezone.write(0);
        daylight.write(0);
        std::ptr::copy_nonoverlapping(b"UTC\0".as_ptr(), std_name, 4);
        std::ptr::copy_nonoverlapping(b"UTC\0".as_ptr(), dst_name, 4);
    }
}

// The WASI functions that Emscripten's libc imports. Skia only writes diagnostics, so files and
// the environment are empty, and both output streams go to the console.

const WASI_SUCCESS: u16 = 0;
const WASI_EBADF: u16 = 8;

#[repr(C)]
pub struct CioVec {
    buf: *const u8,
    len: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wasi_fd_write(
    fd: u32,
    iovs: *const CioVec,
    iovs_len: usize,
    nwritten: *mut usize,
) -> u16 {
    if fd != 1 && fd != 2 {
        return WASI_EBADF;
    }
    let mut text = Vec::new();
    unsafe {
        for iov in std::slice::from_raw_parts(iovs, iovs_len) {
            text.extend_from_slice(std::slice::from_raw_parts(iov.buf, iov.len));
        }
        nwritten.write(text.len());
    }
    let text = String::from_utf8_lossy(&text);
    let text = text.trim_end();
    if !text.is_empty() {
        web_sys::console::log_1(&format!("[skia fd {fd}] {text}").into());
    }
    WASI_SUCCESS
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasi_fd_close(_fd: u32) -> u16 {
    WASI_SUCCESS
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasi_fd_seek(
    _fd: u32,
    _offset: i64,
    _whence: u8,
    _new_offset: *mut u64,
) -> u16 {
    WASI_EBADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wasi_environ_sizes_get(count: *mut usize, buf_size: *mut usize) -> u16 {
    unsafe {
        count.write(0);
        buf_size.write(0);
    }
    WASI_SUCCESS
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasi_environ_get(_environ: *mut *mut u8, _buf: *mut u8) -> u16 {
    WASI_SUCCESS
}

/// Nanoseconds of the page's monotonic clock, for every clock ID.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wasi_clock_time_get(_id: u32, _precision: u64, time: *mut u64) -> u16 {
    unsafe { time.write((emscripten_get_now() * 1_000_000.0) as u64) };
    WASI_SUCCESS
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emscripten_builtin_free(ptr: *mut c_void) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emscripten_builtin_memalign(align: usize, size: usize) -> *mut c_void {
    unsafe { aligned_alloc(align, size) }
}

// The file system calls that Emscripten's JS runtime implements. The browser has no files, so
// they fail with Emscripten's errno values, negated as syscalls return them. C varargs are one
// pointer argument in wasm.

const ENOENT: c_int = 44;
const EBADF: c_int = 8;
const ENOSYS: c_int = 52;

#[unsafe(no_mangle)]
pub extern "C" fn __syscall_openat(
    _dirfd: c_int,
    _path: *const u8,
    _flags: c_int,
    _varargs: *mut c_void,
) -> c_int {
    -ENOENT
}

#[unsafe(no_mangle)]
pub extern "C" fn __syscall_stat64(_path: *const u8, _buf: *mut c_void) -> c_int {
    -ENOENT
}

#[unsafe(no_mangle)]
pub extern "C" fn __syscall_lstat64(_path: *const u8, _buf: *mut c_void) -> c_int {
    -ENOENT
}

#[unsafe(no_mangle)]
pub extern "C" fn __syscall_newfstatat(
    _dirfd: c_int,
    _path: *const u8,
    _buf: *mut c_void,
    _flags: c_int,
) -> c_int {
    -ENOENT
}

#[unsafe(no_mangle)]
pub extern "C" fn __syscall_fstat64(_fd: c_int, _buf: *mut c_void) -> c_int {
    -EBADF
}

#[unsafe(no_mangle)]
pub extern "C" fn __syscall_getdents64(_fd: c_int, _dirp: *mut c_void, _count: usize) -> c_int {
    -EBADF
}

#[unsafe(no_mangle)]
pub extern "C" fn __syscall_fcntl64(_fd: c_int, _cmd: c_int, _varargs: *mut c_void) -> c_int {
    -EBADF
}

#[unsafe(no_mangle)]
pub extern "C" fn __syscall_ioctl(_fd: c_int, _request: c_int, _varargs: *mut c_void) -> c_int {
    -EBADF
}

#[unsafe(no_mangle)]
pub extern "C" fn _mmap_js(
    _length: usize,
    _prot: c_int,
    _flags: c_int,
    _fd: c_int,
    _offset: i64,
    _allocated: *mut c_int,
    _addr: *mut *mut c_void,
) -> c_int {
    -ENOSYS
}

#[unsafe(no_mangle)]
pub extern "C" fn _munmap_js(
    _addr: *mut c_void,
    _length: usize,
    _prot: c_int,
    _flags: c_int,
    _fd: c_int,
    _offset: i64,
) -> c_int {
    -ENOSYS
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasi_fd_read(
    _fd: u32,
    _iovs: *const c_void,
    _iovs_len: usize,
    _nread: *mut usize,
) -> u16 {
    WASI_EBADF
}

#[unsafe(no_mangle)]
pub extern "C" fn __wasi_fd_pread(
    _fd: u32,
    _iovs: *const c_void,
    _iovs_len: usize,
    _offset: u64,
    _nread: *mut usize,
) -> u16 {
    WASI_EBADF
}
