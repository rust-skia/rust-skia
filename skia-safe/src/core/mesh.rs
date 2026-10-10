use std::{fmt, ops::DerefMut, ptr};

use skia_bindings::{
    self as sb, SkMesh, SkMesh_IndexBuffer, SkMesh_VertexBuffer, SkMeshSpecification,
    SkMeshSpecification_Attribute, SkMeshSpecification_Varying, SkRefCntBase,
};

use crate::{
    AlphaType, ColorSpace, Data, Rect, gpu,
    interop::{self, AsStr},
    prelude::*,
};

pub use crate::runtime_effect::{Child, ChildPtr, Uniform};

pub use sb::SkMesh_Mode as Mode;
variant_name!(Mode::TriangleStrip);

/// A vertex attribute: its type, its byte offset inside a vertex, and the name of the field in the
/// SkSL `Attributes` struct.
pub type Attribute = Handle<SkMeshSpecification_Attribute>;
unsafe_send_sync!(Attribute);

impl NativeDrop for SkMeshSpecification_Attribute {
    fn drop(&mut self) {
        unsafe { sb::C_SkMeshSpecification_Attribute_destruct(self) }
    }
}

impl NativeClone for SkMeshSpecification_Attribute {
    fn clone(&self) -> Self {
        construct(|a| unsafe { sb::C_SkMeshSpecification_Attribute_CopyConstruct(a, self) })
    }
}

impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attribute")
            .field("type", &self.ty())
            .field("offset", &self.offset())
            .field("name", &self.name())
            .finish()
    }
}

impl Attribute {
    pub fn new(ty: attribute::Type, offset: usize, name: impl AsRef<str>) -> Self {
        let name = name.as_ref().as_bytes();
        Self::construct(|a| unsafe {
            sb::C_SkMeshSpecification_Attribute_Construct(
                a,
                ty,
                offset,
                name.as_ptr() as _,
                name.len(),
            )
        })
    }

    pub fn ty(&self) -> attribute::Type {
        self.native().type_
    }

    pub fn offset(&self) -> usize {
        self.native().offset
    }

    pub fn name(&self) -> &str {
        self.native().name.as_str()
    }
}

pub mod attribute {
    /// CPU representation and shader type of a vertex attribute. `UByte4_unorm` is four bytes on
    /// the CPU and a `half4` in the shader.
    pub use skia_bindings::SkMeshSpecification_Attribute_Type as Type;
    variant_name!(Type::UByte4_unorm);
}

/// A value written by the vertex program and read, interpolated, by the fragment program.
pub type Varying = Handle<SkMeshSpecification_Varying>;
unsafe_send_sync!(Varying);

impl NativeDrop for SkMeshSpecification_Varying {
    fn drop(&mut self) {
        unsafe { sb::C_SkMeshSpecification_Varying_destruct(self) }
    }
}

impl NativeClone for SkMeshSpecification_Varying {
    fn clone(&self) -> Self {
        construct(|v| unsafe { sb::C_SkMeshSpecification_Varying_CopyConstruct(v, self) })
    }
}

impl fmt::Debug for Varying {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Varying")
            .field("type", &self.ty())
            .field("name", &self.name())
            .finish()
    }
}

impl Varying {
    pub fn new(ty: varying::Type, name: impl AsRef<str>) -> Self {
        let name = name.as_ref().as_bytes();
        Self::construct(|v| unsafe {
            sb::C_SkMeshSpecification_Varying_Construct(v, ty, name.as_ptr() as _, name.len())
        })
    }

    pub fn ty(&self) -> varying::Type {
        self.native().type_
    }

    pub fn name(&self) -> &str {
        self.native().name.as_str()
    }
}

pub mod varying {
    pub use skia_bindings::SkMeshSpecification_Varying_Type as Type;
    variant_name!(Type::Half4);
}

