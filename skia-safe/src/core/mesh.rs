//! Custom meshes: a [`MeshSpecification`] with SkSL vertex and fragment programs, and [`Mesh`]es
//! that draw vertex (and index) buffers with it.

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
    /// The CPU representation and the shader type of a vertex attribute.
    ///
    /// Variants:
    /// - [`Type::Float`] CPU representation float, shader type `float`.
    /// - [`Type::Float2`] CPU representation two floats, shader type `float2`.
    /// - [`Type::Float3`] CPU representation three floats, shader type `float3`.
    /// - [`Type::Float4`] CPU representation four floats, shader type `float4`.
    /// - [`Type::UByte4_unorm`] CPU representation four bytes, shader type `half4`.
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
    /// The shader type of a varying.
    ///
    /// Variants:
    /// - [`Type::Float`] `float`
    /// - [`Type::Float2`] `float2`
    /// - [`Type::Float3`] `float3`
    /// - [`Type::Float4`] `float4`
    /// - [`Type::Half`] `half`
    /// - [`Type::Half2`] `half2`
    /// - [`Type::Half3`] `half3`
    /// - [`Type::Half4`] `half4`
    pub use skia_bindings::SkMeshSpecification_Varying_Type as Type;
    variant_name!(Type::Half4);
}

/// A specification for custom meshes. Specifies the vertex buffer attributes and stride, the
/// vertex program that produces a user-defined set of varyings, and a fragment program that ingests
/// the interpolated varyings and produces local coordinates for shading and optionally a color.
///
/// The varyings must include a `float2` named `position`. If the passed varyings does not
/// contain such a varying then one is implicitly added to the final specification and the SkSL
/// `Varyings` struct described below. It is an error to have a varying named `position` that has a
/// type other than `float2`.
///
/// The provided attributes and varyings are used to create `Attributes` and `Varyings` structs in
/// SkSL that are used by the shaders. Each attribute from the [`Attribute`] slice becomes a member
/// of the SkSL `Attributes` struct and likewise for the varyings.
///
/// The signature of the vertex program must be:
/// `Varyings main(const Attributes)`.
///
/// The signature of the fragment program must be either:
/// `float2 main(const Varyings)`
/// or
/// `float2 main(const Varyings, out (half4|float4) color)`
///
/// where the return value is the local coordinates that will be used to access [`crate::Shader`].
/// If the color variant is used, the returned color will be blended with [`crate::Paint`]'s
/// [`crate::Shader`] (or [`crate::Paint`] color in absence of a [`crate::Shader`]) using the
/// [`crate::Blender`] passed to [`crate::Canvas::draw_mesh()`]. To use interpolated local space
/// positions as the shader coordinates, equivalent to how [`crate::Path`]s are shaded, return the
/// position field from the `Varying` struct as the coordinates.
///
/// The vertex and fragment programs may both contain uniforms. Uniforms with the same name are
/// assumed to be shared between stages. It is an error to specify uniforms in the vertex and
/// fragment program with the same name but different types, dimensionality, or layouts.
pub type MeshSpecification = RCHandle<SkMeshSpecification>;
// Immutable after construction (only `const` fields) with an atomic refcount; Skia's Graphite
// shares specifications between recorder threads.
unsafe_send_sync!(MeshSpecification);

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
            .field("uniform_size", &self.uniform_size())
            .field("uniforms", &self.uniforms())
            .field("children", &self.children())
            .field("stride", &self.stride())
            .finish()
    }
}

impl MeshSpecification {
    /// The maximum vertex stride. Enforced when creating a specification.
    pub const MAX_STRIDE: usize = sb::SkMeshSpecification_kMaxStride;
    /// The maximum number of attributes. Enforced when creating a specification.
    pub const MAX_ATTRIBUTES: usize = sb::SkMeshSpecification_kMaxAttributes;
    /// The alignment of the vertex stride. Enforced when creating a specification.
    pub const STRIDE_ALIGNMENT: usize = sb::SkMeshSpecification_kStrideAlignment;
    /// The alignment of attribute offsets. Enforced when creating a specification.
    pub const OFFSET_ALIGNMENT: usize = sb::SkMeshSpecification_kOffsetAlignment;
    /// The maximum number of varyings. Enforced when creating a specification.
    pub const MAX_VARYINGS: usize = sb::SkMeshSpecification_kMaxVaryings;

