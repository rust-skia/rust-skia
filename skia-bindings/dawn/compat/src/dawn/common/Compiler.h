// rust-skia: Replaces Dawn's internal `src/dawn/common/Compiler.h` for `dawn_proc.cpp`.
//
// The generated `dawn_proc.cpp` includes three of Dawn's internal headers. rust-skia builds no
// other part of Dawn, so this directory provides only what `dawn_proc.cpp` uses from them.

#ifndef RUST_SKIA_DAWN_COMPAT_COMPILER_H_
#define RUST_SKIA_DAWN_COMPAT_COMPILER_H_

// Dawn uses this macro to exempt the calls through the function table from control flow
// integrity checks. Skia is not built with those checks, so the macro expands to nothing.
#define DAWN_NO_SANITIZE(instrumentation)

#endif  // RUST_SKIA_DAWN_COMPAT_COMPILER_H_