/// A specification for custom meshes. Specifies the vertex buffer attributes and stride, the
/// vertex program that produces a user-defined set of varyings, and a fragment program that
/// ingests the interpolated varyings and produces local coordinates for shading and optionally a
/// color.
///
/// The varyings must include a `float2` named `position`. If the passed varyings do not contain
/// such a varying, one is implicitly added.
///
/// The signature of the vertex program must be `Varyings main(const Attributes)`.
///
/// The signature of the fragment program must be either `float2 main(const Varyings)` or
/// `float2 main(const Varyings, out (half4|float4) color)`, where the return value is the local
/// coordinates that will be used to access the [`crate::Shader`] of the [`crate::Paint`]. If the
/// color variant is used, the returned color is blended with the paint's shader (or the paint
/// color) using the blender passed to [`crate::Canvas::draw_mesh()`].
pub type MeshSpecification = RCHandle<SkMeshSpecification>;

impl NativeRefCounted for SkMeshSpecification {
    fn _ref(&self) {
        unsafe { sb::C_SkMeshSpecification_ref(self) }
    }

    fn _unref(&self) {
        unsafe { sb::C_SkMeshSpecification_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { sb::C_SkMeshSpecification_unique(self) }
    }
}

impl fmt::Debug for MeshSpecification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MeshSpecification")
            .field("attributes", &self.attributes())
            .field("stride", &self.stride())
            .field("uniform_size", &self.uniform_size())
            .field("uniforms", &self.uniforms())
            .field("children", &self.children())
            .finish()
    }
}

impl MeshSpecification {
    /// Creates a specification whose fragment program color (if any) is sRGB and premultiplied.
    ///
    /// Returns the SkSL / validation error text on failure.
    ///
    /// - `attributes` the vertex attributes consumed by `vs`. At least one is required, offsets
    ///   must be 4 byte aligned.
    /// - `vertex_stride` the byte distance between successive vertices, 4 byte aligned.
    /// - `varyings` the varyings written by `vs` and read by `fs`. May be empty.
    /// - `vs` the vertex program.
    /// - `fs` the fragment program.
    pub fn make(
        attributes: &[Attribute],
        vertex_stride: usize,
        varyings: &[Varying],
        vs: impl AsRef<str>,
        fs: impl AsRef<str>,
    ) -> Result<Self, String> {
        Self::make_with_color_space(
            attributes,
            vertex_stride,
            varyings,
            vs,
            fs,
            ColorSpace::new_srgb(),
            None,
        )
    }

    /// Like [`Self::make()`], with the color space and alpha type of the color produced by `fs`.
    ///
    /// Both are ignored if `fs` has no color out parameter. If it has one, `color_space` must not
    /// be `None`. `alpha_type` defaults to [`AlphaType::Premul`].
    pub fn make_with_color_space(
        attributes: &[Attribute],
        vertex_stride: usize,
        varyings: &[Varying],
        vs: impl AsRef<str>,
        fs: impl AsRef<str>,
        color_space: impl Into<Option<ColorSpace>>,
        alpha_type: impl Into<Option<AlphaType>>,
    ) -> Result<Self, String> {
        let vs = interop::String::from_str(vs);
        let fs = interop::String::from_str(fs);
        let mut error = interop::String::default();
        Self::from_ptr(unsafe {
            sb::C_SkMeshSpecification_Make(
                attributes.as_ptr() as _,
                attributes.len(),
                vertex_stride,
                varyings.as_ptr() as _,
                varyings.len(),
                vs.native(),
                fs.native(),
                color_space.into().into_ptr_or_null(),
                alpha_type.into().unwrap_or(AlphaType::Premul),
                error.native_mut(),
            )
        })
        .ok_or_else(|| error.to_string())
    }

    pub fn attributes(&self) -> &[Attribute] {
        unsafe {
            let mut count = 0;
            let ptr = sb::C_SkMeshSpecification_attributes(self.native(), &mut count);
            safer::from_raw_parts(Attribute::from_native_ptr(ptr), count)
        }
    }

    /// Combined size of all uniforms. A [`Mesh`] created with this specification needs a [`Data`]
    /// of this size. Use [`Self::uniforms()`] to get the offset of each uniform.
    pub fn uniform_size(&self) -> usize {
        unsafe { sb::C_SkMeshSpecification_uniformSize(self.native()) }
    }

