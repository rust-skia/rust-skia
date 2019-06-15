use crate::prelude::*;
use crate::{Color, Data, Point, Rect};
use skia_bindings::{
    C_SkVertices_Bone_mapRect, C_SkVertices_Builder_destruct, C_SkVertices_Builder_detach,
    C_SkVertices_Decode, C_SkVertices_MakeCopy, C_SkVertices_applyBones, C_SkVertices_encode,
    C_SkVertices_ref, C_SkVertices_unique, C_SkVertices_unref, SkColor, SkPoint, SkVertices,
    SkVertices_Bone, SkVertices_Builder, SkVertices_BuilderFlags_kHasBones_BuilderFlag,
    SkVertices_BuilderFlags_kHasColors_BuilderFlag,
    SkVertices_BuilderFlags_kHasTexCoords_BuilderFlag,
    SkVertices_BuilderFlags_kIsNonVolatile_BuilderFlag, SkVertices_VertexMode,
};
#[cfg(test)]
use skia_bindings::{SkVertices_BoneIndices, SkVertices_BoneWeights};
#[cfg(test)]
use std::mem;
use std::ops::{Deref, DerefMut};
use std::{ptr, slice};

pub type BoneIndices = [u32; 4];

#[test]
fn bone_indices_layout() {
    assert_eq!(
        mem::size_of::<BoneIndices>(),
        mem::size_of::<SkVertices_BoneIndices>()
    );
}

pub type BoneWeights = [u32; 4];

