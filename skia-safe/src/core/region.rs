//! Describes the set of pixels used to clip [`crate::Canvas`]. [`Region`] is compact, efficiently
//! storing a single integer rectangle, or a run length encoded array of rectangles. [`Region`] may
//! reduce the current [`crate::Canvas`] clip, or may be drawn as one or more integer rectangles.
//! The [`Region`] iterator returns the scan lines or rectangles contained by it, optionally
//! intersecting a bounding rectangle.

use std::{fmt, iter, marker::PhantomData, mem, ptr};

use crate::{Contains, IPoint, IRect, IVector, Path, PathBuilder, QuickReject, prelude::*};
use skia_bindings::{
    self as sb, SkRegion, SkRegion_Cliperator, SkRegion_Iterator, SkRegion_RunHead,
    SkRegion_Spanerator,
};

pub type Region = Handle<SkRegion>;
unsafe_send_sync!(Region);

impl NativeDrop for SkRegion {
    /// Releases ownership of any shared data and deletes data if the region is sole owner.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_destructor>
    fn drop(&mut self) {
        unsafe { sb::C_SkRegion_destruct(self) }
    }
}

impl NativeClone for SkRegion {
    /// Constructs a copy of an existing region. Makes two regions identical by value. Internally,
    /// the region and the returned result share pointer values. The underlying rectangle array is
    /// copied when modified.
    ///
    /// Creating a region copy is very efficient and never allocates memory. Regions are always
    /// copied by value from the interface; the underlying shared pointers are not exposed.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_copy_const_SkRegion>
    /// Example (C++): <https://fiddle.skia.org/c/@Region_copy_operator>
    fn clone(&self) -> Self {
        unsafe { SkRegion::new1(self) }
    }
}

impl NativePartialEq for SkRegion {
    /// Compares the region and `rhs`; returns true if they enclose exactly the same area.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_equal1_operator>
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkRegion_Equals(self, rhs) }
    }
}

impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Region")
            .field("is_empty", &self.is_empty())
            .field("is_rect", &self.is_rect())
            .field("is_complex", &self.is_complex())
            .field("bounds", &self.bounds())
            .finish()
    }
}

/// The logical operations that can be performed when combining two regions.
pub use skia_bindings::SkRegion_Op as RegionOp;
variant_name!(RegionOp::ReverseDifference);

impl Region {
    /// Constructs an empty region. The region is set to empty bounds at (0, 0) with zero width and
    /// height.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_empty_constructor>
    pub fn new() -> Region {
        Self::from_native_c(unsafe { SkRegion::new() })
    }

    /// Constructs a rectangular region matching the bounds of `rect`.
    ///
    /// - `rect` bounds of the constructed region
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_copy_const_SkIRect>
    pub fn from_rect(rect: impl AsRef<IRect>) -> Region {
        Self::from_native_c(unsafe { SkRegion::new2(rect.as_ref().native()) })
    }

    /// Sets the region to `src`, and returns true if `src` bounds is not empty. This makes the
    /// region and `src` identical by value. Internally, the region and `src` share pointer values.
    /// The underlying rectangle array is copied when modified.
    ///
    /// Creating a region copy is very efficient and never allocates memory. Regions are always
    /// copied by value from the interface; the underlying shared pointers are not exposed.
    ///
    /// - `src` region to copy
    pub fn set(&mut self, src: &Region) -> bool {
        unsafe { sb::C_SkRegion_set(self.native_mut(), src.native()) }
    }

    /// Exchanges the rectangle array of the region and `other`. `swap` internally exchanges
    /// pointers, so it is lightweight and does not allocate memory.
    ///
    /// `swap` usage has largely been replaced by assignment. Paths do not copy their content on
    /// assignment until they are written to, making assignment as efficient as `swap`.
    ///
    /// - `other` region to swap with
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_swap>
    pub fn swap(&mut self, other: &mut Region) {
        unsafe { self.native_mut().swap(other.native_mut()) }
    }

    const EMPTY_RUN_HEAD_PTR: *mut SkRegion_RunHead = -1 as _;
    const RECT_RUN_HEAD_PTR: *mut SkRegion_RunHead = ptr::null_mut();

    /// Returns true if the region is empty. An empty region has bounds width or height less than
    /// or equal to zero. The default constructor constructs an empty region; `set_empty` and
    /// `set_rect` with dimensionless data make the region empty.
    pub fn is_empty(&self) -> bool {
        ptr::eq(self.native().fRunHead, Self::EMPTY_RUN_HEAD_PTR)
    }