    pub fn uniforms(&self) -> &[Uniform] {
        unsafe {
            let mut count = 0;
            let ptr = sb::C_SkMeshSpecification_uniforms(self.native(), &mut count);
            safer::from_raw_parts(Uniform::from_native_ptr(ptr), count)
        }
    }

    pub fn children(&self) -> &[Child] {
        unsafe {
            let mut count = 0;
            let ptr = sb::C_SkMeshSpecification_children(self.native(), &mut count);
            safer::from_raw_parts(Child::from_native_ptr(ptr), count)
        }
    }

    pub fn find_child(&self, name: impl AsRef<str>) -> Option<&Child> {
        let name = name.as_ref().as_bytes();
        unsafe {
            sb::C_SkMeshSpecification_findChild(self.native(), name.as_ptr() as _, name.len())
        }
        .into_non_null()
        .map(|ptr| Child::from_native_ref(unsafe { ptr.as_ref() }))
    }

    pub fn find_uniform(&self, name: impl AsRef<str>) -> Option<&Uniform> {
        let name = name.as_ref().as_bytes();
        unsafe {
            sb::C_SkMeshSpecification_findUniform(self.native(), name.as_ptr() as _, name.len())
        }
        .into_non_null()
        .map(|ptr| Uniform::from_native_ref(unsafe { ptr.as_ref() }))
    }

    pub fn find_attribute(&self, name: impl AsRef<str>) -> Option<&Attribute> {
        let name = name.as_ref().as_bytes();
        unsafe {
            sb::C_SkMeshSpecification_findAttribute(self.native(), name.as_ptr() as _, name.len())
        }
        .into_non_null()
        .map(|ptr| Attribute::from_native_ref(unsafe { ptr.as_ref() }))
    }

    pub fn find_varying(&self, name: impl AsRef<str>) -> Option<&Varying> {
        let name = name.as_ref().as_bytes();
        unsafe {
            sb::C_SkMeshSpecification_findVarying(self.native(), name.as_ptr() as _, name.len())
        }
        .into_non_null()
        .map(|ptr| Varying::from_native_ref(unsafe { ptr.as_ref() }))
    }

    pub fn stride(&self) -> usize {
        unsafe { sb::C_SkMeshSpecification_stride(self.native()) }
    }

    /// The color space of the color produced by the fragment program, `None` if it produces none.
    pub fn color_space(&self) -> Option<ColorSpace> {
        ColorSpace::from_unshared_ptr(unsafe {
            sb::C_SkMeshSpecification_colorSpace(self.native())
        })
    }
}

/// The vertex data of a [`Mesh`]. Create one with [`meshes::make_vertex_buffer()`] (CPU-backed) or
/// `gpu::meshes::make_vertex_buffer()` (GPU-backed).
pub type VertexBuffer = RCHandle<SkMesh_VertexBuffer>;
require_base_type!(SkMesh_VertexBuffer, sb::SkRefCnt);

impl NativeRefCountedBase for SkMesh_VertexBuffer {
    type Base = SkRefCntBase;
}

impl fmt::Debug for VertexBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VertexBuffer")
            .field("size", &self.size())
            .finish()
    }
}

impl VertexBuffer {
    /// The size of the buffer in bytes.
    pub fn size(&self) -> usize {
        unsafe { sb::C_SkMesh_VertexBuffer_size(self.native()) }
    }

    /// Copies `data` into the buffer at the byte `offset`. Fails if `offset + data.len() >
    /// self.size()` or if either `offset` or `data.len()` is not aligned to 4 bytes.
    ///
    /// A GPU-backed buffer needs the `context` it was created with, a CPU-backed buffer ignores
    /// it.
    pub fn update<'a>(
        &mut self,
        context: impl Into<Option<&'a mut gpu::DirectContext>>,
        data: &[u8],
        offset: usize,
    ) -> bool {
        unsafe {
            sb::C_SkMesh_VertexBuffer_update(
                self.native_mut(),
                context.into().native_ptr_or_null_mut(),
                data.as_ptr() as _,
                offset,
                data.len(),
            )
        }
    }
}