    /// If successful the return is a specification. Otherwise, it is a non-empty error string.
    ///
    /// The color produced by `fs` (if any) is sRGB and premultiplied.
    ///
    /// - `attributes` The vertex attributes that will be consumed by `vs`. Attributes need not be
    ///   tightly packed but attribute offsets must be aligned to [`Self::OFFSET_ALIGNMENT`] and
    ///   offset + size may not be greater than `vertex_stride`. At least one attribute is required.
    /// - `vertex_stride` The offset between successive attribute values. This must be aligned to
    ///   [`Self::STRIDE_ALIGNMENT`].
    /// - `varyings` The varyings that will be written by `vs` and read by `fs`. This may be empty.
    /// - `vs` The vertex shader code that computes a vertex position and the varyings from the
    ///   attributes.
    /// - `fs` The fragment code that computes a local coordinate and optionally a color from the
    ///   varyings. The local coordinate is used to sample [`crate::Shader`].
    pub fn make(
        attributes: &[Attribute],
        vertex_stride: usize,
        varyings: &[Varying],
        vs: impl AsRef<str>,
        fs: impl AsRef<str>,
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
                error.native_mut(),
            )
        })
        .ok_or_else(|| error.to_string())
    }

    /// Like [`Self::make()`], with the color space and alpha type of the color produced by `fs`.
    ///
    /// - `color_space` The colorspace of the color produced by `fs`. Ignored if `fs`'s main()
    ///   function does not have a color out param. If it has one, `None` is an error.
    /// - `alpha_type` The alpha type of the color produced by `fs`. Ignored if `fs`'s main()
    ///   function does not have a color out param. Cannot be [`AlphaType::Unknown`]. `None` uses
    ///   [`AlphaType::Premul`].
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
        let alpha_type = alpha_type.into();
        let mut error = interop::String::default();
        Self::from_ptr(unsafe {
            sb::C_SkMeshSpecification_MakeWithColorSpace(
                attributes.as_ptr() as _,
                attributes.len(),
                vertex_stride,
                varyings.as_ptr() as _,
                varyings.len(),
                vs.native(),
                fs.native(),
                color_space.into().into_ptr_or_null(),
                alpha_type.as_ref().map_or(ptr::null(), |at| at as *const _),
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

    /// Combined size of all 'uniform' variables. When creating a [`Mesh`] with this specification
    /// provide a [`Data`] of this size, containing values for all of those variables. Use
    /// [`Self::uniforms()`] to get the offset of each uniform within the [`Data`].
    pub fn uniform_size(&self) -> usize {
        unsafe { sb::C_SkMeshSpecification_uniformSize(self.native()) }
    }

    /// Provides info about individual uniforms including the offset into a [`Data`] where each
    /// uniform value should be placed.
    pub fn uniforms(&self) -> &[Uniform] {
        unsafe {
            let mut count = 0;
            let ptr = sb::C_SkMeshSpecification_uniforms(self.native(), &mut count);
            safer::from_raw_parts(Uniform::from_native_ptr(ptr), count)
        }
    }

    /// Provides basic info about individual children: names, indices and runtime effect type.
    pub fn children(&self) -> &[Child] {
        unsafe {
            let mut count = 0;
            let ptr = sb::C_SkMeshSpecification_children(self.native(), &mut count);
            safer::from_raw_parts(Child::from_native_ptr(ptr), count)
        }
    }

    /// Returns the named child's description, or `None` if not found.
    pub fn find_child(&self, name: impl AsRef<str>) -> Option<&Child> {
        let name = name.as_ref().as_bytes();
        unsafe {
            sb::C_SkMeshSpecification_findChild(self.native(), name.as_ptr() as _, name.len())
        }
        .into_non_null()
        .map(|ptr| Child::from_native_ref(unsafe { ptr.as_ref() }))
    }

    /// Returns the named uniform variable's description, or `None` if not found.
    pub fn find_uniform(&self, name: impl AsRef<str>) -> Option<&Uniform> {
        let name = name.as_ref().as_bytes();
        unsafe {
            sb::C_SkMeshSpecification_findUniform(self.native(), name.as_ptr() as _, name.len())
        }
        .into_non_null()
        .map(|ptr| Uniform::from_native_ref(unsafe { ptr.as_ref() }))
    }

    /// Returns the named attribute, or `None` if not found.
    pub fn find_attribute(&self, name: impl AsRef<str>) -> Option<&Attribute> {
        let name = name.as_ref().as_bytes();
        unsafe {
            sb::C_SkMeshSpecification_findAttribute(self.native(), name.as_ptr() as _, name.len())
        }
        .into_non_null()
        .map(|ptr| Attribute::from_native_ref(unsafe { ptr.as_ref() }))
    }

    /// Returns the named varying, or `None` if not found.
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

    /// Modifies the data in the [`IndexBuffer`] by copying `data` into the buffer at `offset`.
    /// Fails if `offset + data.len() > self.size()` or if either `offset` or `data.len()` is not
    /// aligned to 4 bytes. The [`gpu::DirectContext`] must match that used to create the buffer.
    /// We take it as a parameter to emphasize that the context must be used to update the data
    /// and thus the context must be valid for the current thread.
    ///
    /// A CPU-backed buffer is updated with `None` and fails if a context is passed.
    ///
    /// The buffer is shared: every [`Mesh`] and clone that holds it sees the update.
    pub fn update<'a>(
        &self,
        context: impl Into<Option<&'a mut gpu::DirectContext>>,
        data: &[u8],
        offset: usize,
    ) -> bool {
        unsafe {
            sb::C_SkMesh_IndexBuffer_update(
                self.native_mut_force(),
                context.into().native_ptr_or_null_mut(),
                data.as_ptr() as _,
                offset,
                data.len(),
            )
        }
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

    /// Modifies the data in the [`VertexBuffer`] by copying `data` into the buffer at `offset`.
    /// Fails if `offset + data.len() > self.size()` or if either `offset` or `data.len()` is not
    /// aligned to 4 bytes. The [`gpu::DirectContext`] must match that used to create the buffer.
    /// We take it as a parameter to emphasize that the context must be used to update the data
    /// and thus the context must be valid for the current thread.
    ///
    /// A CPU-backed buffer is updated with `None` and fails if a context is passed.
    ///
    /// The buffer is shared: every [`Mesh`] and clone that holds it sees the update.
    pub fn update<'a>(
        &self,
        context: impl Into<Option<&'a mut gpu::DirectContext>>,
        data: &[u8],
        offset: usize,
    ) -> bool {
        unsafe {
            sb::C_SkMesh_VertexBuffer_update(
                self.native_mut_force(),
                context.into().native_ptr_or_null_mut(),
                data.as_ptr() as _,
                offset,
                data.len(),
            )
        }
    }
}

pub use sb::SkMesh_Mode as Mode;
variant_name!(Mode::TriangleStrip);

/// A vertex buffer, a topology, optionally an index buffer, and a compatible
/// [`MeshSpecification`].
///
/// The data in the vertex buffer is expected to contain the attributes described by the spec
/// for `vertex_count` vertices, beginning at `vertex_offset`. `vertex_offset` must be aligned to
/// the [`MeshSpecification`]'s vertex stride. The size of the buffer must be at least
/// `vertex_offset + spec.stride() * vertex_count` (even if vertex attributes contains pad at the
/// end of the stride). If the specified bounds do not contain all the points output by the spec's
/// vertex program when applied to the vertices in the custom mesh, then the result is undefined.
///
/// [`Mesh::make_indexed()`] may be used to create an indexed mesh. `index_count` indices are read
/// from the index buffer at the specified offset, which must be aligned to 2. The indices are
/// always unsigned 16-bit integers. The index count must be at least 3.
///
/// If [`Mesh::make()`] is used, the implicit index sequence is 0, 1, 2, 3, ... and
/// `vertex_count` must be at least 3.
///
/// Both [`Mesh::make()`] and [`Mesh::make_indexed()`] take a [`Data`] with the uniform values.
/// See [`MeshSpecification::uniform_size()`] and [`MeshSpecification::uniforms()`] for sizing and
/// packing uniforms into the [`Data`].
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
            .field("vertex_offset", &self.vertex_offset())
            .field("vertex_count", &self.vertex_count())
            .field("index_offset", &self.index_offset())
            .field("index_count", &self.index_count())
            .field("bounds", &self.bounds())
            .finish()
    }
}

