use crate::prelude::*;
use crate::skia::{
    scalar,
    ISize,
    IPoint,
    Point,
    Size,
    IVector,
    Contains,
    Vector
};
use skia_bindings::{
    SkIRect,
    SkRect,
};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct IRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32
}

impl NativeTransmutable<SkIRect> for IRect {}

#[test]
fn test_irect_layout() {
    IRect::test_layout();
}

impl IRect {

    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> IRect {
        IRect { left, top, right, bottom }
    }

    pub fn from_size(size: ISize) -> IRect {
        Self::new(0, 0, size.width, size.height)
    }

    pub fn left_top(&self) -> IPoint {
        (self.left, self.top).into()
    }

    pub fn x(&self) -> i32 {
        self.left
    }

    pub fn y(&self) -> i32 {
        self.top
    }

    pub fn width(&self) -> i32 {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { self.native().height() }
    }

    pub fn size(&self) -> ISize {
        (self.width(), self.height()).into()
    }

    pub fn width_64(&self) -> i64 {
        unsafe { self.native().width64() }
    }

    pub fn height_64(&self) -> i64 {
        unsafe { self.native().height64() }
    }

    pub fn is_empty_64(&self) -> bool {
        unsafe { self.native().isEmpty64() }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.native().isEmpty() }
    }

    #[warn(unused)]
    pub fn with_offset(&self, delta: IVector) -> Self {
        let cloned = *self;
        Self::from_native(unsafe {
            cloned.native().makeOffset(delta.x, delta.y)
        })
    }

    #[warn(unused)]
    pub fn with_inset(&self, delta: IVector) -> Self {
        /* does not link:
        Self::from_native(unsafe {
            cloned.native().makeInset(delta.x, delta.y)
        }) */
        self.with_outset(-delta)
    }

    #[warn(unused)]
    pub fn with_outset(&self, delta: IVector) -> Self {
        Self::from_native(unsafe {
            self.native().makeOutset(delta.x, delta.y)
        })
    }

    #[warn(unused)]
    pub fn with_offset_to(&self, new_x: i32, new_y: i32) -> Self {
        let mut copied = *self;
        unsafe {
            copied.native_mut().offsetTo(new_x, new_y)
        }
        copied
    }

    #[warn(unused)]
    pub fn with_adjustment(&self, d_l: i32, d_t: i32, d_r: i32, d_b: i32) -> Self {
        let mut copied = *self;
        unsafe {
            copied.native_mut().adjust(d_l, d_t, d_r, d_b)
        }
        copied
    }

    pub fn contains_no_empty_check(&self, r: &Self) -> bool {
        unsafe { self.native().containsNoEmptyCheck1(r.native()) }
    }

    pub fn intersect(a: &Self, b: &Self) -> Option<Self> {
        let mut intersection = Self::default();
        unsafe { intersection.native_mut().intersect1(a.native(), b.native())}
            .if_true_some(intersection)
    }

    pub fn intersect_no_empty_check(a: &Self, b: &Self) -> Option<Self> {
        let mut intersection = Self::default();
        unsafe { intersection.native_mut().intersectNoEmptyCheck(a.native(), b.native())}
            .if_true_some(intersection)
    }

    pub fn intersects(a: &Self, b: &Self) -> bool {
        unsafe { SkIRect::Intersects(a.native(), b.native()) }
    }

    pub fn intersects_no_empty_check(a: &Self, b: &Self) -> bool {
        unsafe { SkIRect::IntersectsNoEmptyCheck(a.native(), b.native()) }
    }

    pub fn join(a: &Self, b: &Self) -> Self {
        let mut copied = *a;
        unsafe { copied.native_mut().join1(b.native()) }
        copied
    }

    #[warn(unused)]
    pub fn sorted(&self) -> Self {
        let mut copied = *self;
        // makeSorted does not link:
        // IRect::from_native(unsafe { self.native().makeSorted() })
        unsafe { copied.native_mut().sort() }
        copied
    }
}

impl Contains<IPoint> for IRect {
    fn contains(&self, other: IPoint) -> bool {
        unsafe { self.native().contains(other.x, other.y) }
    }
}

impl Contains<&IRect> for IRect {
    fn contains(&self, other: &IRect) -> bool {
        unsafe { self.native().contains2(other.native()) }
    }
}

