use crate::prelude::*;
use crate::{Color, Data, Point, Rect};
use skia_bindings as sb;
use skia_bindings::{SkColor, SkPoint, SkVertices, SkVertices_Bone, SkVertices_Builder};
#[cfg(test)]
use skia_bindings::{SkVertices_BoneIndices, SkVertices_BoneWeights};
#[cfg(test)]
use std::mem;
use std::ops::{Index, IndexMut};
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
#[repr(C)]
pub struct Bone {
    values: [f32; 6],
}

impl NativeTransmutable<SkVertices_Bone> for Bone {}

impl Index<usize> for Bone {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for Bone {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl Bone {
    pub fn map_point(&self, point: impl Into<Point>) -> Point {
        let point = point.into();
        let values = &self.values;
        let x = values[0] * point.x + values[2] * point.y + values[4];
        let y = values[1] * point.x + values[3] * point.y + values[5];
        Point::new(x, y)
    }

    pub fn map_rect(&self, rect: impl AsRef<Rect>) -> Rect {
        Rect::from_native(unsafe {
            sb::C_SkVertices_Bone_mapRect(self.native(), rect.as_ref().native())
        })
    }
}

#[test]
fn test_bone_layout() {
    Bone::test_layout();
}

pub use skia_bindings::SkVertices_VertexMode as VertexMode;
#[test]
fn test_vertices_vertex_mode_naming() {
    let _ = VertexMode::Triangles;
}

pub type Vertices = RCHandle<SkVertices>;

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
        bone_indices_and_weights: Option<(&[BoneIndices], &[BoneWeights])>,
        indices: Option<&[u16]>,
        is_volatile: impl Into<Option<bool>>,
    ) -> Vertices {
        let vertex_count = positions.len();
        assert_eq!(texs.len(), vertex_count);
        assert_eq!(colors.len(), vertex_count);
        if let Some((bi, bw)) = bone_indices_and_weights {
            assert_eq!(bi.len(), vertex_count);
            assert_eq!(bw.len(), vertex_count);
        }

        let bone_indices = bone_indices_and_weights.map(|t| t.0);
        let bone_weights = bone_indices_and_weights.map(|t| t.1);

        let bone_indices_ptr = bone_indices.map(|bi| bi.as_ptr()).unwrap_or(ptr::null());
        let bone_weights_ptr = bone_weights.map(|bw| bw.as_ptr()).unwrap_or(ptr::null());

        let indices_ptr = indices.map(|i| i.as_ptr()).unwrap_or(ptr::null());
        let indices_count = indices.map(|i| i.len()).unwrap_or(0);

        Vertices::from_ptr(unsafe {
            sb::C_SkVertices_MakeCopy(
                mode,
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
        self.native().fUniqueID
    }

    pub fn mode(&self) -> VertexMode {
        self.native().fMode
    }

    pub fn bounds(&self) -> &Rect {
        Rect::from_native_ref(&self.native().fBounds)
    }

    pub fn has_colors(&self) -> bool {
        self.colors().is_some()
    }

    pub fn has_tex_coords(&self) -> bool {
        self.tex_coords().is_some()
    }

    pub fn has_bones(&self) -> bool {
        self.bone_indices().is_some()
    }

    pub fn has_indices(&self) -> bool {
        self.indices().is_some()
    }

    pub fn vertex_count(&self) -> usize {
        self.native().fVertexCnt.try_into().unwrap()
    }

    pub fn positions(&self) -> &[Point] {
        let positions: *const SkPoint = self.native().fPositions;
        unsafe { slice::from_raw_parts(positions as _, self.vertex_count()) }
    }

    pub fn tex_coords(&self) -> Option<&[Point]> {
        let texs: *const SkPoint = self.native().fTexs.into_option()?;
        Some(unsafe { slice::from_raw_parts(texs as _, self.vertex_count()) })
    }

    pub fn colors(&self) -> Option<&[Color]> {
        let colors: *const SkColor = self.native().fColors.into_option()?;
        Some(unsafe { slice::from_raw_parts(colors as _, self.vertex_count()) })
    }

    pub fn bone_indices(&self) -> Option<&[BoneIndices]> {
        let indices = self.native().fBoneIndices.into_option()?;
        Some(unsafe { slice::from_raw_parts(indices as _, self.vertex_count()) })
    }

    pub fn bone_weights(&self) -> Option<&[BoneWeights]> {
        let weights = self.native().fBoneWeights.into_option()?;
        Some(unsafe { slice::from_raw_parts(weights as _, self.vertex_count()) })
    }

    pub fn index_count(&self) -> usize {
        self.native().fIndexCnt.try_into().unwrap()
    }

    pub fn indices(&self) -> Option<&[u16]> {
        let indices = self.native().fIndices.into_option()?;
        Some(unsafe { slice::from_raw_parts_mut(indices as _, self.index_count()) })
    }

    pub fn is_volatile(&self) -> bool {
        self.native().fIsVolatile
    }

    pub fn apply_bones(&self, bones: &[Bone]) -> Vertices {
        Vertices::from_ptr(unsafe {
            sb::C_SkVertices_applyBones(
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
        Vertices::from_ptr(unsafe { sb::C_SkVertices_Decode(buffer.as_ptr() as _, buffer.len()) })
    }

    pub fn encode(&self) -> Data {
        Data::from_ptr(unsafe { sb::C_SkVertices_encode(self.native()) }).unwrap()
    }
}

bitflags! {
    pub struct BuilderFlags: u32 {
        const HAS_TEX_COORDS = sb::SkVertices_BuilderFlags_kHasTexCoords_BuilderFlag as u32;
        const HAS_COLORS = sb::SkVertices_BuilderFlags_kHasColors_BuilderFlag as u32;
        const HAS_BONES = sb::SkVertices_BuilderFlags_kHasBones_BuilderFlag as u32;
        const IS_NON_VOLATILE = sb::SkVertices_BuilderFlags_kIsNonVolatile_BuilderFlag as u32;
    }
}

pub type Builder = Handle<SkVertices_Builder>;

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
        Self::from_native(unsafe {
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
            let coords: *mut SkPoint = self.native_mut().texCoords().into_option()?;
            Some(slice::from_raw_parts_mut(coords as _, self.vertex_count()))
        }
    }

    pub fn colors(&mut self) -> Option<&mut [Color]> {
        unsafe {
            let colors: *mut SkColor = self.native_mut().colors().into_option()?;
            Some(slice::from_raw_parts_mut(colors as _, self.vertex_count()))
        }
    }

    pub fn bone_indices(&mut self) -> Option<&mut [BoneIndices]> {
        unsafe {
            let indices = self.native_mut().boneIndices().into_option()?;
            Some(slice::from_raw_parts_mut(indices as _, self.vertex_count()))
        }
    }

    pub fn bone_weights(&mut self) -> Option<&mut [BoneWeights]> {
        unsafe {
            let weights = self.native_mut().boneWeights().into_option()?;
            Some(slice::from_raw_parts_mut(weights as _, self.vertex_count()))
        }
    }

    pub fn indices(&mut self) -> Option<&mut [u16]> {
        unsafe {
            let indices = self.native_mut().indices().into_option()?;
            Some(slice::from_raw_parts_mut(indices as _, self.index_count()))
        }
    }

    pub fn detach(mut self) -> Vertices {
        Vertices::from_ptr(unsafe { sb::C_SkVertices_Builder_detach(self.native_mut()) }).unwrap()
    }
}
