use std::ops::{DerefMut,Deref};
use std::{ptr, mem, slice};
use crate::prelude::*;
use crate::skia::{
    Point,
    Rect,
    Color,
    Data
};
use rust_skia::{
    C_SkVertices_Decode,
    C_SkVertices_applyBones,
    C_SkVertices_Builder_detach,
    C_SkVertices_Builder_destruct,
    SkVertices_Builder,
    SkVertices_BoneIndices,
    SkVertices_BoneWeights,
    SkColor,
    SkPoint,
    C_SkVertices_MakeCopy,
    C_SkVertices_ref,
    SkVertices,
    C_SkVertices_unref,
    SkVertices_Bone,
    SkVertices_VertexMode,
    SkVertices_BuilderFlags,
    C_SkVertices_encode
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
pub type VerticesVertexMode = EnumHandle<SkVertices_VertexMode>;

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
        mode: VerticesVertexMode,
        positions: &[Point],
        texs: &[Point],
        colors: &[Color],
        bone_indices_and_weights: Option<(&BoneIndices, &BoneWeights)>,
        indices: Option<&[u16]>,
        is_volatile: bool) -> Vertices {

        let vertex_count = positions.len();
        assert_eq!(vertex_count, texs.len());
        assert_eq!(vertex_count, colors.len());

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
                indices_count.try_into().unwrap(),
                indices_ptr,
                is_volatile
            )}).unwrap()
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { self.native().uniqueID() }
    }

    pub fn mode(&self) -> VerticesVertexMode {
        unsafe { self.native().mode() }.into_handle()
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

    // TODO: use wrapper type as soon we can transmute points
    pub fn positions(&self) -> &[SkPoint] {
        unsafe {
            let ptr = self.native().positions();
            slice::from_raw_parts(ptr, self.vertex_count())
        }
    }

    // TODO: use wrapper type as soon we can transmute points
    pub fn tex_coords(&self) -> Option<&[SkPoint]> {
        unsafe {
            let ptr = self.native().positions().to_option()?;
            Some(slice::from_raw_parts(ptr, self.vertex_count()))
        }
    }

    // TODO: use wrapper type as soon we can transmute colors
    pub fn colors(&self) -> Option<&[SkColor]> {
        unsafe {
            let ptr = self.native().colors().to_option()?;
            Some(slice::from_raw_parts(ptr, self.vertex_count()))
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
        unsafe {
            self.native().isVolatile()
        }
    }

    pub fn apply_bones(&self, bones: &[Bone]) -> Vertices {
        Vertices::from_ptr(unsafe {
            C_SkVertices_applyBones(
                self.native(),
                bones.native().as_ptr(),
                bones.len().try_into().unwrap())
        }).unwrap()
    }

    pub fn approximate_size(&self) -> usize {
        unsafe { self.native().approximateSize() }
    }

    pub fn decode(buffer: &[u8]) -> Option<Vertices> {
        Vertices::from_ptr(unsafe {
            C_SkVertices_Decode(buffer.as_ptr() as _, buffer.len())
        })
    }

    pub fn encode(&self) -> Data {
        Data::from_ptr(unsafe {
            C_SkVertices_encode(self.native())
        }).unwrap()
    }
}

bitflags! {
    pub struct VerticesBuilderFlags: u32 {
        const HasTexCoords = SkVertices_BuilderFlags::kHasTexCoords_BuilderFlag as u32;
        const HasColors = SkVertices_BuilderFlags::kHasColors_BuilderFlag as u32;
        const HasBones = SkVertices_BuilderFlags::kHasBones_BuilderFlag as u32;
        const IsNonVolatile = SkVertices_BuilderFlags::kIsNonVolatile_BuilderFlag as u32;
    }
}

pub type VerticesBuilder = Handle<SkVertices_Builder>;

impl NativeDrop for SkVertices_Builder {
    fn drop(&mut self) {
        unsafe { C_SkVertices_Builder_destruct(self) }
    }
}

impl Handle<SkVertices_Builder> {
    pub fn new(mode: VerticesVertexMode, vertex_count: usize, index_count: usize, flags: VerticesBuilderFlags) -> VerticesBuilder {
        unsafe {
            SkVertices_Builder::new(
                mode.native(),
                vertex_count.try_into().unwrap(),
                index_count.try_into().unwrap(),
                flags.bits())
        }.into_handle()
    }

    pub fn is_valid(&self) -> bool {
        // does not link
        // unsafe { self.native().isValid() }
        // TODO: write a C wrapper function in case the implementation changes
        self.native().fVertices.fPtr != ptr::null_mut()
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

    // TODO: implement this with the proper return type as soon we can transmute points.
    pub fn positions(&mut self) -> &mut [SkPoint] {
        unsafe {
            let positions = self.native_mut().positions();
            slice::from_raw_parts_mut(positions, self.vertex_count())
        }
    }

    // TODO: implement this with the proper return type as soon we can transmute points.
    pub fn tex_coords(&mut self) -> Option<&mut [SkPoint]> {
        unsafe {
            let coords = self.native_mut().texCoords().to_option()?;
            Some(slice::from_raw_parts_mut(coords, self.vertex_count()))
        }
    }

    // TODO: implement this with the proper return type as soon we can transmute points.
    pub fn colors(&mut self) -> Option<&mut [SkColor]> {
        unsafe {
            let colors = self.native_mut().colors().to_option()?;
            Some(slice::from_raw_parts_mut(colors, self.vertex_count()))
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
        Vertices::from_ptr(unsafe {
            C_SkVertices_Builder_detach(self.native_mut())
        }).unwrap()
    }
}