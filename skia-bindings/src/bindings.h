#ifndef SKIA_BINDINGS_BINDINGS_H
#define SKIA_BINDINGS_BINDINGS_H

#include "include/core/SkRefCnt.h"

template<typename T>
inline sk_sp<T> spFromConst(const T* pt) {
    return sk_sp<T>(const_cast<T*>(pt));
}

#endif //SKIA_BINDINGS_BINDINGS_H