impl Mesh {
    /// Creates a non-indexed [`Mesh`]. If the mesh is invalid the returned string give contain
    /// the reason for the failure (e.g. the vertex buffer was null or uniform data too small).
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

    /// Creates an indexed [`Mesh`]. If the mesh is invalid the returned string give contain the
    /// reason for the failure (e.g. the index buffer was null or uniform data too small).
    ///
    /// `index_count` (at least 3) unsigned 16-bit indices are read from `index_buffer` at the byte
    /// offset `index_offset`, which must be a multiple of 2. See [`Self::make()`] for the other
    /// parameters.
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

    pub fn children(&self) -> impl Iterator<Item = ChildPtr> + use<'_> {
        let children = unsafe {
            let mut count = 0;
            let ptr = sb::C_SkMesh_children(self.native(), &mut count);
            safer::from_raw_parts(ptr, count)
        };
        children.iter().map(|child| {
            // Meshes are only made from `ChildPtr`s, which always hold a child.
            ChildPtr::from_native_ref(child).expect("mesh child is null")
        })
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
    use std::ptr;

    use super::{IndexBuffer, VertexBuffer};
    use crate::prelude::*;
    use skia_bindings as sb;

    /// Makes a CPU-backed index buffer to be used with [`super::Mesh`]es.
    ///
    /// - `data` The data used to populate the buffer (native endian unsigned 16-bit indices). Its
    ///   length is both the size of the data and the size of the resulting buffer, in bytes.
    pub fn make_index_buffer(data: &[u8]) -> Option<IndexBuffer> {
        IndexBuffer::from_ptr(unsafe {
            sb::C_SkMeshes_MakeIndexBuffer(data.as_ptr() as _, data.len())
        })
    }

