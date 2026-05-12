#include <algorithm>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <new>

extern "C" void* skia_bindings_alloc(size_t size, size_t align);
extern "C" void* skia_bindings_alloc_zeroed(size_t size, size_t align);
extern "C" void skia_bindings_dealloc(void* ptr, size_t size, size_t align);

namespace {

constexpr size_t kBaseAlignment = alignof(std::max_align_t);

struct AllocHeader {
    uintptr_t base_ptr;
    size_t total_size;
    size_t alignment;
};

[[noreturn]] void trap_out_of_memory() {
#if defined(__clang__)
    __builtin_trap();
#else
    abort();
#endif
}

bool is_power_of_two(size_t value) {
    return value != 0 && (value & (value - 1)) == 0;
}

bool checked_add(size_t a, size_t b, size_t* out) {
    if (a > SIZE_MAX - b) {
        return false;
    }
    *out = a + b;
    return true;
}

uintptr_t align_up(uintptr_t value, size_t alignment) {
    return (value + (uintptr_t)(alignment - 1)) & ~((uintptr_t)alignment - 1);
}

AllocHeader* header_from_ptr(const void* ptr) {
    return reinterpret_cast<AllocHeader*>(reinterpret_cast<uintptr_t>(ptr) - sizeof(AllocHeader));
}

void* allocate_block(size_t size, size_t alignment, bool zeroed, bool throw_on_failure) {
    const size_t normalized_size = size == 0 ? 1 : size;
    const size_t normalized_alignment = std::max(alignment, kBaseAlignment);

    if (!is_power_of_two(normalized_alignment)) {
        if (throw_on_failure) {
            trap_out_of_memory();
        }
        return nullptr;
    }

    size_t overhead;
    if (!checked_add(sizeof(AllocHeader), normalized_alignment - 1, &overhead)) {
        if (throw_on_failure) {
            trap_out_of_memory();
        }
        return nullptr;
    }

    size_t total_size;
    if (!checked_add(normalized_size, overhead, &total_size)) {
        if (throw_on_failure) {
            trap_out_of_memory();
        }
        return nullptr;
    }

    void* base = zeroed ? skia_bindings_alloc_zeroed(total_size, kBaseAlignment)
                        : skia_bindings_alloc(total_size, kBaseAlignment);
    if (!base) {
        if (throw_on_failure) {
            trap_out_of_memory();
        }
        return nullptr;
    }

    const uintptr_t raw = reinterpret_cast<uintptr_t>(base) + sizeof(AllocHeader);
    const uintptr_t user = align_up(raw, normalized_alignment);
    auto* header = reinterpret_cast<AllocHeader*>(user - sizeof(AllocHeader));
    header->base_ptr = reinterpret_cast<uintptr_t>(base);
    header->total_size = total_size;
    header->alignment = normalized_alignment;

    return reinterpret_cast<void*>(user);
}

void deallocate_block(void* ptr) {
    if (!ptr) {
        return;
    }

    AllocHeader* header = header_from_ptr(ptr);
    skia_bindings_dealloc(reinterpret_cast<void*>(header->base_ptr), header->total_size, kBaseAlignment);
}

}  // namespace

void* operator new(std::size_t size) {
    return allocate_block(size, kBaseAlignment, false, true);
}

void* operator new[](std::size_t size) {
    return allocate_block(size, kBaseAlignment, false, true);
}

void* operator new(std::size_t size, const std::nothrow_t&) noexcept {
    return allocate_block(size, kBaseAlignment, false, false);
}

void* operator new[](std::size_t size, const std::nothrow_t&) noexcept {
    return allocate_block(size, kBaseAlignment, false, false);
}

void* operator new(std::size_t size, std::align_val_t alignment) {
    return allocate_block(size, static_cast<size_t>(alignment), false, true);
}

void* operator new[](std::size_t size, std::align_val_t alignment) {
    return allocate_block(size, static_cast<size_t>(alignment), false, true);
}

void* operator new(std::size_t size, std::align_val_t alignment, const std::nothrow_t&) noexcept {
    return allocate_block(size, static_cast<size_t>(alignment), false, false);
}

void* operator new[](std::size_t size, std::align_val_t alignment, const std::nothrow_t&) noexcept {
    return allocate_block(size, static_cast<size_t>(alignment), false, false);
}

void operator delete(void* ptr) noexcept {
    deallocate_block(ptr);
}

void operator delete[](void* ptr) noexcept {
    deallocate_block(ptr);
}

void operator delete(void* ptr, std::size_t) noexcept {
    deallocate_block(ptr);
}

void operator delete[](void* ptr, std::size_t) noexcept {
    deallocate_block(ptr);
}

void operator delete(void* ptr, const std::nothrow_t&) noexcept {
    deallocate_block(ptr);
}

void operator delete[](void* ptr, const std::nothrow_t&) noexcept {
    deallocate_block(ptr);
}

void operator delete(void* ptr, std::align_val_t) noexcept {
    deallocate_block(ptr);
}

void operator delete[](void* ptr, std::align_val_t) noexcept {
    deallocate_block(ptr);
}

void operator delete(void* ptr, std::size_t, std::align_val_t) noexcept {
    deallocate_block(ptr);
}

void operator delete[](void* ptr, std::size_t, std::align_val_t) noexcept {
    deallocate_block(ptr);
}