/// The index data of an indexed [`Mesh`]: unsigned 16-bit indices. Create one with
/// [`meshes::make_index_buffer()`] (CPU-backed) or `gpu::meshes::make_index_buffer()`
/// (GPU-backed).
pub type IndexBuffer = RCHandle<SkMesh_IndexBuffer>;
require_base_type!(SkMesh_IndexBuffer, sb::SkRefCnt);

impl NativeRefCountedBase for SkMesh_IndexBuffer {
    type Base = SkRefCntBase;
}

impl fmt::Debug for IndexBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IndexBuffer")
            .field("size", &self.size())
            .finish()
    }
}

impl IndexBuffer {
    /// The size of the buffer in bytes.
    pub fn size(&self) -> usize {
        unsafe { sb::C_SkMesh_IndexBuffer_size(self.native()) }
    }

    /// Copies `data` into the buffer at the byte `offset`. Fails if `offset + data.len() >
    /// self.size()` or if either `offset` or `data.len()` is not aligned to 4 bytes.
    ///
    /// A GPU-backed buffer needs the `context` it was created with, a CPU-backed buffer ignores
    /// it.
    pub fn update<'a>(
        &mut self,
        context: impl Into<Option<&'a mut gpu::DirectContext>>,
        data: &[u8],
        offset: usize,
    ) -> bool {
        unsafe {
            sb::C_SkMesh_IndexBuffer_update(
                self.native_mut(),
                context.into().native_ptr_or_null_mut(),
                data.as_ptr() as _,
                offset,
                data.len(),
            )
        }
    }
}

/// A vertex buffer, a topology, optionally an index buffer, and a compatible
/// [`MeshSpecification`]. Draw it with [`crate::Canvas::draw_mesh()`].
///
/// Note: In this Skia version, meshes are drawn by the Ganesh GPU backend only. The raster, PDF
/// and SVG backends ignore them.
// Heap allocated: `SkMesh` stores its children in an array with inline storage and can not be
// moved by Rust.
pub type Mesh = RefHandle<SkMesh>;

impl NativeDrop for SkMesh {
    fn drop(&mut self) {
        unsafe { sb::C_SkMesh_delete(self) }
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Self {
        Self::from_ptr(unsafe { sb::C_SkMesh_clone(self.native()) }).unwrap()
    }
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mesh")
            .field("mode", &self.mode())
            .field("vertex_count", &self.vertex_count())
            .field("vertex_offset", &self.vertex_offset())
            .field("index_count", &self.index_count())
            .field("index_offset", &self.index_offset())
            .field("bounds", &self.bounds())
            .finish()
    }
}

impl Mesh {
    /// Creates a non-indexed mesh. Returns the reason if the mesh is invalid (for example, the
    /// uniform data is too small).
    ///
    /// - `vertex_count` must be at least 3.
    /// - `vertex_offset` the byte offset of the first vertex in `vertex_buffer`, a multiple of the
    ///   specification's stride.
    /// - `uniforms` the uniform values, see [`MeshSpecification::uniform_size()`] and
    ///   [`MeshSpecification::uniforms()`].
    /// - `children` one entry per [`MeshSpecification::children()`].
    /// - `bounds` must contain all positions output by the vertex program, otherwise the result
    ///   is undefined.
    #[allow(clippy::too_many_arguments)]
    pub fn make(
        spec: impl Into<MeshSpecification>,
        mode: Mode,
        vertex_buffer: impl Into<VertexBuffer>,
        vertex_count: usize,
        vertex_offset: usize,
        uniforms: impl Into<Option<Data>>,
        children: &[ChildPtr],
        bounds: impl AsRef<Rect>,
    ) -> Result<Self, String> {
        Self::make_native(
            spec.into(),
            mode,
            vertex_buffer.into(),
            vertex_count,
            vertex_offset,
            None,
            uniforms.into(),
            children,
            bounds.as_ref(),
        )
    }

