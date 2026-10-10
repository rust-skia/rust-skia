use std::ptr;

use skia_bindings as sb;

use crate::{
    gpu::DirectContext,
    mesh::{IndexBuffer, VertexBuffer},
    prelude::*,
};

/// Makes a GPU-backed index buffer to be used with [`crate::Mesh`]es.
///
/// - `context` The data will be uploaded to the corresponding GPU and the returned buffer will
///   only be compatible with surfaces using the same context.
/// - `data` The data used to populate the buffer (native endian unsigned 16-bit indices). Its
///   length is both the size of the data and the size of the resulting buffer.
pub fn make_index_buffer(context: &mut DirectContext, data: &[u8]) -> Option<IndexBuffer> {
    IndexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_MakeIndexBufferGanesh(context.native_mut(), data.as_ptr() as _, data.len())
    })
}

/// Makes a zero-initialized GPU-backed index buffer to be used with [`crate::Mesh`]es.
///
/// - `context` The buffer will be created on the corresponding GPU and will only be compatible with
///   surfaces using the same context.
/// - `size` The size of the resulting buffer, in bytes.
pub fn make_zero_initialized_index_buffer(
    context: &mut DirectContext,
    size: usize,
) -> Option<IndexBuffer> {
    IndexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_MakeIndexBufferGanesh(context.native_mut(), ptr::null(), size)
    })
}

/// Makes a copy of an index buffer. The copy will be GPU backed.
///
/// Returns `None` if the contents of `src` can not be read back (GPU-backed buffers).
pub fn copy_index_buffer(context: &mut DirectContext, src: &IndexBuffer) -> Option<IndexBuffer> {
    IndexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_CopyIndexBufferGanesh(context.native_mut(), src.native())
    })
}

/// Makes a GPU-backed vertex buffer to be used with [`crate::Mesh`]es.
///
/// - `context` The data will be uploaded to the corresponding GPU and the returned buffer will
///   only be compatible with surfaces using the same context.
/// - `data` The data used to populate the buffer. Its length is both the size of the data and the
///   size of the resulting buffer.
pub fn make_vertex_buffer(context: &mut DirectContext, data: &[u8]) -> Option<VertexBuffer> {
    VertexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_MakeVertexBufferGanesh(context.native_mut(), data.as_ptr() as _, data.len())
    })
}

/// Makes a zero-initialized GPU-backed vertex buffer to be used with [`crate::Mesh`]es.
///
/// - `context` The buffer will be created on the corresponding GPU and will only be compatible with
///   surfaces using the same context.
/// - `size` The size of the resulting buffer, in bytes.
pub fn make_zero_initialized_vertex_buffer(
    context: &mut DirectContext,
    size: usize,
) -> Option<VertexBuffer> {
    VertexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_MakeVertexBufferGanesh(context.native_mut(), ptr::null(), size)
    })
}

/// Makes a copy of a vertex buffer. The copy will be GPU backed.
///
/// Returns `None` if the contents of `src` can not be read back (GPU-backed buffers).
pub fn copy_vertex_buffer(context: &mut DirectContext, src: &VertexBuffer) -> Option<VertexBuffer> {
    VertexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_CopyVertexBufferGanesh(context.native_mut(), src.native())
    })
}
