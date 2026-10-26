#ifndef SKIA_BINDINGS_BINDINGS_H
#define SKIA_BINDINGS_BINDINGS_H

#include <vector>
#include <optional>
#include "include/core/SkRefCnt.h"
#include "include/core/SkString.h"

template<typename T>
inline sk_sp<T> spFromConst(const T* pt) {
    return sk_sp<T>(const_cast<T*>(pt));
}

template<typename T>
inline sk_sp<T> sp(T* pt) {
    return sk_sp<T>(pt);
}

template<typename T>
inline std::optional<T> opt(const T* pt) {
    return pt ? std::optional<T>(*pt) : std::nullopt;
}

extern "C" struct TraitObject {
    void* data;
    void* vtable;
};

/// A VecSink is passed from Rust to C++ for receiving a slice of values.
template<typename T> struct VecSink {
    // The Rust closure, borrowed for the duration of the call. Only `set_fn` knows its type.
    void* data;
    void (*set_fn)(T *, size_t, void*);

    void set(T* ptr, size_t len) {
        set_fn(ptr, len, data);
    }

    void set(std::vector<T>& v) {
        if (v.empty()) {
            set_fn(nullptr, 0, data);
        } else {
            set_fn(v.data(), v.size(), data);
        }
    }
};

template<typename T> struct Sink {
    // The Rust closure, borrowed for the duration of the call. Only `set_fn` knows its type.
    void* data;
    void (*set_fn)(const T *, void*);

    void set(const T& value) {
        set_fn(&value, data);
    }
};

struct SkStrings {
    std::vector<SkString> strings;
};

#endif //SKIA_BINDINGS_BINDINGS_H
