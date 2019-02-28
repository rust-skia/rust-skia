use std::ops::{DerefMut,Deref};
use crate::prelude::*;
use crate::skia::{
    Point,
    Rect
};
use rust_skia::{
    C_SkVertices_ref,
    SkVertices,
    C_SkVertices_unref,
    SkVertices_BoneIndices,
    SkVertices_Bone,
    SkVertices_VertexMode
};

pub type BoneIndices = [u32; 4];
pub type BoneWeights = [u32; 4];

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

/*
impl RCHandle<SkVertices> {
    fn new_copy(
        mode: VertexMode,
        positions: &[Point],
        texs: &[Point],
        colors: &[Color], )


}
*/

#[test]
fn test_bone_layout() {
    Bone::test_layout();
}