    /// Returns true if the region is one [`IRect`] with positive dimensions.
    pub fn is_rect(&self) -> bool {
        ptr::eq(self.native().fRunHead, Self::RECT_RUN_HEAD_PTR)
    }

    /// Returns true if the region is described by more than one rectangle.
    pub fn is_complex(&self) -> bool {
        !self.is_empty() && !self.is_rect()
    }

    /// Returns the minimum and maximum axes values of the rectangle array. Returns (0, 0, 0, 0) if
    /// the region is empty.
    pub fn bounds(&self) -> &IRect {
        IRect::from_native_ref(&self.native().fBounds)
    }

    /// Returns a value that increases with the number of elements in the region. Returns zero if
    /// the region is empty. Returns one if the region equals an [`IRect`]; otherwise, returns a
    /// value greater than one indicating that the region is complex.
    ///
    /// Call to compare regions for relative complexity.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_computeRegionComplexity>
    pub fn compute_region_complexity(&self) -> usize {
        unsafe { self.native().computeRegionComplexity().try_into().unwrap() }
    }

    /// Appends the outline of the region to the path builder. Returns true if the region is not
    /// empty; otherwise, returns false, and leaves the path unmodified.
    ///
    /// - `path` path to append to
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_getBoundaryPath>
    pub fn add_boundary_path(&self, path: &mut PathBuilder) -> bool {
        unsafe { self.native().addBoundaryPath(path.native_ptr()) }
    }

    #[deprecated(since = "0.91.0", note = "Use boundary_path()")]
    pub fn get_boundary_path(&self, path: &mut Path) -> bool {
        unsafe { sb::C_SkRegion_getBoundaryPath(self.native(), path.native_mut()) };
        !path.is_empty()
    }

    /// Returns the boundary of the region as a path, or `None` if the region is empty.
    pub fn boundary_path(&self) -> Option<Path> {
        let mut path = Path::default();
        unsafe { sb::C_SkRegion_getBoundaryPath(self.native(), path.native_mut()) };
        (!path.is_empty()).then_some(path)
    }

    /// Constructs an empty region. The region is set to empty bounds at (0, 0) with zero width and
    /// height. Always returns false.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_setEmpty>
    pub fn set_empty(&mut self) -> bool {
        unsafe { self.native_mut().setEmpty() }
    }

    /// Constructs a rectangular region matching the bounds of `rect`. If `rect` is empty,
    /// constructs an empty region and returns false.
    ///
    /// - `rect` bounds of the constructed region
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_setRect>
    pub fn set_rect(&mut self, rect: impl AsRef<IRect>) -> bool {
        unsafe { self.native_mut().setRect(rect.as_ref().native()) }
    }

    /// Constructs a region as the union of the rectangles in `rects`. If `rects` is empty,
    /// constructs an empty region. Returns false if the constructed region is empty.
    ///
    /// May be faster than repeated calls to `op`.
    ///
    /// - `rects` array of rectangles
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_setRects>
    pub fn set_rects(&mut self, rects: &[IRect]) -> bool {
        unsafe {
            sb::C_SkRegion_setRects(
                self.native_mut(),
                rects.native().as_ptr(),
                rects.len().try_into().unwrap(),
            )
        }
    }

    /// Sets the region to a copy of `region`. Creating a region copy is very efficient and never
    /// allocates memory. Regions are always copied by value from the interface; the underlying
    /// shared pointers are not exposed.
    ///
    /// - `region` region to copy by value
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_setRegion>
    pub fn set_region(&mut self, region: &Region) -> bool {
        unsafe { self.native_mut().setRegion(region.native()) }
    }

    /// Constructs a region to match the outline of `path` within `clip`. Returns false if the
    /// constructed region is empty.
    ///
    /// The constructed region draws the same pixels as `path` through `clip` when anti-aliasing is
    /// disabled.
    ///
    /// - `path` path providing outline
    /// - `clip` region containing path
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_setPath>
    pub fn set_path(&mut self, path: &Path, clip: &Region) -> bool {
        unsafe { self.native_mut().setPath(path.native(), clip.native()) }
    }

    // There is also a trait for intersects() below.

    /// Returns true if the region intersects `rect`. Returns false if either `rect` or the region
    /// is empty, or they do not intersect.
    ///
    /// - `rect` rectangle to intersect
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_intersects>
    pub fn intersects_rect(&self, rect: impl AsRef<IRect>) -> bool {
        unsafe { self.native().intersects(rect.as_ref().native()) }
    }

