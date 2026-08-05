use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};

fn normalized_layout(size: usize, align: usize) -> Option<Layout> {
    let size = size.max(1);
    let align = align.max(1);
    Layout::from_size_align(size, align).ok()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn skia_bindings_alloc(size: usize, align: usize) -> *mut u8 {
    let Some(layout) = normalized_layout(size, align) else {
        return std::ptr::null_mut();
    };
    unsafe { alloc(layout) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn skia_bindings_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    let Some(layout) = normalized_layout(size, align) else {
        return std::ptr::null_mut();
    };
    unsafe { alloc_zeroed(layout) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn skia_bindings_dealloc(ptr: *mut u8, size: usize, align: usize) {
    if ptr.is_null() {
        return;
    }
    let Some(layout) = normalized_layout(size, align) else {
        return;
    };
    unsafe { dealloc(ptr, layout) }
}
