// rust-skia: Replaces Dawn's internal `src/utils/log.h` for `dawn_proc.cpp`.
//
// See `src/dawn/common/Compiler.h` in this directory for why these replacements exist.

#ifndef RUST_SKIA_DAWN_COMPAT_LOG_H_
#define RUST_SKIA_DAWN_COMPAT_LOG_H_

#include <cstdio>

namespace dawn {

// An error message that is written to stderr as one line when it goes out of scope.
// `dawn_proc.cpp` uses it to warn when `wgpuCreateInstance` is called before a function table is
// installed.
class ErrorLogMessage {
  public:
    ErrorLogMessage() = default;
    ErrorLogMessage(const ErrorLogMessage&) = delete;
    ErrorLogMessage& operator=(const ErrorLogMessage&) = delete;
    ~ErrorLogMessage() { std::fputc('\n', stderr); }

    ErrorLogMessage& operator<<(const char* text) {
        std::fputs(text, stderr);
        return *this;
    }
};

inline ErrorLogMessage ErrorLog() {
    return {};
}

}  // namespace dawn

#endif  // RUST_SKIA_DAWN_COMPAT_LOG_H_