    /// Returns true if the region intersects `other`. Returns false if either `other` or the region
    /// is empty, or they do not intersect.
    ///
    /// - `other` region to intersect
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_intersects_2>
    pub fn intersects_region(&self, other: &Region) -> bool {
        unsafe { self.native().intersects1(other.native()) }
    }

    // contains() trait below.

    /// Returns true if the point (`point.x`, `point.y`) is inside the region. Returns false if the
    /// region is empty.
    ///
    /// - `point` test point
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_contains>
    pub fn contains_point(&self, point: IPoint) -> bool {
        unsafe { self.native().contains(point.x, point.y) }
    }

    /// Returns true if `rect` is completely inside the region. Returns false if the region or
    /// `rect` is empty.
    ///
    /// - `rect` rectangle to contain
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_contains_2>
    pub fn contains_rect(&self, rect: impl AsRef<IRect>) -> bool {
        unsafe { self.native().contains1(rect.as_ref().native()) }
    }

    /// Returns true if `other` is completely inside the region. Returns false if the region or
    /// `other` is empty.
    ///
    /// - `other` region to contain
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_contains_3>
    pub fn contains_region(&self, other: &Region) -> bool {
        unsafe { self.native().contains2(other.native()) }
    }

    /// Returns true if the region is a single rectangle and contains `r`. May return false even
    /// though the region contains `r`.
    ///
    /// - `r` rectangle to contain
    pub fn quick_contains(&self, r: impl AsRef<IRect>) -> bool {
        let r = r.as_ref();
        unsafe { sb::C_SkRegion_quickContains(self.native(), r.native()) }
    }

    // See also the quick_reject() trait below.

    /// Returns true if the region does not intersect `rect`. Returns true if `rect` is empty or the
    /// region is empty. May return false even though the region does not intersect `rect`.
    ///
    /// - `rect` rectangle to intersect
    pub fn quick_reject_rect(&self, rect: impl AsRef<IRect>) -> bool {
        let rect = rect.as_ref();
        self.is_empty() || rect.is_empty() || !IRect::intersects(self.bounds(), rect)
    }

    /// Returns true if the region does not intersect `rgn`. Returns true if `rgn` is empty or the
    /// region is empty. May return false even though the region does not intersect `rgn`.
    ///
    /// - `rgn` region to intersect
    pub fn quick_reject_region(&self, rgn: &Region) -> bool {
        self.is_empty() || rgn.is_empty() || !IRect::intersects(self.bounds(), rgn.bounds())
    }

    /// Offsets the region by the vector (`d.x`, `d.y`). Has no effect if the region is empty.
    ///
    /// - `d` offset vector
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_translate_2>
    pub fn translate(&mut self, d: impl Into<IVector>) {
        let d = d.into();
        let self_ptr = self.native_mut() as *mut _;
        unsafe { self.native().translate(d.x, d.y, self_ptr) }
    }

    /// Returns a copy of the region offset by the vector (`d.x`, `d.y`).
    ///
    /// - `d` offset vector
    #[must_use]
    pub fn translated(&self, d: impl Into<IVector>) -> Self {
        let mut r = self.clone();
        r.translate(d);
        r
    }

    /// Replaces the region with the result of the region `op` `rect`. Returns true if the replaced
    /// region is not empty.
    ///
    /// - `rect` rectangle operand
    /// - `op` logical operation
    pub fn op_rect(&mut self, rect: impl AsRef<IRect>, op: RegionOp) -> bool {
        let self_ptr = self.native_mut() as *const _;
        unsafe { self.native_mut().op1(self_ptr, rect.as_ref().native(), op) }
    }

    /// Replaces the region with the result of the region `op` `region`. Returns true if the
    /// replaced region is not empty.
    ///
    /// - `region` region operand
    /// - `op` logical operation
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_op_6>
    pub fn op_region(&mut self, region: &Region, op: RegionOp) -> bool {
        let self_ptr = self.native_mut() as *const _;
        unsafe { self.native_mut().op2(self_ptr, region.native(), op) }
    }

    /// Replaces the region with the result of `rect` `op` `region`. Returns true if the replaced
    /// region is not empty.
    ///
    /// - `rect` rectangle operand
    /// - `region` region operand
    /// - `op` logical operation
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_op_4>
    pub fn op_rect_region(
        &mut self,
        rect: impl AsRef<IRect>,
        region: &Region,
        op: RegionOp,
    ) -> bool {
        unsafe {
            self.native_mut()
                .op(rect.as_ref().native(), region.native(), op)
        }
    }

