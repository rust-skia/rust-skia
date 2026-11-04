// rust-skia: Replaces Dawn's internal `src/utils/assert.h` for `dawn_proc.cpp`.
//
// See `src/dawn/common/Compiler.h` in this directory for why these replacements exist.

#ifndef RUST_SKIA_DAWN_COMPAT_ASSERT_H_
#define RUST_SKIA_DAWN_COMPAT_ASSERT_H_

#include <cstdio>
#include <cstdlib>

// `dawnProcSetProcs` uses this check to reject a function table whose Dawn revision stamp differs
// from the one `dawn_proc.cpp` was generated with.
#define DAWN_CHECK(condition)                                                     \
    do {                                                                          \
        if (!(condition)) {                                                       \
            std::fprintf(stderr, "dawn_proc: check failed: %s\n", #condition);    \
            std::abort();                                                         \
        }                                                                         \
    } while (false)

#endif  // RUST_SKIA_DAWN_COMPAT_ASSERT_H_
