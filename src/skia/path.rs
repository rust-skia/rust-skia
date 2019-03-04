use std::mem;
use crate::prelude::*;
use crate::skia::{
    Point,
    RRect,
    Rect,
    scalar,
    Vector
};
use rust_skia::{
    C_SkPath_Equals,
    SkPath_Direction,
    SkPath,
    C_SkPath_destruct,
    SkPath_FillType,
    SkPath_Convexity
};

pub type CountourDirection = EnumHandle<SkPath_Direction>;

impl EnumHandle<SkPath_Direction> {
    pub const CW: Self = Self(SkPath_Direction::kCW_Direction);
    pub const CCW: Self = Self(SkPath_Direction::kCCW_Direction);
}

pub type PathFillType = EnumHandle<SkPath_FillType>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPath_FillType> {
    pub const Winding: Self = Self(SkPath_FillType::kWinding_FillType);
    pub const EventOdd: Self = Self(SkPath_FillType::kEvenOdd_FillType);
    pub const InverseWinding: Self = Self(SkPath_FillType::kInverseWinding_FillType);
    pub const InverseEvenOdd: Self = Self(SkPath_FillType::kInverseEvenOdd_FillType);
}

pub type PathConvexity = EnumHandle<SkPath_Convexity>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPath_Convexity> {
    pub const Unknown: Self = Self(SkPath_Convexity::kUnknown_Convexity);
    pub const Convex: Self = Self(SkPath_Convexity::kConvex_Convexity);
    pub const Concave: Self = Self(SkPath_Convexity::kConcave_Convexity);
}

pub type Path = Handle<SkPath>;

impl NativeDrop for SkPath {
    fn drop(&mut self) {
        unsafe { C_SkPath_destruct(self) }
    }
}

impl NativeClone for SkPath {
    fn clone(&self) -> Self {
        unsafe { SkPath::new1(self) }
    }
}

impl NativePartialEq for SkPath {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkPath_Equals(self, rhs) }
    }
}

impl Handle<SkPath> {

    pub fn new() -> Path {
        unsafe { SkPath::new() }.into_handle()
    }

    pub fn is_interpolatable(&self, compare: &Path) -> bool {
        unsafe { self.native().isInterpolatable(compare.native()) }
    }

    pub fn interpolate(&self, ending: &Path, weight: scalar) -> Option<Path> {
        let mut out = Path::new();
        unsafe { self.native().interpolate(ending.native(), weight, out.native_mut()) }
            .if_true_some(out)
    }

    pub fn fill_type(&self) -> PathFillType {
        PathFillType::from_native(unsafe {
                self.native().getFillType()
        })
    }

    pub fn set_fill_type(&mut self, ft: PathFillType) -> &mut Self {
        unsafe { self.native_mut().setFillType(ft.native()) }
        self
    }

    pub fn is_inverse_fill_type(&self) -> bool {
        unsafe { self.native().isInverseFillType() }
    }

    pub fn toggle_inverse_fill_type(&mut self) -> &mut Self {
        unsafe { self.native_mut().toggleInverseFillType() }
        self
    }

    pub fn convexity(&self) -> PathConvexity {
        unsafe { self.native().getConvexity() }
            .into_handle()
    }

    pub fn convexity_or_unknown(&self) -> PathConvexity {
        unsafe { self.native().getConvexityOrUnknown() }
            .into_handle()
    }

    pub fn set_convexity(&mut self, convexity: PathConvexity) -> &mut Self {
        unsafe { self.native_mut().setConvexity(convexity.native()) }
        self
    }

    pub fn is_convex(&self) -> bool {
        unsafe { self.native().isConvex() }
    }

    pub fn is_oval(&self) -> Option<Rect> {
        let mut bounds = Rect::default();
        unsafe { self.native().isOval(bounds.native_mut()) }
            .if_true_some(bounds)
    }