    /// Replaces the region with the result of `region` `op` `rect`. Returns true if the replaced
    /// region is not empty.
    ///
    /// - `region` region operand
    /// - `rect` rectangle operand
    /// - `op` logical operation
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_op_5>
    pub fn op_region_rect(
        &mut self,
        region: &Region,
        rect: impl AsRef<IRect>,
        op: RegionOp,
    ) -> bool {
        unsafe {
            self.native_mut()
                .op1(region.native(), rect.as_ref().native(), op)
        }
    }

    /// Writes the region to `buf`, and returns the number of bytes written.
    ///
    /// - `buf` storage for binary data
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_writeToMemory>
    pub fn write_to_memory(&self, buf: &mut Vec<u8>) {
        unsafe {
            let size = self.native().writeToMemory(ptr::null_mut());
            buf.resize(size, 0);
            let written = self.native().writeToMemory(buf.as_mut_ptr() as _);
            debug_assert!(written == size);
        }
    }

    /// Constructs the region from `buf` of size `buf.len()`. Returns the bytes read. The returned
    /// value will be a multiple of four or zero if the length was too small.
    ///
    /// - `buf` storage for binary data
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_readFromMemory>
    pub fn read_from_memory(&mut self, buf: &[u8]) -> usize {
        unsafe {
            self.native_mut()
                .readFromMemory(buf.as_ptr() as _, buf.len())
        }
    }
}

//
// combine overloads (static)
//

pub trait Combine<A, B>: Sized {
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

#[derive(Clone)]
#[repr(transparent)]
/// Goes through the region one rectangle at a time. For each "strip" of one or more contiguous
/// Y values (scanlines) in ascending order, the iterator returns each rectangle in that strip
/// (from left to right) before advancing to the next strip (which may or may not have a gap).
pub struct Iterator<'a>(SkRegion_Iterator, PhantomData<&'a Region>);

native_transmutable!(SkRegion_Iterator, Iterator<'_>);

impl fmt::Debug for Iterator<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iterator")
            .field("is_done", &self.is_done())
            .field("rect", self.rect())
            .finish()
    }
}

impl<'a> Iterator<'a> {
    /// Initializes an iterator with an empty region. [`Self::is_done()`] on the iterator returns
    /// true. Call [`Self::reset()`] to initialize the iterator at a later time.
    pub fn new_empty() -> Self {
        Iterator::construct(|iterator| unsafe {
            sb::C_SkRegion_Iterator_Construct(iterator);
        })
    }

    /// Sets the iterator to return elements of the region's rectangle array.
    ///
    /// - `region` region to iterate
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_Iterator_copy_const_SkRegion>
    pub fn new(region: &'a Region) -> Iterator<'a> {
        Iterator::from_native_c(unsafe { SkRegion_Iterator::new(region.native()) })
    }

    /// Moves the iterator to the start of the region. Returns true if the region was set;
    /// otherwise, returns false.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_Iterator_rewind>
    pub fn rewind(&mut self) -> bool {
        unsafe { self.native_mut().rewind() }
    }

    /// Resets the iterator, using the new region.
    ///
    /// - `region` region to iterate
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_Iterator_reset>
    pub fn reset(mut self, region: &Region) -> Iterator {
        unsafe {
            self.native_mut().reset(region.native());
            mem::transmute(self)
        }
    }

    /// Returns true if the iterator is pointing to the final rectangle in the region.
    pub fn is_done(&self) -> bool {
        self.native().fDone
    }

    /// Advances the iterator to the next rectangle in the region if it is not done. This moves to
    /// the next rectangle to the right within the current horizontal strip. If the end of the strip
    /// is reached, it automatically advances to the first rectangle in the next strip, skipping any
    /// vertical gaps.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_Iterator_next>
    pub fn next(&mut self) {
        unsafe {
            self.native_mut().next();
        }
    }

    /// Returns the rectangle element in the region. Does not return predictable results if the
    /// region is empty.
    pub fn rect(&self) -> &IRect {
        IRect::from_native_ref(&self.native().fRect)
    }

    /// Returns the region if set; otherwise, returns `None`.
    pub fn rgn(&self) -> Option<&Region> {
        unsafe {
            let r = sb::C_SkRegion_Iterator_rgn(self.native()).into_non_null()?;
            Some(Region::from_native_ref(r.as_ref()))
        }
    }
}

impl iter::Iterator for Iterator<'_> {
    type Item = IRect;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let r = *self.rect();
        Iterator::next(self);
        Some(r)
    }
}

