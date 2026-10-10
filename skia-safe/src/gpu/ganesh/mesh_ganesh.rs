use skia_bindings as sb;

use crate::{
    gpu::DirectContext,
    mesh::{IndexBuffer, VertexBuffer},
    prelude::*,
};

/// Makes a GPU-backed index buffer by uploading `data` (native endian unsigned 16-bit indices).
/// The buffer is only compatible with surfaces using the same `context`.
pub fn make_index_buffer(context: &mut DirectContext, data: &[u8]) -> Option<IndexBuffer> {
    IndexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_MakeIndexBufferGanesh(context.native_mut(), data.as_ptr() as _, data.len())
    })
}

/// Makes a GPU-backed copy of an index buffer. Returns `None` if the contents of `src` can not be
/// read back (GPU-backed buffers).
pub fn copy_index_buffer(context: &mut DirectContext, src: &IndexBuffer) -> Option<IndexBuffer> {
    IndexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_CopyIndexBufferGanesh(context.native_mut(), src.native())
    })
}

/// Makes a GPU-backed vertex buffer by uploading `data`. The buffer is only compatible with
/// surfaces using the same `context`.
pub fn make_vertex_buffer(context: &mut DirectContext, data: &[u8]) -> Option<VertexBuffer> {
    VertexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_MakeVertexBufferGanesh(context.native_mut(), data.as_ptr() as _, data.len())
    })
}

/// Makes a GPU-backed copy of a vertex buffer. Returns `None` if the contents of `src` can not be
/// read back (GPU-backed buffers).
pub fn copy_vertex_buffer(context: &mut DirectContext, src: &VertexBuffer) -> Option<VertexBuffer> {
    VertexBuffer::from_ptr(unsafe {
        sb::C_SkMeshes_CopyVertexBufferGanesh(context.native_mut(), src.native())
    })
}
