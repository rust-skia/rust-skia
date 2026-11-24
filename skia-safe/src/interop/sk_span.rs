//! Access to `SkSpan` fields, which bindgen emits as opaque bytes.
//!
//! `SkSpan` keeps its pointer and size in a templated private base class with a non-type template
//! parameter, which bindgen cannot represent. For the dynamic extent that every Skia struct field
//! uses, the base class holds the pointer first and the size second.

use std::mem;

/// The bytes that bindgen emits for a field of type `SkSpan<T>`.
pub type SpanBytes = skia_bindings::__BindgenOpaqueArray8<[u8; 16]>;

#[repr(C)]
struct Span<T> {
    ptr: *const T,
    size: usize,
}

const _: () = assert!(mem::size_of::<Span<u8>>() == mem::size_of::<SpanBytes>());
const _: () = assert!(mem::align_of::<Span<u8>>() == mem::align_of::<SpanBytes>());

/// Returns the pointer and the size of an `SkSpan<T>` field.
///
/// The caller guarantees that `span` is a dynamic-extent `SkSpan` of element type `T`.
pub unsafe fn parts<T>(span: &SpanBytes) -> (*const T, usize) {
    let span = unsafe { &*(span as *const SpanBytes as *const Span<T>) };
    (span.ptr, span.size)
}