    /// Makes a zero-initialized CPU-backed index buffer to be used with [`super::Mesh`]es.
    ///
    /// - `size` The size of the resulting buffer, in bytes.
    pub fn make_zero_initialized_index_buffer(size: usize) -> Option<IndexBuffer> {
        IndexBuffer::from_ptr(unsafe { sb::C_SkMeshes_MakeIndexBuffer(ptr::null(), size) })
    }

    /// Makes a copy of an index buffer. The copy will be CPU-backed.
    ///
    /// Returns `None` if the contents of `src` can not be read back (GPU-backed buffers).
    pub fn copy_index_buffer(src: &IndexBuffer) -> Option<IndexBuffer> {
        IndexBuffer::from_ptr(unsafe { sb::C_SkMeshes_CopyIndexBuffer(src.native()) })
    }

    /// Makes a CPU-backed vertex buffer to be used with [`super::Mesh`]es.
    ///
    /// - `data` The data used to populate the buffer. Its length is both the size of the data and
    ///   the size of the resulting buffer, in bytes.
    pub fn make_vertex_buffer(data: &[u8]) -> Option<VertexBuffer> {
        VertexBuffer::from_ptr(unsafe {
            sb::C_SkMeshes_MakeVertexBuffer(data.as_ptr() as _, data.len())
        })
    }

    /// Makes a zero-initialized CPU-backed vertex buffer to be used with [`super::Mesh`]es.
    ///
    /// - `size` The size of the resulting buffer, in bytes.
    pub fn make_zero_initialized_vertex_buffer(size: usize) -> Option<VertexBuffer> {
        VertexBuffer::from_ptr(unsafe { sb::C_SkMeshes_MakeVertexBuffer(ptr::null(), size) })
    }

