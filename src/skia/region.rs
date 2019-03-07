use crate::prelude::*;
use crate::skia::{
    IRect,
    Path,
    IPoint,
    Contains,
    QuickReject
};
use rust_skia::{
    C_SkRegion_destruct,
    C_SkRegion_equals,
    SkRegion,
    SkRegion_Op
};
use crate::skia::IVector;

pub type Region = Handle<SkRegion>;

impl NativeDrop for SkRegion {
    fn drop(&mut self) {
        // does not link:
        // unsafe { SkRegion::destruct(self) }
        unsafe { C_SkRegion_destruct(self) }
    }
}

impl NativeClone for SkRegion {
    fn clone(&self) -> Self {
        unsafe { SkRegion::new1(self) }
    }
}

impl NativePartialEq for SkRegion {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkRegion_equals(self, rhs) }
    }
}

pub type RegionOp = EnumHandle<SkRegion_Op>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkRegion_Op> {
    pub const Difference: Self = Self(SkRegion_Op::kDifference_Op);
    pub const Intersect: Self = Self(SkRegion_Op::kIntersect_Op);
    pub const Union: Self = Self(SkRegion_Op::kUnion_Op);
    pub const XOR: Self = Self(SkRegion_Op::kXOR_Op);
    pub const ReverseDifference: Self = Self(SkRegion_Op::kReverseDifference_Op);
    pub const Replace: Self = Self(SkRegion_Op::kReplace_Op);
}

impl Handle<SkRegion> {
    pub fn new() -> Region {
        unsafe { SkRegion::new() }.into_handle()
    }

    pub fn from_rect(rect: &IRect) -> Region {
        unsafe { SkRegion::new2(&rect.into_native()) }
            .into_handle()
    }

    pub fn set(&mut self, src: &Region) -> bool {
        unsafe { self.native_mut().set(src.native()) }
    }

    pub fn swap(&mut self, other: &mut Region) {
        unsafe { self.native_mut().swap(other.native_mut()) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.native().isEmpty() }
    }

    pub fn is_rect(&self) -> bool {
        unsafe { self.native().isRect() }
    }

    pub fn is_complex(&self) -> bool {
        unsafe { self.native().isComplex() }
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_native(unsafe { *self.native().getBounds() })
    }

    pub fn compute_region_complexity(&self) -> usize {
        unsafe { self.native().computeRegionComplexity().try_into().unwrap() }
    }

    pub fn boundary_path(&self, path: &mut Path) -> bool {
        unsafe { self.native().getBoundaryPath(path.native_mut()) }
    }

    pub fn set_empty(&mut self) -> bool {
        unsafe { self.native_mut().setEmpty() }
    }

    pub fn set_rect(&mut self, rect: &IRect) -> bool {
        unsafe { self.native_mut().setRect(&rect.into_native()) }
    }

    pub fn set_rects(&mut self, rects: &[IRect]) -> bool {
        unsafe {
            self.native_mut().setRects(
                rects.native().as_ptr(),
                rects.len().try_into().unwrap())
        }
    }

    pub fn set_region(&mut self, region: &Region) -> bool {
        unsafe { self.native_mut().setRegion(region.native()) }
    }

    pub fn set_path(&mut self, path: &Path, clip: &Region) -> bool {
        unsafe { self.native_mut().setPath(path.native(), clip.native()) }
    }

    pub fn intersects_rect(&self, rect: &IRect) -> bool {
        unsafe { self.native().intersects(&rect.into_native()) }
    }

    pub fn intersects_region(&self, other: &Region) -> bool {
        unsafe { self.native().intersects1(other.native()) }
    }

    pub fn contains_point(&self, point: IPoint) -> bool {
        unsafe { self.native().contains(point.x, point.y) }
    }

    pub fn contains_rect(&self, rect: &IRect) -> bool {
        unsafe { self.native().contains1(&rect.into_native()) }
    }

    pub fn contains_region(&self, other: &Region) -> bool {
        unsafe { self.native().contains2(other.native()) }
    }

    pub fn quick_contains(&self, rect: &IRect) -> bool {
        unsafe { self.native().quickContains(&rect.into_native()) }
    }

