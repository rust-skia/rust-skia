#include <stddef.h>
#include <stdint.h>

typedef uint32_t __wasi_fd_t;
typedef uint64_t __wasi_filesize_t;
typedef uint32_t __wasi_clockid_t;
typedef uint64_t __wasi_timestamp_t;
typedef uint16_t __wasi_whence_t;
typedef uint16_t __wasi_errno_t;
typedef uint32_t __wasi_size_t;

typedef struct {
    const void* buf;
    size_t buf_len;
} __wasi_ciovec_t;

// __WASI_ERRNO_BADF
#define WASI_BADF ((__wasi_errno_t)8)
#define WASI_OK ((__wasi_errno_t)0)

__wasi_errno_t __imported_wasi_snapshot_preview1_clock_time_get(
    __wasi_clockid_t id,
    __wasi_timestamp_t precision,
    __wasi_timestamp_t* time
) {
    (void)id;
    (void)precision;
    if (time) {
        *time = 0;
    }
    return WASI_OK;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_close(__wasi_fd_t fd) {
    (void)fd;
    return WASI_BADF;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_seek(
    __wasi_fd_t fd,
    __wasi_filesize_t offset,
    __wasi_whence_t whence,
    __wasi_filesize_t* newoffset
) {
    (void)fd;
    (void)offset;
    (void)whence;
    if (newoffset) {
        *newoffset = 0;
    }
    return WASI_BADF;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_environ_get(
    char** environ,
    char* environ_buf
) {
    (void)environ;
    (void)environ_buf;
    return WASI_OK;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_environ_sizes_get(
    __wasi_size_t* environ_count,
    __wasi_size_t* environ_buf_size
) {
    if (environ_count) {
        *environ_count = 0;
    }
    if (environ_buf_size) {
        *environ_buf_size = 0;
    }
    return WASI_OK;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_prestat_get(
    __wasi_fd_t fd,
    void* prestat
) {
    (void)fd;
    (void)prestat;
    return WASI_BADF;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_prestat_dir_name(
    __wasi_fd_t fd,
    char* path,
    __wasi_size_t path_len
) {
    (void)fd;
    (void)path;
    (void)path_len;
    return WASI_BADF;
}

void __imported_wasi_snapshot_preview1_proc_exit(__wasi_size_t code) {
    (void)code;
}

int sem_destroy(void* sem) {
    (void)sem;
    return 0;
}

int sem_post(void* sem) {
    (void)sem;
    return 0;
}

int sem_wait(void* sem) {
    (void)sem;
    return 0;
}

int sem_init(void* sem, int pshared, unsigned int value) {
    (void)sem;
    (void)pshared;
    (void)value;
    return 0;
}

__wasi_errno_t __imported_wasi_snapshot_preview1_fd_write(
    __wasi_fd_t fd,
    const __wasi_ciovec_t* iovs,
    size_t iovs_len,
    size_t* nwritten
) {
    (void)fd;
    (void)iovs;
    (void)iovs_len;
    if (nwritten) {
        *nwritten = 0;
    }
    return WASI_BADF;
}