    /// Makes a copy of a vertex buffer. The copy will be CPU-backed.
    ///
    /// Returns `None` if the contents of `src` can not be read back (GPU-backed buffers).
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

        // The second element pins the stride of the attribute slice across FFI.
        assert_eq!(spec.attributes()[1].name(), "color");
        assert_eq!(spec.attributes()[1].offset(), 8);
        assert_eq!(
            spec.find_varying("color").unwrap().ty(),
            varying::Type::Half4
        );
        assert_eq!(spec.find_uniform("tint").unwrap().offset(), 0);
        // `make()` relies on Skia's default color space.
        assert!(spec.color_space().unwrap().is_srgb());
    }

    #[test]
    fn specification_errors_are_returned() {
        let attributes = [Attribute::new(attribute::Type::Float2, 0, "pos")];

        let error = MeshSpecification::make(&attributes, 8, &[], "not sksl", FS).unwrap_err();
        assert!(!error.is_empty());

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
    fn cloned_mesh_outlives_its_original() {
        let spec = MeshSpecification::make(
            &[Attribute::new(attribute::Type::Float2, 0, "pos")],
            8,
            &[],
            VS,
            FS,
        )
        .unwrap();
        let vb = meshes::make_vertex_buffer(&f32_bytes(&[0.0, 0.0, 10.0, 0.0, 0.0, 10.0])).unwrap();
        let indices: Vec<u8> = [0u16, 1, 2].iter().flat_map(|i| i.to_ne_bytes()).collect();
        let ib = meshes::make_index_buffer(&indices).unwrap();

        let indexed = Mesh::make_indexed(
            &spec,
            Mode::Triangles,
            &vb,
            3,
            0,
            &ib,
            3,
            0,
            None,
            &[],
            Rect::from_wh(10.0, 10.0),
        )
        .unwrap();
        let clone = indexed.clone();
        drop(indexed);
        assert_eq!(clone.index_buffer().unwrap().size(), 6);
    }

    #[test]
    fn children_outlive_their_mesh() {
        let spec = MeshSpecification::make(
            &[Attribute::new(attribute::Type::Float2, 0, "pos")],
            8,
            &[],
            VS,
            "uniform shader child;
            float2 main(const Varyings v, out half4 color) {
                color = child.eval(v.position); return v.position; }",
        )
        .unwrap();
        let vb = meshes::make_vertex_buffer(&f32_bytes(&[0.0, 0.0, 10.0, 0.0, 0.0, 10.0])).unwrap();
        let mesh = Mesh::make(
            &spec,
            Mode::Triangles,
            &vb,
            3,
            0,
            None,
            &[ChildPtr::from(crate::shaders::color(crate::Color::RED))],
            Rect::from_wh(10.0, 10.0),
        )
        .unwrap();

        let children: Vec<_> = mesh.children().collect();
        drop(mesh);
        let [ChildPtr::Shader(shader)] = children.as_slice() else {
            panic!("expected one shader child, got {children:?}");
        };
        assert!(shader.is_opaque());
    }

    #[test]
    fn missing_uniforms_are_an_error() {
        let spec = MeshSpecification::make(
            &[Attribute::new(attribute::Type::Float2, 0, "pos")],
            8,
            &[],
            VS,
            "uniform half4 tint;
            float2 main(const Varyings v, out half4 color) {
                color = tint; return v.position; }",
        )
        .unwrap();
        let vb = meshes::make_vertex_buffer(&f32_bytes(&[0.0, 0.0, 10.0, 0.0, 0.0, 10.0])).unwrap();

        // Skia dereferences null uniforms while it formats this error, see `C_SkMesh_Make`.
        let error = Mesh::make(
            &spec,
            Mode::Triangles,
            &vb,
            3,
            0,
            None,
            &[],
            Rect::from_wh(10.0, 10.0),
        )
        .unwrap_err();
        assert!(error.contains("uniform data"), "{error}");
    }

    fn f32_bytes(values: &[f32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_ne_bytes()).collect()
    }
}