    pub fn quick_reject_rect(&self, rect: &IRect) -> bool {
        unsafe { self.native().quickReject(&rect.into_native()) }
    }

    pub fn quick_reject_region(&self, other: &Region) -> bool {
        // does not link:
        // unsafe { self.native().quickReject1(other.native()) }
        self.is_empty()
            || other.is_empty()
            || !IRect::intersects(&self.bounds(), &other.bounds())
    }

    pub fn translate(&mut self, d: IVector) {
        unsafe { self.native_mut().translate(d.x, d.y) }
    }

    pub fn op_rect(&mut self, rect: &IRect, op: RegionOp) -> bool {
        unsafe { self.native_mut().op(&rect.into_native(), op.0 )}
    }

    pub fn op_region(&mut self, region: &Region, op: RegionOp) -> bool {
        unsafe { self.native_mut().op2(region.native(), op.0 )}
    }

    pub fn op_rect_region(&mut self, rect: &IRect, region: &Region, op: RegionOp) -> bool {
        unsafe { self.native_mut().op3(&rect.into_native(), region.native(), op.0) }
    }

    pub fn op_region_rect(&mut self, region: &Region, rect: &IRect, op: RegionOp) -> bool {
        unsafe { self.native_mut().op4(region.native(), &rect.into_native(), op.0) }
    }
}

//
// combine overloads (static)
//

pub trait Combine<A, B> : Sized {
    fn combine(a: &A, op: RegionOp, b: &B) -> Self;

    fn difference(a: &A, b: &B) -> Self {
        Self::combine(a, RegionOp::Difference, b)
    }

    fn intersect(a: &A, b: &B) -> Self {
        Self::combine(a, RegionOp::Intersect, b)
    }

    fn xor(a: &A, b: &B) -> Self {
        Self::combine(a, RegionOp::XOR, b)
    }

    fn union(a: &A, b: &B) -> Self {
        Self::combine(a, RegionOp::Union, b)
    }

    fn reverse_difference(a: &A, b: &B) -> Self {
        Self::combine(a, RegionOp::ReverseDifference, b)
    }

    fn replace(a: &A, b: &B) -> Self {
        Self::combine(a, RegionOp::Replace, b)
    }
}

impl Combine<IRect, Region> for Handle<SkRegion> {
    fn combine(rect: &IRect, op: RegionOp, region: &Region) -> Self {
        let mut r = Region::new();
        r.op_rect_region(rect, region, op);
        r
    }
}

impl Combine<Region, IRect> for Handle<SkRegion> {
    fn combine(region: &Region, op: RegionOp, rect: &IRect) -> Self {
        let mut r = Region::new();
        r.op_region_rect(region, rect, op);
        r
    }
}

impl Combine<Region, Region> for Handle<SkRegion> {
    fn combine(a: &Region, op: RegionOp, b: &Region) -> Self {
        let mut a = a.clone();
        a.op_region(b, op);
        a
    }
}

//
// intersects overloads
//

pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}

impl Intersects<IRect> for Region {
    fn intersects(&self, rect: &IRect) -> bool {
        self.intersects_rect(rect)
    }
}

impl Intersects<Region> for Region {
    fn intersects(&self, other: &Region) -> bool {
        self.intersects_region(other)
    }
}

//
// contains overloads
//

impl Contains<IPoint> for Region {
    fn contains(&self, point: IPoint) -> bool {
        self.contains_point(point)
    }
}

impl Contains<&IRect> for Region {
    fn contains(&self, rect: &IRect) -> bool {
        self.contains_rect(rect)
    }
}

impl Contains<&Region> for Region {
    fn contains(&self, other: &Region) -> bool {
        self.contains_region(other)
    }
}

//
// quick_reject overloads
//

impl QuickReject<IRect> for Region {
    fn quick_reject(&self, rect: &IRect) -> bool {
        self.quick_reject_rect(rect)
    }
}

impl QuickReject<Region> for Region {
    fn quick_reject(&self, other: &Region) -> bool {
        self.quick_reject_region(other)
    }
}

#[test]
fn new_clone_drop() {
    let region = Region::new();
    let _cloned = region.clone();
}

#[test]
fn can_compare() {
    let r1 = Region::new();
    let r2 = r1.clone();
    assert!(r1 == r2);
}