impl Contains<&Rect> for IRect {
    fn contains(&self, other: &Rect) -> bool {
        unsafe { self.native().contains3(other.native()) }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rect {
    pub left: scalar,
    pub top: scalar,
    pub right: scalar,
    pub bottom: scalar
}

impl NativeTransmutable<SkRect> for Rect {}

#[test]
fn test_rect_layout() {
    Rect::test_layout();
}

impl Rect {
    pub fn new(left: scalar, top: scalar, right: scalar, bottom: scalar) -> Self {
        Self { left, top, right, bottom }
    }

    pub fn from_size(size: Size) -> Self {
        (Point::default(), size).into()
    }

    // TODO: do we need that?
    pub fn from_isize(size: ISize) -> Rect {
        Self::from_size(size.into())
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.native().isEmpty() }
    }

    pub fn is_sorted(&self) -> bool {
        unsafe { self.native().isSorted() }
    }

    pub fn is_finite(&self) -> bool {
        unsafe { self.native().isFinite() }
    }

    pub fn left_top(&self) -> Point {
        (self.left, self.top).into()
    }

    pub fn x(&self) -> scalar {
        self.left
    }

    pub fn y(&self) -> scalar {
        self.top
    }

    pub fn size(&self) -> Size {
        (self.width(), self.height()).into()
    }

    pub fn width(&self) -> scalar {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> scalar {
        unsafe { self.native().height() }
    }

    pub fn center(&self) -> Point {
        (self.center_x(), self.center_y()).into()
    }

    pub fn center_x(&self) -> scalar {
        unsafe {self.native().centerX() }
    }

    pub fn center_y(&self) -> scalar {
        unsafe {self.native().centerY() }
    }

    pub fn to_quad(&self) -> [Point; 4] {
        let mut quad = [Point::default(); 4];
        unsafe { self.native().toQuad(quad.native_mut().as_mut_ptr()) }
        quad
    }

    pub fn from_bounds(points: &[Point]) -> Option<Self> {
        let mut r = Self::default();
        unsafe { r.native_mut().setBoundsCheck(points.native().as_ptr(), points.len().try_into().unwrap()) }
            .if_true_some(r)
    }

    pub fn with_offset(&self, d: Vector) -> Self {
        Self::new(self.left + d.x, self.top + d.y, self.right + d.x, self.bottom + d.y)
    }

    pub fn with_inset(&self, d: Vector) -> Self {
        Self::new(self.left + d.x, self.top + d.y, self.right - d.x, self.bottom - d.y)
    }

    pub fn with_outset(&self, d: Vector) -> Self {
        Self::new(self.left - d.x, self.top - d.y, self.right + d.x, self.bottom + d.y)
    }

    pub fn with_offset_to(&self, new_x: scalar, new_y: scalar) -> Self {
        // does not link:
        // let mut r = self.clone();
        // unsafe { r.native_mut().offsetTo(new_x, new_y) };
        Self::new(new_x, new_y, new_x - self.left, new_y - self.top)
    }

    pub fn intersect(a: &Rect, b: &Rect) -> Option<Rect> {
        let mut intersection = Rect::default();
        unsafe { intersection.native_mut().intersect2(a.native(), b.native()) }
            .if_true_some(intersection)
    }

    pub fn intersects(a: &Rect, b: &Rect) -> bool {
        unsafe { SkRect::Intersects(a.native(), b.native()) }
    }

    pub fn join(a: &Rect, b: &Rect) -> Rect {
        let mut joined = *a;
        unsafe { joined.native_mut().join1(b.native()) }
        joined
    }

    #[warn(unused)]
    pub fn round(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { self.native().round(r.native_mut()) };
        r
    }

    #[warn(unused)]
    pub fn round_in(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { self.native().roundIn(r.native_mut()) };
        r
    }

    #[warn(unused)]
    pub fn sorted(&self) -> Rect {
        Rect::from_native(unsafe { self.native().makeSorted() })
    }

    pub fn as_scalars(&self) -> &[scalar; 4] {
        unsafe { transmute_ref(self) }
    }
}

impl Contains<Point> for Rect {
    fn contains(&self, other: Point) -> bool {
        // does not link:
        // unsafe { self.native().contains(other.x, other.y) }
        other.x >= self.left &&
        other.x < self.right &&
        other.y >= self.top &&
        other.y < self.bottom
    }
}

impl Contains<Rect> for Rect {
    fn contains(&self, other: Rect) -> bool {
        unsafe { self.native().contains1(other.native()) }
    }
}

impl Contains<IRect> for Rect {
    fn contains(&self, other: IRect) -> bool {
        unsafe { self.native().contains2(other.native()) }
    }
}

#[test]
fn contains_overloads_compile() {
    let r = Rect::default();
    r.contains(Point::default());
    r.contains(Rect::default());
    r.contains(IRect::default());
}

pub trait RoundOut<R> {
    fn round_out(&self) -> R;
}

impl RoundOut<IRect> for Rect {
    fn round_out(&self) -> IRect {
        let mut r = IRect::default();
        unsafe { self.native().roundOut(r.native_mut()) };
        r
    }
}

impl RoundOut<Rect> for Rect {
    fn round_out(&self) -> Rect {
        let mut r = Rect::default();
        unsafe { self.native().roundOut1(r.native_mut()) };
        r
    }
}

//
// From
//

impl From<(IPoint, ISize)> for IRect {
    fn from((point, size): (IPoint, ISize)) -> Self {
        IRect::new(point.x, point.y, point.x + size.width, point.y + size.height)
    }
}

impl From<(Point, Size)> for Rect {
    fn from((point, size): (Point, Size)) -> Self {
        Rect::new(point.x, point.y, point.x + size.width, point.y + size.height)
    }
}

impl From<IRect> for Rect {
    fn from(source: IRect) -> Self {
        Self::new(source.left as _, source.top as _, source.right as _, source.bottom as _)
    }
}
