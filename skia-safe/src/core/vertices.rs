use crate::prelude::*;
use crate::{Color, Point, Rect};
use skia_bindings as sb;
use skia_bindings::{
    SkPoint, SkVertices, SkVertices_Attribute, SkVertices_Attribute_Type, SkVertices_Builder,
};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::{ptr, slice};

#[deprecated(since = "0.29.0", note = "removed without replacement")]
pub type BoneIndices = [u32; 4];

#[deprecated(since = "0.29.0", note = "removed without replacement")]
pub type BoneWeights = [u32; 4];

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
#[deprecated(since = "0.29.0", note = "removed without replacement")]
pub struct Bone {
    values: [f32; 6],
}

pub use skia_bindings::SkVertices_VertexMode as VertexMode;
#[test]
fn test_vertices_vertex_mode_naming() {
    let _ = VertexMode::Triangles;
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AttributeType {
    Float = sb::SkVertices_Attribute_Type::Float as _,
    Float2 = sb::SkVertices_Attribute_Type::Float2 as _,
    Float3 = sb::SkVertices_Attribute_Type::Float3 as _,
    Float4 = sb::SkVertices_Attribute_Type::Float4 as _,
    Byte4UNorm = sb::SkVertices_Attribute_Type::Byte4_unorm as _,
}

impl NativeTransmutable<SkVertices_Attribute_Type> for AttributeType {}
#[test]
fn test_attribute_type_layout() {
    AttributeType::test_layout()
}

pub use skia_bindings::SkVertices_Attribute_Usage as AttributeUsage;
#[test]
fn test_attribute_usage_naming() {
    let _ = AttributeUsage::Vector;
}

#[repr(C)]
#[derive(Copy, Clone, Eq, Debug)]
pub struct Attribute<'a> {
    pub ty: AttributeType,
    pub usage: AttributeUsage,
    pub marker_id: u32,
    marker_name: *const c_char,
    pd: PhantomData<&'a CStr>,
}

impl NativeTransmutable<SkVertices_Attribute> for Attribute<'_> {}
#[test]
fn test_attribute_layout() {
    Attribute::test_layout()
}

impl PartialEq for Attribute<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.usage == other.usage && self.marker_id == other.marker_id
    }
}

impl Default for Attribute<'_> {
    fn default() -> Self {
        Attribute::new(AttributeType::Float)
    }
}

impl Attribute<'_> {
    pub fn new(ty: AttributeType) -> Self {
        Self::new_with_usage_and_marker(ty, None, None)
    }

    pub fn new_with_usage_and_marker<'a>(
        ty: AttributeType,
        usage: impl Into<Option<AttributeUsage>>,
        marker_name: impl Into<Option<&'a CStr>>,
    ) -> Attribute<'a> {
        let marker_name = marker_name
            .into()
            .map(|m| m.as_ptr())
            .unwrap_or(ptr::null());

        Attribute::from_native_c(unsafe {
            SkVertices_Attribute::new(
                ty.into_native(),
                usage.into().unwrap_or(AttributeUsage::Raw),
                marker_name,
            )
        })
    }

    pub fn marker_name(&self) -> Option<&CStr> {
        if !self.marker_name.is_null() {
            unsafe { CStr::from_ptr(self.marker_name) }.into()
        } else {
            None
        }
    }

    pub fn channel_count(self) -> usize {
        unsafe { self.native().channelCount() }.try_into().unwrap()
    }

    pub fn bytes_per_vertex(self) -> usize {
        unsafe { self.native().bytesPerVertex() }
    }

    pub fn is_valid(self) -> bool {
        unsafe { self.native().isValid() }
    }
}

pub type Vertices = RCHandle<SkVertices>;
unsafe impl Send for Vertices {}
unsafe impl Sync for Vertices {}

impl NativeRefCounted for SkVertices {
    fn _ref(&self) {
        unsafe { sb::C_SkVertices_ref(self) }
    }

    fn _unref(&self) {
        unsafe { sb::C_SkVertices_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { sb::C_SkVertices_unique(self) }
    }
}

impl RCHandle<SkVertices> {
    pub fn new_copy(
        mode: VertexMode,
        positions: &[Point],
        texs: &[Point],
        colors: &[Color],
        indices: Option<&[u16]>,
    ) -> Vertices {
        let vertex_count = positions.len();
        assert_eq!(texs.len(), vertex_count);
        assert_eq!(colors.len(), vertex_count);

        let indices_ptr = indices.map(|i| i.as_ptr()).unwrap_or(ptr::null());
        let indices_count = indices.map(|i| i.len()).unwrap_or(0);

        Vertices::from_ptr(unsafe {
            sb::C_SkVertices_MakeCopy(
                mode,
                vertex_count as _,
                positions.native().as_ptr(),
                texs.native().as_ptr(),
                colors.native().as_ptr(),
                indices_count.try_into().unwrap(),
                indices_ptr,
            )
        })
        .unwrap()
    }

    pub const MAX_CUSTOM_ATTRIBUTES: usize = 8;

