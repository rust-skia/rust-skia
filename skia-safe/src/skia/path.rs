use crate::prelude::*;
use crate::skia::{
    Point,
    RRect,
    Rect,
    scalar,
    Vector,
    Data,
    Matrix
};
use skia_bindings::{
    SkPath_AddPathMode,
    SkPath_ArcSize,
    C_SkPath_Equals,
    SkPath_Direction,
    SkPath,
    C_SkPath_destruct,
    SkPath_FillType,
    SkPath_Convexity,
    C_SkPath_ConvertToNonInverseFillType,
    C_SkPath_serialize,
    SkPath_SegmentMask_kLine_SegmentMask,
    SkPath_SegmentMask_kQuad_SegmentMask,
    SkPath_SegmentMask_kConic_SegmentMask,
    SkPath_SegmentMask_kCubic_SegmentMask
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PathDirection {
    CW = SkPath_Direction::kCW_Direction as _,
    CCW = SkPath_Direction::kCCW_Direction as _
}

impl NativeTransmutable<SkPath_Direction> for PathDirection {}
#[test] fn test_path_direction_layout() { PathDirection::test_layout() }

impl Default for PathDirection {
    fn default() -> Self {
        PathDirection::CW
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PathFillType {
    Winding = SkPath_FillType::kWinding_FillType as _,
    EventOdd = SkPath_FillType::kEvenOdd_FillType as _,
    InverseWinding = SkPath_FillType::kInverseWinding_FillType as _,
    InverseEvenOdd = SkPath_FillType::kInverseEvenOdd_FillType as _
}

impl NativeTransmutable<SkPath_FillType> for PathFillType {}
#[test] fn test_path_fill_type_layout() { PathFillType::test_layout() }

impl PathFillType {
    pub fn is_inverse(self) -> bool {
        unsafe { SkPath::IsInverseFillType(self.into_native()) }
    }

    pub fn to_non_inverse(self) -> Self {
        // does not link:
        // unsafe { SkPath::ConvertToNonInverseFillType(self.native()) }
        //     .into_handle()
        Self::from_native(unsafe {
            C_SkPath_ConvertToNonInverseFillType(self.into_native())
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PathConvexity {
    Unknown = SkPath_Convexity::kUnknown_Convexity as _,
    Convex = SkPath_Convexity::kConvex_Convexity as _,
    Concave = SkPath_Convexity::kConcave_Convexity as _
}

impl NativeTransmutable<SkPath_Convexity> for PathConvexity {}
#[test] fn test_path_convexity_layout() { PathConvexity::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PathArcSize {
    Small = SkPath_ArcSize::kSmall_ArcSize as _,
    Large = SkPath_ArcSize::kLarge_ArcSize as _
}

impl NativeTransmutable<SkPath_ArcSize> for PathArcSize {}
#[test] fn test_arc_size_layout() { PathArcSize::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum AddPathMode  {
    Append = SkPath_AddPathMode::kAppend_AddPathMode as _,
    Extend = SkPath_AddPathMode::kExtend_AddPathMode as _
}

impl NativeTransmutable<SkPath_AddPathMode> for AddPathMode {}
#[test] fn test_add_path_mode_layout() { AddPathMode::test_layout() }

bitflags! {
    pub struct PathSegmentMask: u32 {
        const LINE = SkPath_SegmentMask_kLine_SegmentMask as _;
        const QUAD = SkPath_SegmentMask_kQuad_SegmentMask as _;
        const CONIC = SkPath_SegmentMask_kConic_SegmentMask as _;
        const CUBIC = SkPath_SegmentMask_kCubic_SegmentMask as _;
    }
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

impl Default for Handle<SkPath> {
    fn default() -> Self {
        unsafe { SkPath::new() }.into_handle()
    }
}

impl Handle<SkPath> {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_interpolatable(&self, compare: &Path) -> bool {
        unsafe { self.native().isInterpolatable(compare.native()) }
    }

    pub fn interpolate(&self, ending: &Path, weight: scalar) -> Option<Path> {
        let mut out = Path::default();
        unsafe { self.native().interpolate(ending.native(), weight, out.native_mut()) }
            .if_true_some(out)
    }

    pub fn fill_type(&self) -> PathFillType {
        PathFillType::from_native(unsafe {
                self.native().getFillType()
        })
    }

    pub fn set_fill_type(&mut self, ft: PathFillType) -> &mut Self {
        unsafe { self.native_mut().setFillType(ft.into_native()) }
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
        PathConvexity::from_native(unsafe { self.native().getConvexity() })
    }

    pub fn convexity_or_unknown(&self) -> PathConvexity {
        PathConvexity::from_native(unsafe { self.native().getConvexityOrUnknown() })
    }

    pub fn set_convexity(&mut self, convexity: PathConvexity) -> &mut Self {
        unsafe { self.native_mut().setConvexity(convexity.into_native()) }
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

    pub fn is_line_degenerate<P: Into<Point>>(p1: P, p2: P, exact: bool) -> bool {
        unsafe { SkPath::IsLineDegenerate(p1.into().native(), p2.into().native(), exact) }
    }

    pub fn is_quad_degenerate<P: Into<Point>>(p1: P, p2: P, p3: P, exact: bool) -> bool {
        unsafe { SkPath::IsQuadDegenerate(p1.into().native(), p2.into().native(), p3.into().native(), exact) }
    }

    pub fn is_cubic_degenerate<P: Into<Point>>(p1: P, p2: P, p3: P, p4: P, exact: bool) -> bool {
        unsafe { SkPath::IsCubicDegenerate(p1.into().native(), p2.into().native(), p3.into().native(), p4.into().native(), exact) }
    }

    pub fn is_line(&self) -> Option<(Point, Point)> {
        let mut line = [Point::default(); 2];
        unsafe { self.native().isLine(line.native_mut().as_mut_ptr()) }
            .if_true_some((line[0], line[1]))
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

    pub fn move_to<P: Into<Point>>(&mut self, p: P) -> &mut Self {
        unsafe { self.native_mut().moveTo1(p.into().native()); }
        self
    }

    pub fn r_move_to<V: Into<Vector>>(&mut self, d: V) -> &mut Self {
        let d = d.into();
        unsafe { self.native_mut().rMoveTo(d.x, d.y); }
        self
    }

    pub fn line_to<P: Into<Point>>(&mut self, p: P) -> &mut Self {
        unsafe { self.native_mut().lineTo1(p.into().native()); }
        self
    }

    pub fn r_line_to<V: Into<Vector>>(&mut self, d: V) -> &mut Self {
        let d = d.into();
        unsafe { self.native_mut().rLineTo(d.x, d.y); }
        self
    }

    pub fn quad_to<P: Into<Point>>(&mut self, p1: P, p2: P) -> &mut Self {
        unsafe { self.native_mut().quadTo1(p1.into().native(), p2.into().native()) };
        self
    }

    pub fn r_quad_to<V: Into<Vector>>(&mut self, dx1: V, dx2: V) -> &mut Self {
        let (dx1, dx2) = (dx1.into(), dx2.into());
        unsafe { self.native_mut().rQuadTo(dx1.x, dx1.y, dx2.x, dx2.y) };
        self
    }

    pub fn conic_to<P: Into<Point>>(&mut self, p1: P, p2: P, w: scalar) -> &mut Self {
        unsafe { self.native_mut().conicTo1(p1.into().native(), p2.into().native(), w) };
        self
    }

    pub fn r_conic_to<V: Into<Vector>>(&mut self, d1: V, d2: V, w: scalar) -> &mut Self {
        let (d1, d2) = (d1.into(), d2.into());
        unsafe { self.native_mut().rConicTo(d1.x, d1.y, d2.x, d2.y, w) };
        self
    }

    pub fn cubic_to<P: Into<Point>>(&mut self, p1: P, p2: P, p3: P) -> &mut Self {
        unsafe { self.native_mut().cubicTo1(p1.into().native(), p2.into().native(), p3.into().native()) };
        self
    }

    pub fn r_cubic_to<V: Into<Vector>>(&mut self, d1: V, d2: V, d3: V) -> &mut Self {
        let (d1, d2, d3) = (d1.into(), d2.into(), d3.into());
        unsafe { self.native_mut().rCubicTo(d1.x, d1.y, d2.x, d2.y, d3.x, d3.y) };
        self
    }

    pub fn arc_to(&mut self, oval: &Rect, start_angle: scalar, sweep_angle: scalar, force_move_to: bool) -> &mut Self {
        unsafe { self.native_mut().arcTo(oval.native(), start_angle, sweep_angle, force_move_to) };
        self
    }

    pub fn arc_to_tangent<P: Into<Point>>(&mut self, p1: P, p2: P, radius: scalar) -> &mut Self {
        // does not link:
        // unsafe { self.native_mut().arcTo2(*p1.native(), *p2.native(), radius) };
        let (p1, p2) = (p1.into(), p2.into());
        unsafe { self.native_mut().arcTo1(p1.x, p1.y, p2.x, p2.y, radius) };
        self
    }

    pub fn arc_to_rotated<P: Into<Point>>(&mut self, r: P, x_axis_rotate: scalar, large_arc: PathArcSize, sweep: PathDirection, xy: P) -> &mut Self {
        let (r, xy) = (r.into(), xy.into());
        unsafe { self.native_mut().arcTo4(*r.native(), x_axis_rotate, large_arc.into_native(), sweep.into_native(), *xy.native()) };
        self
    }

    pub fn r_arc_to_rotated<P: Into<Point>>(&mut self, r: P, x_axis_rotate: scalar, large_arc: PathArcSize, sweep: PathDirection, xy: P) -> &mut Self {
        let (r, xy) = (r.into(), xy.into());
        unsafe { self.native_mut().rArcTo(r.x, r.y, x_axis_rotate, large_arc.into_native(), sweep.into_native(), xy.x, xy.y) };
        self
    }

    pub fn close(&mut self) -> &mut Self {
        unsafe { self.native_mut().close(); }
        self
    }

    pub fn convert_conic_to_quads<P: Into<Point>>(p0: P, p1: P, p2: P, w: scalar, pts: &mut [Point], pow2: usize) -> Option<usize> {
        let (p0, p1, p2) = (p0.into(), p1.into(), p2.into());
        let max_pts_storage = 1 + 2 * (1 << pow2);
        if max_pts_storage <= pts.len() {
            Some(unsafe {
                SkPath::ConvertConicToQuads(p0.native(), p1.native(), p2.native(), w, pts.native_mut().as_mut_ptr(), pow2.try_into().unwrap())
                    .try_into().unwrap()
            })
        } else {
            None
        }
    }

    // TODO: return type is probably worth a struct.
    pub fn is_rect(&self) -> Option<(Rect, bool, PathDirection)> {
        let mut rect = Rect::default();
        let mut is_closed = Default::default();
        let mut direction = PathDirection::default();

        unsafe { self.native().isRect(rect.native_mut(), &mut is_closed, direction.native_mut()) }
            .if_true_some((rect, is_closed, direction))
    }

    // TODO: return type is probably worth a struct.
    pub fn is_nested_fill_rects(&self) -> Option<([Rect;2], [PathDirection;2])> {
        let mut rects = [Rect::default(); 2];
        let mut dirs = [PathDirection::CW; 2];
        unsafe { self.native().isNestedFillRects(rects.native_mut().as_mut_ptr(), dirs.native_mut().as_mut_ptr()) }
            .if_true_some((rects, dirs))
    }

    pub fn add_rect(&mut self, rect: &Rect, dir_start: Option<(PathDirection, usize)>) -> &mut Self {
        let dir = dir_start.map(|ds| ds.0).unwrap_or_default();
        let start = dir_start.map(|ds| ds.1).unwrap_or_default();
        unsafe {
            self.native_mut().addRect1(rect.native(), dir.into_native(), start.try_into().unwrap())
        };
        self
    }

    pub fn add_oval(&mut self, oval: &Rect, dir_start: Option<(PathDirection, usize)>) -> &mut Self {
        let dir = dir_start.map(|ds| ds.0).unwrap_or_default();
        let start = dir_start.map(|ds| ds.1).unwrap_or_default();
        unsafe {
            self.native_mut().addOval1(oval.native(), dir.into_native(), start.try_into().unwrap())
        };
        self
    }

    pub fn add_circle<P: Into<Point>>(&mut self, p: P, radius: scalar, dir: Option<PathDirection>) -> &mut Self {
        let p = p.into();
        let dir = dir.unwrap_or_default();
        unsafe {
            self.native_mut().addCircle(p.x, p.y, radius, dir.into_native())
        };
        self
    }

    pub fn add_arc(&mut self, oval: &Rect, start_angle: scalar, sweep_angle: scalar) -> &mut Self {
        unsafe {
            self.native_mut().addArc(oval.native(), start_angle, sweep_angle)
        };
        self
    }

    // decided to use the simpler variant of the two, if more radii need to be specified,
    // add_rrect can be used.
    pub fn add_round_rect(&mut self, rect: &Rect, (rx, ry): (scalar, scalar), dir: Option<PathDirection>) -> &mut Self {
        let dir = dir.unwrap_or_default();
        unsafe {
            self.native_mut().addRoundRect(rect.native(), rx, ry, dir.into_native())
        };
        self
    }

    pub fn add_rrect(&mut self, rrect: &RRect, dir_start: Option<(PathDirection, usize)>) -> &mut Self {
        let dir = dir_start.map(|ds| ds.0).unwrap_or_default();
        let start = dir_start.map(|ds| ds.1).unwrap_or_default();
        unsafe {
            self.native_mut().addRRect1(rrect.native(), dir.into_native(), start.try_into().unwrap())
        };
        self
    }

    pub fn add_poly(&mut self, pts: &[Point], close: bool) -> &mut Self {
        unsafe {
            self.native_mut().addPoly(pts.native().as_ptr(), pts.len().try_into().unwrap(), close)
        };
        self
    }

    // TODO: mode
    pub fn add_path<V: Into<Vector>>(&mut self, src: &Path, d: V, mode: AddPathMode) -> &mut Self {
        let d = d.into();
        unsafe {
            self.native_mut().addPath(src.native(), d.x, d.y, mode.into_native())
        };
        self
    }

    // TODO: mode
    pub fn add_path_matrix(&mut self, src: &Path, matrix: &Matrix, mode: AddPathMode) -> &mut Self {
        unsafe {
            self.native_mut().addPath2(src.native(), matrix.native(), mode.into_native())
        };
        self
    }

    pub fn reverse_add_path(&mut self, src: &Path) -> &mut Self {
        unsafe {
            self.native_mut().reverseAddPath(src.native())
        };
        self
    }

    pub fn offset<V: Into<Vector>>(&mut self, d: V) -> &mut Self {
        let d = d.into();
        unsafe {
            self.native_mut().offset1(d.x, d.y)
        };
        self
    }

    #[must_use]
    pub fn with_offset<V: Into<Vector>>(&self, d: V) -> Path {
        let d = d.into();
        let mut path = Path::default();
        unsafe {
            self.native().offset(d.x, d.y, path.native_mut())
        };
        path
    }

    pub fn transform(&mut self, matrix: &Matrix) -> &mut Self {
        unsafe {
            self.native_mut().transform1(matrix.native())
        };
        self
    }

    #[must_use]
    pub fn with_transform(&self, matrix: &Matrix) -> Path {
        let mut path = Path::default();
        unsafe {
            self.native().transform(matrix.native(), path.native_mut())
        };
        path
    }

    pub fn last_pt(&self) -> Option<Point> {
        let mut last_pt = Point::default();
        unsafe {
            self.native().getLastPt(last_pt.native_mut())
        }.if_true_some(last_pt)
    }

    pub fn set_last_pt<P: Into<Point>>(&mut self, p: P) -> &mut Self {
        let p = p.into();
        unsafe {
            // does not link:
            // self.native_mut().setLastPt1(p.native())
            self.native_mut().setLastPt(p.x, p.y)
        };
        self
    }

    pub fn segment_masks(&self) -> PathSegmentMask {
        PathSegmentMask::from_bits_truncate(unsafe {
            self.native().getSegmentMasks()
        })
    }

    // TODO: Iter
    // TODO: RawIter

    pub fn contains<P: Into<Point>>(&self, p: P) -> bool {
        let p = p.into();
        unsafe { self.native().contains(p.x, p.y) }
    }

    pub fn serialize(&self) -> Data {
        Data::from_ptr(unsafe {
            C_SkPath_serialize(self.native())
        }).unwrap()
    }

    pub fn deserialize(data: &Data) -> Option<Path> {
        let mut path = Path::default();
        let bytes = data.bytes();
        unsafe {
            path.native_mut().readFromMemory(
                bytes.as_ptr() as _,
                bytes.len()) > 0
        }.if_true_some(path)
    }
}