    pub fn is_rrect(&self) -> Option<RRect> {
        let mut rrect = RRect::default();
        unsafe { self.native().isRRect(rrect.native_mut()) }
            .if_true_some(rrect)
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() };
        self
    }

    pub fn rewind(&mut self) -> &mut Self {
        unsafe { self.native_mut().rewind() };
        self
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.native().isEmpty() }
    }

    pub fn is_last_contour_closed(&self) -> bool {
        unsafe { self.native().isLastContourClosed() }
    }

    pub fn is_finite(&self) -> bool {
        unsafe { self.native().isFinite() }
    }

    pub fn is_volatile(&self) -> bool {
        unsafe { self.native().isVolatile() }
    }

    pub fn set_is_volatile(&mut self, is_volatile: bool) -> &mut Self {
        unsafe { self.native_mut().setIsVolatile(is_volatile) }
        self
    }

    pub fn is_line_degenerate(p1: Point, p2: Point, exact: bool) -> bool {
        unsafe { SkPath::IsLineDegenerate(p1.native(), p2.native(), exact) }
    }

    pub fn is_quad_degenerate(p1: Point, p2: Point, p3: Point, exact: bool) -> bool {
        unsafe { SkPath::IsQuadDegenerate(p1.native(), p2.native(), p3.native(), exact) }
    }

    pub fn is_cubic_degenerate(p1: Point, p2: Point, p3: Point, p4: Point, exact: bool) -> bool {
        unsafe { SkPath::IsCubicDegenerate(p1.native(), p2.native(), p3.native(), p4.native(), exact) }
    }

    pub fn is_line(&self) -> Option<[Point; 2]> {
        let mut line = [Point::default(); 2];
        unsafe { self.native().isLine(line.native_mut().as_mut_ptr()) }
            .if_true_some(line)
    }

    pub fn count_points(&self) -> usize {
        unsafe { self.native().countPoints().try_into().unwrap() }
    }

    pub fn point(&self, index: usize) -> Point {
        Point::from_native(unsafe {
            self.native().getPoint(index.try_into().unwrap())
        })
    }

    pub fn points(&self, points: &mut [Point]) -> usize {
        unsafe { self.native().getPoints(
            points.native_mut().as_mut_ptr(),
            points.len().try_into().unwrap())
        }.try_into().unwrap()
    }

    pub fn count_verbs(&self) -> usize {
        unsafe { self.native().countVerbs() }.try_into().unwrap()
    }

    pub fn verbs(&self, verbs: &mut [u8]) -> usize {
        unsafe { self.native().getVerbs(
            verbs.as_mut_ptr(),
            verbs.len().try_into().unwrap())
        }.try_into().unwrap()
    }

    pub fn swap(&mut self, other: &mut Path) -> &mut Self {
        unsafe { self.native_mut().swap(other.native_mut()) }
        self
    }

    pub fn bounds(&self) -> Rect {
        Rect::from_native(unsafe {
            *self.native().getBounds()
        })
    }

    pub fn update_bounds_cache(&mut self) -> &mut Self {
        unsafe { self.native_mut().updateBoundsCache() }
        self
    }

    pub fn compute_tight_bounds(&self) -> Rect {
        Rect::from_native(unsafe {
            self.native().computeTightBounds()
        })
    }

    pub fn conservatively_contains_rect(&self, rect: &Rect) -> bool {
        unsafe { self.native().conservativelyContainsRect(rect.native()) }
    }

    pub fn inc_reserve(&mut self, extra_pt_count: usize) -> &mut Self {
        unsafe { self.native_mut().incReserve(extra_pt_count.try_into().unwrap()) }
        self
    }

    pub fn shrink_to_fit(&mut self) -> &mut Self {
        unsafe { self.native_mut().shrinkToFit() }
        self
    }

    pub fn move_to(&mut self, p: Point) -> &mut Self {
        unsafe { self.native_mut().moveTo1(p.native()); }
        self
    }

    pub fn r_move_to(&mut self, d: Vector) -> &mut Self {
        unsafe { self.native_mut().rMoveTo(d.x, d.y); }
        self
    }

    pub fn line_to(&mut self, p: Point) -> &mut Self {
        unsafe { self.native_mut().lineTo1(p.native()); }
        self
    }

    pub fn r_line_to(&mut self, p: Vector) -> &mut Self {
        unsafe { self.native_mut().rLineTo(p.x, p.y); }
        self
    }

    pub fn quad_to(&mut self, p1: Point, p2: Point) -> &mut Self {
        unsafe { self.native_mut().quadTo1(p1.native(), p2.native()) };
        self
    }

    pub fn r_quad_to(&mut self, dx1: Vector, dx2: Vector) -> &mut Self {
        unsafe { self.native_mut().rQuadTo(dx1.x, dx1.y, dx2.x, dx2.y) };
        self
    }

    pub fn conic_to(&mut self, p1: Point, p2: Point, w: scalar) -> &mut Self {
        unsafe { self.native_mut().conicTo1(p1.native(), p2.native(), w) };
        self
    }

    pub fn r_conic_to(&mut self, d1: Vector, d2: Vector, w: scalar) -> &mut Self {
        unsafe { self.native_mut().rConicTo(d1.x, d1.y, d2.x, d2.y, w) };
        self
    }

    pub fn cubic_to(&mut self, p1: Point, p2: Point, p3: Point) -> &mut Self {
        unsafe { self.native_mut().cubicTo1(p1.native(), p2.native(), p3.native()) };
        self
    }

    pub fn r_cubic_to(&mut self, d1: Vector, d2: Vector, d3: Vector) -> &mut Self {
        unsafe { self.native_mut().rCubicTo(d1.x, d1.y, d2.x, d2.y, d3.x, d3.y) };
        self
    }

    /*
    pub fn arc_to(&mut self, oval: &Rect, start_angle: scalar, sweep_angle: scalar, force_move_to: bool) -> &mut Self {
        unsafe { self.native_mut().arcTo(d1.x, d1.y, d2.x, d2.y, d3.x, d3.y) };

    }
    */

    pub fn close(&mut self) -> &mut Self {
        unsafe { self.native_mut().close(); }
        self
    }
}