    pub fn unique_id(&self) -> u32 {
        self.native().fUniqueID
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    pub fn mode(&self) -> VertexMode {
        self.native().fMode
    }

    pub fn bounds(&self) -> &Rect {
        Rect::from_native_ref(&self.native().fBounds)
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn has_colors(&self) -> bool {
        self.colors().is_some()
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn has_tex_coords(&self) -> bool {
        self.tex_coords().is_some()
    }

    #[deprecated(since = "0.29.0", note = "returns false")]
    pub fn has_bones(&self) -> bool {
        false
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn has_indices(&self) -> bool {
        self.indices().is_some()
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    pub fn vertex_count(&self) -> usize {
        self.native().fVertexCount.try_into().unwrap()
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    pub fn index_count(&self) -> usize {
        self.native().fIndexCount.try_into().unwrap()
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn positions(&self) -> &[Point] {
        let positions: *const SkPoint = self.native().fPositions;
        unsafe { slice::from_raw_parts(positions as _, self.vertex_count()) }
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn tex_coords(&self) -> Option<&[Point]> {
        let texs = self.native().fTexs.into_option()?;
        Some(unsafe { slice::from_raw_parts(texs.as_ptr() as *const _, self.vertex_count()) })
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn colors(&self) -> Option<&[Color]> {
        let colors = self.native().fColors.into_option()?;
        Some(unsafe { slice::from_raw_parts(colors.as_ptr() as *const _, self.vertex_count()) })
    }

    #[deprecated(since = "0.29.0", note = "returns None")]
    #[allow(deprecated)]
    pub fn bone_indices(&self) -> Option<&[BoneIndices]> {
        None
    }

    #[deprecated(since = "0.29.0", note = "returns None")]
    #[allow(deprecated)]
    pub fn bone_weights(&self) -> Option<&[BoneWeights]> {
        None
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn indices(&self) -> Option<&[u16]> {
        let indices = self.native().fIndices.into_option()?;
        Some(unsafe { slice::from_raw_parts_mut(indices.as_ptr(), self.index_count()) })
    }

    #[deprecated(since = "0.29.0", note = "returns false")]
    pub fn is_volatile(&self) -> bool {
        false
    }

    #[deprecated(since = "0.29.0", note = "removed without replacement")]
    #[allow(deprecated)]
    pub fn apply_bones(&self, _bones: &[Bone]) -> ! {
        unimplemented!("removed without replacement")
    }

    pub fn approximate_size(&self) -> usize {
        unsafe { self.native().approximateSize() }
    }

    #[deprecated(since = "0.31.0", note = "removed without replacement")]
    pub fn decode(_buffer: &[u8]) -> ! {
        panic!("removed without replacement");
    }

    #[deprecated(since = "0.31.0", note = "removed without replacement")]
    pub fn encode(&self) -> ! {
        panic!("removed without replacement");
    }
}

bitflags! {
    pub struct BuilderFlags: u32 {
        const HAS_TEX_COORDS = sb::SkVertices_BuilderFlags_kHasTexCoords_BuilderFlag as u32;
        const HAS_COLORS = sb::SkVertices_BuilderFlags_kHasColors_BuilderFlag as u32;
    }
}

pub type Builder = Handle<SkVertices_Builder>;
unsafe impl Send for Builder {}
unsafe impl Sync for Builder {}

impl NativeDrop for SkVertices_Builder {
    fn drop(&mut self) {
        unsafe { sb::C_SkVertices_Builder_destruct(self) }
    }
}

impl Handle<SkVertices_Builder> {
    pub fn new(
        mode: VertexMode,
        vertex_count: usize,
        index_count: usize,
        flags: BuilderFlags,
    ) -> Builder {
        Self::from_native_c(unsafe {
            SkVertices_Builder::new(
                mode,
                vertex_count.try_into().unwrap(),
                index_count.try_into().unwrap(),
                flags.bits(),
            )
        })
    }

    pub fn is_valid(&self) -> bool {
        !self.native().fVertices.fPtr.is_null()
    }

    pub fn positions(&mut self) -> &mut [Point] {
        unsafe {
            let vertices = &*self.native().fVertices.fPtr;
            slice::from_raw_parts_mut(
                Point::from_native_ref_mut(&mut *vertices.fPositions),
                vertices.fVertexCount.try_into().unwrap(),
            )
        }
    }

    pub fn indices(&mut self) -> Option<&mut [u16]> {
        unsafe {
            let vertices = &*self.native().fVertices.fPtr;
            let indices = vertices.fIndices.into_option()?;
            Some(slice::from_raw_parts_mut(
                indices.as_ptr(),
                vertices.fIndexCount.try_into().unwrap(),
            ))
        }
    }

    // TODO: customData()

    pub fn tex_coords(&mut self) -> Option<&mut [Point]> {
        unsafe {
            let vertices = &*self.native().fVertices.fPtr;
            let mut coords = vertices.fTexs.into_option()?;
            Some(slice::from_raw_parts_mut(
                Point::from_native_ref_mut(coords.as_mut()),
                vertices.fVertexCount.try_into().unwrap(),
            ))
        }
    }

    pub fn colors(&mut self) -> Option<&mut [Color]> {
        unsafe {
            let vertices = &*self.native().fVertices.fPtr;
            let mut colors = vertices.fColors.into_option()?;
            Some(slice::from_raw_parts_mut(
                Color::from_native_ref_mut(colors.as_mut()),
                vertices.fVertexCount.try_into().unwrap(),
            ))
        }
    }

    #[deprecated(since = "0.29.0", note = "returns false")]
    pub fn is_volatile(&self) -> bool {
        false
    }

    #[deprecated(since = "0.29.0", note = "returns None")]
    #[allow(deprecated)]
    pub fn bone_indices(&mut self) -> Option<&mut [BoneIndices]> {
        None
    }

    #[deprecated(since = "0.29.0", note = "returns None")]
    #[allow(deprecated)]
    pub fn bone_weights(&mut self) -> Option<&mut [BoneWeights]> {
        None
    }

    pub fn detach(mut self) -> Vertices {
        Vertices::from_ptr(unsafe { sb::C_SkVertices_Builder_detach(self.native_mut()) }).unwrap()
    }
}