    /// Creates an indexed mesh. `index_count` (at least 3) unsigned 16-bit indices are read from
    /// `index_buffer` at the byte offset `index_offset`, which must be a multiple of 2.
    ///
    /// See [`Self::make()`] for the other parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn make_indexed(
        spec: impl Into<MeshSpecification>,
        mode: Mode,
        vertex_buffer: impl Into<VertexBuffer>,
        vertex_count: usize,
        vertex_offset: usize,
        index_buffer: impl Into<IndexBuffer>,
        index_count: usize,
        index_offset: usize,
        uniforms: impl Into<Option<Data>>,
        children: &[ChildPtr],
        bounds: impl AsRef<Rect>,
    ) -> Result<Self, String> {
        Self::make_native(
            spec.into(),
            mode,
            vertex_buffer.into(),
            vertex_count,
            vertex_offset,
            Some((index_buffer.into(), index_count, index_offset)),
            uniforms.into(),
            children,
            bounds.as_ref(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn make_native(
        spec: MeshSpecification,
        mode: Mode,
        vertex_buffer: VertexBuffer,
        vertex_count: usize,
        vertex_offset: usize,
        indices: Option<(IndexBuffer, usize, usize)>,
        uniforms: Option<Data>,
        children: &[ChildPtr],
        bounds: &Rect,
    ) -> Result<Self, String> {
        let mut children: Vec<_> = children
            .iter()
            .map(|child_ptr| child_ptr.native())
            .collect();
        let children_ptr = children
            .first_mut()
            .map(|c| c.deref_mut() as *mut _)
            .unwrap_or(ptr::null_mut());
        let (index_buffer, index_count, index_offset) = match indices {
            Some((buffer, count, offset)) => (buffer.into_ptr(), count, offset),
            None => (ptr::null_mut(), 0, 0),
        };
        let mut error = interop::String::default();
        Self::from_ptr(unsafe {
            sb::C_SkMesh_Make(
                spec.into_ptr(),
                mode,
                vertex_buffer.into_ptr(),
                vertex_count,
                vertex_offset,
                index_buffer,
                index_count,
                index_offset,
                uniforms.into_ptr_or_null(),
                children_ptr,
                children.len(),
                bounds.native(),
                error.native_mut(),
            )
        })
        .ok_or_else(|| error.to_string())
    }

    pub fn spec(&self) -> MeshSpecification {
        MeshSpecification::from_ptr(unsafe { sb::C_SkMesh_refSpec(self.native()) }).unwrap()
    }

    pub fn mode(&self) -> Mode {
        unsafe { sb::C_SkMesh_mode(self.native()) }
    }

    pub fn vertex_buffer(&self) -> VertexBuffer {
        VertexBuffer::from_ptr(unsafe { sb::C_SkMesh_refVertexBuffer(self.native()) }).unwrap()
    }

    pub fn vertex_offset(&self) -> usize {
        unsafe { sb::C_SkMesh_vertexOffset(self.native()) }
    }

    pub fn vertex_count(&self) -> usize {
        unsafe { sb::C_SkMesh_vertexCount(self.native()) }
    }

    /// The index buffer, `None` if the mesh is not indexed.
    pub fn index_buffer(&self) -> Option<IndexBuffer> {
        IndexBuffer::from_ptr(unsafe { sb::C_SkMesh_refIndexBuffer(self.native()) })
    }

    pub fn index_offset(&self) -> usize {
        unsafe { sb::C_SkMesh_indexOffset(self.native()) }
    }

    pub fn index_count(&self) -> usize {
        unsafe { sb::C_SkMesh_indexCount(self.native()) }
    }

    pub fn uniforms(&self) -> Option<Data> {
        Data::from_ptr_const(unsafe { sb::C_SkMesh_refUniforms(self.native()) })
    }

    pub fn bounds(&self) -> Rect {
        let mut bounds = Rect::default();
        unsafe { sb::C_SkMesh_bounds(self.native(), bounds.native_mut()) };
        bounds
    }

    /// Always `true` for a mesh returned by [`Self::make()`] or [`Self::make_indexed()`]; an
    /// invalid mesh is reported as an error there.
    pub fn is_valid(&self) -> bool {
        unsafe { sb::C_SkMesh_isValid(self.native()) }
    }
}

/// CPU-backed mesh buffers.
pub mod meshes {
    use super::{IndexBuffer, VertexBuffer};
    use crate::prelude::*;
    use skia_bindings as sb;

    /// Makes a CPU-backed index buffer by copying `data` (native endian unsigned 16-bit indices).
    pub fn make_index_buffer(data: &[u8]) -> Option<IndexBuffer> {
        IndexBuffer::from_ptr(unsafe {
            sb::C_SkMeshes_MakeIndexBuffer(data.as_ptr() as _, data.len())
        })
    }

    /// Makes a CPU-backed copy of an index buffer. Returns `None` if the contents of `src` can
    /// not be read back (GPU-backed buffers).
    pub fn copy_index_buffer(src: &IndexBuffer) -> Option<IndexBuffer> {
        IndexBuffer::from_ptr(unsafe { sb::C_SkMeshes_CopyIndexBuffer(src.native()) })
    }

    /// Makes a CPU-backed vertex buffer by copying `data`.
    pub fn make_vertex_buffer(data: &[u8]) -> Option<VertexBuffer> {
        VertexBuffer::from_ptr(unsafe {
            sb::C_SkMeshes_MakeVertexBuffer(data.as_ptr() as _, data.len())
        })
    }

    /// Makes a CPU-backed copy of a vertex buffer. Returns `None` if the contents of `src` can
    /// not be read back (GPU-backed buffers).
    pub fn copy_vertex_buffer(src: &VertexBuffer) -> Option<VertexBuffer> {
        VertexBuffer::from_ptr(unsafe { sb::C_SkMeshes_CopyVertexBuffer(src.native()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VS: &str =
        "Varyings main(const Attributes a) { Varyings v; v.position = a.pos; return v; }";
    const FS: &str = "float2 main(const Varyings v) { return v.position; }";

    fn f32_bytes(values: &[f32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_ne_bytes()).collect()
    }

    #[test]
    fn specification_reflects_attributes_varyings_and_uniforms() {
        let fs = "uniform half4 tint;
            float2 main(const Varyings v, out half4 color) {
                color = v.color * tint; return v.position; }";
        let vs = "Varyings main(const Attributes a) {
            Varyings v; v.position = a.pos; v.color = a.color; return v; }";
        let spec = MeshSpecification::make(
            &[
                Attribute::new(attribute::Type::Float2, 0, "pos"),
                Attribute::new(attribute::Type::UByte4_unorm, 8, "color"),
            ],
            12,
            &[Varying::new(varying::Type::Half4, "color")],
            vs,
            fs,
        )
        .unwrap();

        assert_eq!(spec.stride(), 12);
        assert_eq!(spec.attributes().len(), 2);
        assert_eq!(spec.attributes()[1].name(), "color");
        assert_eq!(spec.attributes()[1].offset(), 8);
        assert_eq!(
            spec.find_attribute("pos").unwrap().ty(),
            attribute::Type::Float2
        );
        assert_eq!(
            spec.find_varying("color").unwrap().ty(),
            varying::Type::Half4
        );
        // `position` is added implicitly.
        assert_eq!(
            spec.find_varying("position").unwrap().ty(),
            varying::Type::Float2
        );
        assert!(spec.find_varying("nope").is_none());
        assert_eq!(spec.uniform_size(), 16);
        assert_eq!(spec.uniforms().len(), 1);
        assert_eq!(spec.find_uniform("tint").unwrap().offset(), 0);
        assert!(spec.children().is_empty());
        assert!(spec.color_space().unwrap().is_srgb());
    }

    #[test]
    fn specification_errors_are_returned() {
        let attributes = [Attribute::new(attribute::Type::Float2, 0, "pos")];

        let error = MeshSpecification::make(&attributes, 8, &[], "not sksl", FS).unwrap_err();
        assert!(!error.is_empty());

        // Stride is not a multiple of 4.
        assert!(MeshSpecification::make(&attributes, 10, &[], VS, FS).is_err());
        // At least one attribute is required.
        assert!(MeshSpecification::make(&[], 8, &[], VS, FS).is_err());

        let fs_with_color = "float2 main(const Varyings v, out half4 color) {
            color = half4(1); return v.position; }";
        let error = MeshSpecification::make_with_color_space(
            &attributes,
            8,
            &[],
            VS,
            fs_with_color,
            None,
            None,
        )
        .unwrap_err();
        assert!(error.contains("color space"), "{error}");
    }

    #[test]
    fn buffers_meshes_and_their_errors() {
        let spec = MeshSpecification::make(
            &[Attribute::new(attribute::Type::Float2, 0, "pos")],
            8,
            &[],
            VS,
            FS,
        )
        .unwrap();

        let vertices = f32_bytes(&[0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 10.0]);
        let mut vb = meshes::make_vertex_buffer(&vertices).unwrap();
        assert_eq!(vb.size(), 32);
        assert!(vb.update(None, &f32_bytes(&[1.0, 1.0]), 0));
        // Out of bounds and unaligned updates are rejected.
        assert!(!vb.update(None, &f32_bytes(&[1.0, 1.0]), 28));
        assert!(!vb.update(None, &[0, 0], 0));
        assert_eq!(meshes::copy_vertex_buffer(&vb).unwrap().size(), 32);

        let indices: Vec<u8> = [0u16, 1, 2, 2, 1, 3]
            .iter()
            .flat_map(|i| i.to_ne_bytes())
            .collect();
        let ib = meshes::make_index_buffer(&indices).unwrap();
        assert_eq!(ib.size(), 12);

        let bounds = Rect::from_wh(10.0, 10.0);
        let mesh = Mesh::make(&spec, Mode::TriangleStrip, &vb, 4, 0, None, &[], bounds).unwrap();
        assert!(mesh.is_valid());
        assert_eq!(mesh.mode(), Mode::TriangleStrip);
        assert_eq!(mesh.vertex_count(), 4);
        assert_eq!(mesh.bounds(), bounds);
        assert!(mesh.index_buffer().is_none());
        assert_eq!(mesh.spec().stride(), 8);

        let indexed = Mesh::make_indexed(
            &spec,
            Mode::Triangles,
            &vb,
            4,
            0,
            &ib,
            6,
            0,
            None,
            &[],
            bounds,
        )
        .unwrap();
        assert_eq!(indexed.index_count(), 6);
        let clone = indexed.clone();
        drop(indexed);
        assert_eq!(clone.index_buffer().unwrap().size(), 12);

        // More vertices than the buffer holds.
        let error = Mesh::make(&spec, Mode::Triangles, &vb, 6, 0, None, &[], bounds).unwrap_err();
        assert!(!error.is_empty());
    }

    #[test]
    fn missing_uniforms_and_children_are_errors() {
        let spec = MeshSpecification::make(
            &[Attribute::new(attribute::Type::Float2, 0, "pos")],
            8,
            &[],
            VS,
            "uniform shader child; uniform half4 tint;
            float2 main(const Varyings v, out half4 color) {
                color = child.eval(v.position) * tint; return v.position; }",
        )
        .unwrap();
        let vb = meshes::make_vertex_buffer(&f32_bytes(&[0.0, 0.0, 10.0, 0.0, 0.0, 10.0])).unwrap();
        let bounds = Rect::from_wh(10.0, 10.0);
        let children = [ChildPtr::from(crate::shaders::color(crate::Color::RED))];
        let uniforms = Data::new_copy(&f32_bytes(&[1.0; 4]));

        let make = |uniforms: Option<Data>, children: &[ChildPtr]| {
            Mesh::make(
                &spec,
                Mode::Triangles,
                &vb,
                3,
                0,
                uniforms,
                children,
                bounds,
            )
        };

        let error = make(None, &children).unwrap_err();
        assert!(error.contains("uniform data"), "{error}");
        let error = make(uniforms.clone().into(), &[]).unwrap_err();
        assert!(error.contains("child"), "{error}");
        // A color filter where a shader is expected.
        let wrong_child = [ChildPtr::from(crate::color_filters::srgb_to_linear_gamma())];
        let error = make(uniforms.clone().into(), &wrong_child).unwrap_err();
        assert!(error.contains("child"), "{error}");

        let mesh = make(uniforms.into(), &children).unwrap();
        assert_eq!(mesh.uniforms().unwrap().size(), 16);
    }
}
