#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type __wasi_fd_t = u32;
pub type __wasi_filesize_t = u64;
pub type __wasi_clockid_t = u32;
pub type __wasi_timestamp_t = u64;
pub type __wasi_whence_t = u16;
pub type __wasi_errno_t = u16;
pub type __wasi_size_t = u32;
pub type __wasi_rights_t = u64;
pub type __wasi_fdflags_t = u16;
pub type __wasi_lookupflags_t = u32;
pub type __wasi_oflags_t = u16;
pub type __wasi_dircookie_t = u64;

// __WASI_ERRNO_BADF
const WASI_BADF: __wasi_errno_t = 8;
const WASI_OK: __wasi_errno_t = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_clock_time_get(
    _id: __wasi_clockid_t,
    _precision: __wasi_timestamp_t,
    time: *mut __wasi_timestamp_t,
) -> __wasi_errno_t {
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
    _newoffset: *mut c_void,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_fdstat_get(
    _fd: __wasi_fd_t,
    _retptr0: *mut c_void,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_fdstat_set_flags(
    _fd: __wasi_fd_t,
    _flags: __wasi_fdflags_t,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_filestat_get(
    _fd: __wasi_fd_t,
    _retptr0: *mut c_void,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_pread(
    _fd: __wasi_fd_t,
    _iovs: *const c_void,
    _iovs_len: usize,
    _offset: __wasi_filesize_t,
    _retptr0: *mut c_void,
) -> __wasi_errno_t {
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
    _iovs: *const c_void,
    _iovs_len: usize,
    _nwritten: *mut c_void,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_read(
    _fd: __wasi_fd_t,
    _iovs: *const c_void,
    _iovs_len: usize,
    _retptr0: *mut c_void,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_fd_readdir(
    _fd: __wasi_fd_t,
    _buf: *mut c_void,
    _buf_len: __wasi_size_t,
    _cookie: __wasi_dircookie_t,
    _retptr0: *mut c_void,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_path_filestat_get(
    _fd: __wasi_fd_t,
    _flags: __wasi_lookupflags_t,
    _path: *const c_char,
    _retptr0: *mut c_void,
) -> __wasi_errno_t {
    WASI_BADF
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __imported_wasi_snapshot_preview1_path_open(
    _fd: __wasi_fd_t,
    _dirflags: __wasi_lookupflags_t,
    _path: *const c_char,
    _oflags: __wasi_oflags_t,
    _fs_rights_base: __wasi_rights_t,
    _fs_rights_inheriting: __wasi_rights_t,
    _fdflags: __wasi_fdflags_t,
    _retptr0: *mut c_void,
) -> __wasi_errno_t {
    WASI_BADF
}
