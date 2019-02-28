use std::ops::{DerefMut,Deref};
use std::{ptr, mem};
use crate::prelude::*;
use crate::skia::{
    Point,
    Rect,
    Color
};
use rust_skia::{
    SkColor,
    SkPoint,
    C_SkVertices_MakeCopy,
    C_SkVertices_ref,
    SkVertices,
    C_SkVertices_unref,
    SkVertices_Bone,
    SkVertices_VertexMode,
};
#[cfg(test)]
use rust_skia:: {
    SkVertices_BoneIndices,
    SkVertices_BoneWeights
};

pub type BoneIndices = [u32; 4];

#[test]
fn bone_indices_layout() {
    assert_eq!(mem::size_of::<BoneIndices>(), mem::size_of::<SkVertices_BoneIndices>());
}

pub type BoneWeights = [u32; 4];

#[test]
fn bone_weights_layout() {
    assert_eq!(mem::size_of::<BoneWeights>(), mem::size_of::<SkVertices_BoneWeights>());
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
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
    pub fn map_point(&self, point: &Point) -> Point {
        Point::from_native(unsafe {
            self.native().mapPoint(&point.into_native())
        })
    }

    pub fn map_rect(&self, rect: &Rect) -> Rect {
        Rect::from_native(unsafe {
            self.native().mapRect(&rect.into_native())
        })
    }
}

#[test]
fn test_bone_layout() {
    Bone::test_layout();
}

    // TODO: think about renaming EnumHandle to EnumWrapper (and others?)
pub type VertexMode = EnumHandle<SkVertices_VertexMode>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkVertices_VertexMode> {
    pub const Triangles: Self = Self(SkVertices_VertexMode::kTriangles_VertexMode);
    pub const TriangleStrip: Self = Self(SkVertices_VertexMode::kTriangleStrip_VertexMode);
    pub const TriangleFan: Self = Self(SkVertices_VertexMode::kTriangleFan_VertexMode);
}

pub type Vertices = RCHandle<SkVertices>;

impl NativeRefCounted for SkVertices {
    fn _ref(&self) {
        unsafe { C_SkVertices_ref(self) }
    }

    fn _unref(&self) {
        unsafe { C_SkVertices_unref(self) }
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
        is_volatile: bool) -> Vertices {

        let vertex_count = positions.len();
        assert_eq!(vertex_count, texs.len());
        assert_eq!(vertex_count, colors.len());
        // TODO: consider to use crate as_num
        assert!(vertex_count <= i32::max_value() as _);

        let bone_indices = bone_indices_and_weights.map(|t| t.0);
        let bone_weights = bone_indices_and_weights.map(|t| t.1);

        let bone_indices_ptr = bone_indices.map(|bi| bi.as_ptr()).unwrap_or(ptr::null());
        let bone_weights_ptr = bone_weights.map(|bw| bw.as_ptr()).unwrap_or(ptr::null());

        let indices_ptr = indices.map(|i| i.as_ptr()).unwrap_or(ptr::null());
        let indices_count = indices.map(|i| i.len()).unwrap_or(0);

        let positions: Vec<SkPoint> = positions.iter().map(|p| p.into_native()).collect();
        let texs: Vec<SkPoint> = texs.iter().map(|p| p.into_native()).collect();
        let colors: Vec<SkColor> = colors.iter().map(|c| c.0).collect();

        Vertices::from_ptr(unsafe {
            C_SkVertices_MakeCopy(
                mode.native(),
                vertex_count as _,
                positions.as_ptr(),
                texs.as_ptr(),
                colors.as_ptr(),
                bone_indices_ptr as _,
                bone_weights_ptr as _,
                indices_count as _,
                indices_ptr,
                is_volatile
            )}).unwrap()
    }
}
