#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type __wasi_fd_t = u32;
pub type __wasi_filesize_t = u64;
pub type __wasi_clockid_t = u32;
pub type __wasi_timestamp_t = u64;
pub type __wasi_whence_t = u16;
pub type __wasi_errno_t = u16;
pub type __wasi_size_t = u32;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __wasi_ciovec_t {
    pub buf: *const c_void,
    pub buf_len: usize,
}

// __WASI_ERRNO_BADF
const WASI_BADF: __wasi_errno_t = 8;
const WASI_OK: __wasi_errno_t = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_clock_time_get(
    _id: __wasi_clockid_t,
    _precision: __wasi_timestamp_t,
    time: *mut __wasi_timestamp_t,
) -> __wasi_errno_t {
    if !time.is_null() {
        unsafe {
            *time = 0;
        }
    }
    WASI_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_close(
    _fd: __wasi_fd_t,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_seek(
    _fd: __wasi_fd_t,
    _offset: __wasi_filesize_t,
    _whence: __wasi_whence_t,
    newoffset: *mut __wasi_filesize_t,
) -> __wasi_errno_t {
    if !newoffset.is_null() {
        unsafe {
            *newoffset = 0;
        }
    }
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_environ_get(
    _environ: *mut *mut c_char,
    _environ_buf: *mut c_char,
) -> __wasi_errno_t {
    WASI_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_environ_sizes_get(
    environ_count: *mut __wasi_size_t,
    environ_buf_size: *mut __wasi_size_t,
) -> __wasi_errno_t {
    if !environ_count.is_null() {
        unsafe {
            *environ_count = 0;
        }
    }
    if !environ_buf_size.is_null() {
        unsafe {
            *environ_buf_size = 0;
        }
    }
    WASI_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_prestat_get(
    _fd: __wasi_fd_t,
    _prestat: *mut c_void,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_prestat_dir_name(
    _fd: __wasi_fd_t,
    _path: *mut c_char,
    _path_len: __wasi_size_t,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub extern "C" fn __imported_wasi_snapshot_preview1_proc_exit(_code: __wasi_size_t) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sem_destroy(_sem: *mut c_void) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sem_post(_sem: *mut c_void) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sem_wait(_sem: *mut c_void) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sem_init(_sem: *mut c_void, _pshared: i32, _value: u32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_write(
    _fd: __wasi_fd_t,
    _iovs: *const __wasi_ciovec_t,
    _iovs_len: usize,
    nwritten: *mut usize,
) -> __wasi_errno_t {
    if !nwritten.is_null() {
        unsafe {
            *nwritten = 0;
        }
    }
    WASI_BADF
}