#[test]
fn test_iterator() {
    let r1 = IRect::new(10, 10, 12, 14);
    let r2 = IRect::new(100, 100, 120, 140);
    let mut r = Region::new();
    r.set_rects(&[r1, r2]);
    let rects: Vec<IRect> = Iterator::new(&r).collect();
    assert_eq!(rects.len(), 2);
    assert_eq!(rects[0], r1);
    assert_eq!(rects[1], r2);
}

#[derive(Clone)]
#[repr(transparent)]
/// Returns the sequence of rectangles, sorted along the y-axis, then the x-axis, that make up the
/// region intersected with the specified clip rectangle.
pub struct Cliperator<'a>(SkRegion_Cliperator, PhantomData<&'a Region>);

native_transmutable!(SkRegion_Cliperator, Cliperator<'_>);

impl Drop for Cliperator<'_> {
    fn drop(&mut self) {
        unsafe { sb::C_SkRegion_Cliperator_destruct(self.native_mut()) }
    }
}

impl fmt::Debug for Cliperator<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cliperator")
            .field("is_done", &self.is_done())
            .field("rect", &self.rect())
            .finish()
    }
}

impl<'a> Cliperator<'a> {
    /// Sets the cliperator to return elements of the region's rectangle array within `clip`.
    ///
    /// - `region` region to iterate
    /// - `clip` bounds of iteration
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_Cliperator_const_SkRegion_const_SkIRect>
    pub fn new(region: &'a Region, clip: impl AsRef<IRect>) -> Cliperator<'a> {
        Cliperator::from_native_c(unsafe {
            SkRegion_Cliperator::new(region.native(), clip.as_ref().native())
        })
    }

    /// Returns true if the cliperator is pointing to the final rectangle in the region.
    pub fn is_done(&self) -> bool {
        self.native().fDone
    }

    /// Advances the iterator to the next rectangle in the region contained by the clip.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_Cliperator_next>
    pub fn next(&mut self) {
        unsafe { self.native_mut().next() }
    }

    /// Returns the rectangle element in the region, intersected with the clip passed to
    /// [`Self::new()`]. Does not return predictable results if the region is empty.
    pub fn rect(&self) -> &IRect {
        IRect::from_native_ref(&self.native().fRect)
    }
}

impl iter::Iterator for Cliperator<'_> {
    type Item = IRect;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        let rect = *self.rect();
        self.next();
        Some(rect)
    }
}

#[derive(Clone)]
#[repr(transparent)]
/// Returns the line segment ends within the region that intersect a horizontal line.
pub struct Spanerator<'a>(SkRegion_Spanerator, PhantomData<&'a Region>);

native_transmutable!(SkRegion_Spanerator, Spanerator<'_>);

impl Drop for Spanerator<'_> {
    fn drop(&mut self) {
        unsafe { sb::C_SkRegion_Spanerator_destruct(self.native_mut()) }
    }
}

impl fmt::Debug for Spanerator<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Spanerator").finish()
    }
}

impl<'a> Spanerator<'a> {
    /// Sets the spanerator to return line segments in the region on the scan line.
    ///
    /// - `region` region to iterate
    /// - `y` horizontal line to intersect
    /// - `left` bounds of iteration
    /// - `right` bounds of iteration
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_Spanerator_const_SkRegion_int_int_int>
    pub fn new(region: &'a Region, y: i32, left: i32, right: i32) -> Spanerator<'a> {
        Spanerator::from_native_c(unsafe {
            SkRegion_Spanerator::new(region.native(), y, left, right)
        })
    }
}

impl iter::Iterator for Spanerator<'_> {
    type Item = (i32, i32);

    /// Advances the iterator to the next span intersecting the region within the line segment
    /// provided in the constructor. Returns the `(left, right)` span if an interval was found.
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Region_Spanerator_next>
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut left = 0;
            let mut right = 0;
            self.native_mut()
                .next(&mut left, &mut right)
                .then_some((left, right))
        }
    }
}

#[test]
fn new_clone_drop() {
    let region = Region::new();
    #[allow(clippy::redundant_clone)]
    let _cloned = region.clone();
}

#[test]
fn can_compare() {
    let r1 = Region::new();
    #[allow(clippy::redundant_clone)]
    let r2 = r1.clone();
    assert!(r1 == r2);
}