#[test]
fn bone_weights_layout() {
    assert_eq!(
        mem::size_of::<BoneWeights>(),
        mem::size_of::<SkVertices_BoneWeights>()
    );
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct Bone([u32; 6]);

impl Deref for Bone {
    type Target = [u32; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl NativeTransmutable<SkVertices_Bone> for Bone {}

impl Bone {
    pub fn map_point(&self, point: impl Into<Point>) -> Point {
        Point::from_native(unsafe { self.native().mapPoint(&point.into().into_native()) })
    }

    pub fn map_rect(&self, rect: impl AsRef<Rect>) -> Rect {
        Rect::from_native(unsafe {
            // does not link.
            // self.native().mapRect(rect.as_ref().native())
            C_SkVertices_Bone_mapRect(self.native(), rect.as_ref().native())
        })
    }
}

#[test]
fn test_bone_layout() {
    Bone::test_layout();
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum VertexMode {
    Triangles = SkVertices_VertexMode::kTriangles_VertexMode as _,
    TriangleStrip = SkVertices_VertexMode::kTriangleStrip_VertexMode as _,
    TriangleFan = SkVertices_VertexMode::kTriangleFan_VertexMode as _,
}

impl NativeTransmutable<SkVertices_VertexMode> for VertexMode {}
#[test]
fn test_vertices_vertex_mode_layout() {
    VertexMode::test_layout()
}

pub type Vertices = RCHandle<SkVertices>;

impl NativeRefCounted for SkVertices {
    fn _ref(&self) {
        unsafe { C_SkVertices_ref(self) }
    }

    fn _unref(&self) {
        unsafe { C_SkVertices_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { C_SkVertices_unique(self) }
    }
}

impl RCHandle<SkVertices> {
    pub fn new_copy(
        mode: VertexMode,
        positions: &[Point],
        texs: &[Point],
        colors: &[Color],
        bone_indices_and_weights: Option<(&BoneIndices, &BoneWeights)>,
        indices: Option<&[u16]>,
        is_volatile: impl Into<Option<bool>>,
    ) -> Vertices {
        let vertex_count = positions.len();
        assert_eq!(vertex_count, texs.len());
        assert_eq!(vertex_count, colors.len());

        let bone_indices = bone_indices_and_weights.map(|t| t.0);
        let bone_weights = bone_indices_and_weights.map(|t| t.1);

        let bone_indices_ptr = bone_indices.map(|bi| bi.as_ptr()).unwrap_or(ptr::null());
        let bone_weights_ptr = bone_weights.map(|bw| bw.as_ptr()).unwrap_or(ptr::null());

        let indices_ptr = indices.map(|i| i.as_ptr()).unwrap_or(ptr::null());
        let indices_count = indices.map(|i| i.len()).unwrap_or(0);

        Vertices::from_ptr(unsafe {
            C_SkVertices_MakeCopy(
                mode.into_native(),
                vertex_count as _,
                positions.native().as_ptr(),
                texs.native().as_ptr(),
                colors.native().as_ptr(),
                bone_indices_ptr as _,
                bone_weights_ptr as _,
                indices_count.try_into().unwrap(),
                indices_ptr,
                is_volatile.into().unwrap_or(true),
            )
        })
        .unwrap()
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { self.native().uniqueID() }
    }

    pub fn mode(&self) -> VertexMode {
        VertexMode::from_native(unsafe { self.native().mode() })
    }

    pub fn bounds(&self) -> Rect {
        Rect::from_native(unsafe { *self.native().bounds() })
    }

    pub fn has_colors(&self) -> bool {
        unsafe { self.native().hasColors() }
    }

    pub fn has_tex_coords(&self) -> bool {
        unsafe { self.native().hasTexCoords() }
    }

    pub fn has_bones(&self) -> bool {
        unsafe { self.native().hasBones() }
    }

    pub fn has_indices(&self) -> bool {
        unsafe { self.native().hasIndices() }
    }

    pub fn vertex_count(&self) -> usize {
        unsafe { self.native().vertexCount().try_into().unwrap() }
    }

    pub fn positions(&self) -> &[Point] {
        unsafe {
            let ptr: *const SkPoint = self.native().positions();
            slice::from_raw_parts(ptr as _, self.vertex_count())
        }
    }

    pub fn tex_coords(&self) -> Option<&[Point]> {
        unsafe {
            let ptr: *const SkPoint = self.native().positions().to_option()?;
            Some(slice::from_raw_parts(ptr as _, self.vertex_count()))
        }
    }

    pub fn colors(&self) -> Option<&[Color]> {
        unsafe {
            let ptr: *const SkColor = self.native().colors().to_option()?;
            Some(slice::from_raw_parts(ptr as _, self.vertex_count()))
        }
    }

    pub fn bone_indices(&self) -> Option<&[BoneIndices]> {
        unsafe {
            let indices = self.native().boneIndices().to_option()?;
            Some(slice::from_raw_parts_mut(indices as _, self.vertex_count()))
        }
    }

    pub fn bone_weights(&self) -> Option<&[BoneWeights]> {
        unsafe {
            let weights = self.native().boneWeights().to_option()?;
            Some(slice::from_raw_parts_mut(weights as _, self.vertex_count()))
        }
    }

    pub fn index_count(&self) -> usize {
        unsafe { self.native().indexCount().try_into().unwrap() }
    }

    pub fn indices(&self) -> Option<&[u16]> {
        unsafe {
            let indices = self.native().indices().to_option()?;
            Some(slice::from_raw_parts_mut(indices as _, self.index_count()))
        }
    }

    pub fn is_volatile(&self) -> bool {
        unsafe { self.native().isVolatile() }
    }

    pub fn apply_bones(&self, bones: &[Bone]) -> Vertices {
        Vertices::from_ptr(unsafe {
            C_SkVertices_applyBones(
                self.native(),
                bones.native().as_ptr(),
                bones.len().try_into().unwrap(),
            )
        })
        .unwrap()
    }

    pub fn approximate_size(&self) -> usize {
        unsafe { self.native().approximateSize() }
    }

    pub fn decode(buffer: &[u8]) -> Option<Vertices> {
        Vertices::from_ptr(unsafe { C_SkVertices_Decode(buffer.as_ptr() as _, buffer.len()) })
    }

    pub fn encode(&self) -> Data {
        Data::from_ptr(unsafe { C_SkVertices_encode(self.native()) }).unwrap()
    }
}

bitflags! {
    pub struct BuilderFlags: u32 {
        const HAS_TEX_COORDS = SkVertices_BuilderFlags_kHasTexCoords_BuilderFlag as u32;
        const HAS_COLORS = SkVertices_BuilderFlags_kHasColors_BuilderFlag as u32;
        const HAS_BONES = SkVertices_BuilderFlags_kHasBones_BuilderFlag as u32;
        const IS_NON_VOLATILE = SkVertices_BuilderFlags_kIsNonVolatile_BuilderFlag as u32;
    }
}

pub type Builder = Handle<SkVertices_Builder>;

impl NativeDrop for SkVertices_Builder {
    fn drop(&mut self) {
        unsafe { C_SkVertices_Builder_destruct(self) }
    }
}

impl Handle<SkVertices_Builder> {
    pub fn new(
        mode: VertexMode,
        vertex_count: usize,
        index_count: usize,
        flags: BuilderFlags,
    ) -> Builder {
        Self::from_native(unsafe {
            SkVertices_Builder::new(
                mode.into_native(),
                vertex_count.try_into().unwrap(),
                index_count.try_into().unwrap(),
                flags.bits(),
            )
        })
    }

    pub fn is_valid(&self) -> bool {
        // does not link
        // unsafe { self.native().isValid() }
        // TODO: write a C wrapper function in case the implementation changes
        !self.native().fVertices.fPtr.is_null()
    }

    pub fn vertex_count(&self) -> usize {
        unsafe { self.native().vertexCount() }.try_into().unwrap()
    }

    pub fn index_count(&self) -> usize {
        unsafe { self.native().indexCount() }.try_into().unwrap()
    }

    pub fn is_volatile(&self) -> bool {
        unsafe { self.native().isVolatile() }
    }

    pub fn positions(&mut self) -> &mut [Point] {
        unsafe {
            let positions: *mut SkPoint = self.native_mut().positions();
            slice::from_raw_parts_mut(positions as _, self.vertex_count())
        }
    }

    pub fn tex_coords(&mut self) -> Option<&mut [Point]> {
        unsafe {
            let coords: *mut SkPoint = self.native_mut().texCoords().to_option()?;
            Some(slice::from_raw_parts_mut(coords as _, self.vertex_count()))
        }
    }

    pub fn colors(&mut self) -> Option<&mut [Color]> {
        unsafe {
            let colors: *mut SkColor = self.native_mut().colors().to_option()?;
            Some(slice::from_raw_parts_mut(colors as _, self.vertex_count()))
        }
    }

    pub fn bone_indices(&mut self) -> Option<&mut [BoneIndices]> {
        unsafe {
            let indices = self.native_mut().boneIndices().to_option()?;
            Some(slice::from_raw_parts_mut(indices as _, self.vertex_count()))
        }
    }

    pub fn bone_weights(&mut self) -> Option<&mut [BoneWeights]> {
        unsafe {
            let weights = self.native_mut().boneWeights().to_option()?;
            Some(slice::from_raw_parts_mut(weights as _, self.vertex_count()))
        }
    }

    pub fn indices(&mut self) -> Option<&mut [u16]> {
        unsafe {
            let indices = self.native_mut().indices().to_option()?;
            Some(slice::from_raw_parts_mut(indices as _, self.index_count()))
        }
    }

    pub fn detach(mut self) -> Vertices {
        Vertices::from_ptr(unsafe { C_SkVertices_Builder_detach(self.native_mut()) }).unwrap()
    }
}
