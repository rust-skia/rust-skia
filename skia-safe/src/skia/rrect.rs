use crate::prelude::*;
use skia_bindings::{
    C_SkRRect_equals,
    SkRRect,
    SkRRect_Type,
    SkRRect_Corner,
};
use crate::skia::{
    Rect,
    Vector,
    Matrix,
    scalar
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum RRectType {
    Empty = SkRRect_Type::kEmpty_Type as _,
    Rect = SkRRect_Type::kRect_Type as _,
    Oval = SkRRect_Type::kOval_Type as _,
    Simple = SkRRect_Type::kSimple_Type as _,
    NinePatch = SkRRect_Type::kNinePatch_Type as _,
    Complex = SkRRect_Type::kComplex_Type as _
}

impl NativeTransmutable<SkRRect_Type> for RRectType {}
#[test] fn test_rrect_type_layout() { RRectType::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum RRectCorner {
    UpperLeft = SkRRect_Corner::kUpperLeft_Corner as _,
    UpperRight = SkRRect_Corner::kUpperRight_Corner as _,
    LowerRight = SkRRect_Corner::kLowerRight_Corner as _,
    LowerLeft = SkRRect_Corner::kLowerLeft_Corner as _
}

impl NativeTransmutable<SkRRect_Corner> for RRectCorner {}
#[test] fn test_rrect_corner_layout() { RRectCorner::test_layout() }

pub type RRect = ValueHandle<SkRRect>;

impl NativePartialEq for SkRRect {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkRRect_equals(self, rhs) }
    }
}

impl Default for ValueHandle<SkRRect> {
    fn default() -> Self {
        // SkRRect::MakeEmpty does not link, so we use new().
        unsafe { SkRRect::new() }
            .into_handle()
    }
}

impl ValueHandle<SkRRect> {

    pub fn get_type(&self) -> RRectType {
        RRectType::from_native(unsafe { self.native().getType() })
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.native().isEmpty() }
    }

    pub fn is_rect(&self) -> bool {
        unsafe { self.native().isRect() }
    }

    pub fn is_oval(&self) -> bool {
        unsafe { self.native().isOval() }
    }

    pub fn is_simple(&self) -> bool {
        unsafe { self.native().isSimple() }
    }

    pub fn is_nine_patch(&self) -> bool {
        unsafe { self.native().isNinePatch() }
    }

    pub fn is_complex(&self) -> bool {
        unsafe { self.native().isComplex() }
    }

    pub fn width(&self) -> scalar {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> scalar {
        unsafe { self.native().height() }
    }

    pub fn simple_radii(&self) -> Vector {
        Vector::from_native(unsafe { self.native().getSimpleRadii() })
    }

    pub fn new_rect<R: AsRef<Rect>>(rect: R) -> Self {
        unsafe { SkRRect::MakeRect(rect.as_ref().native()) }
            .into_handle()
    }

    pub fn new_oval<R: AsRef<Rect>>(oval: R) -> Self {
        unsafe { SkRRect::MakeOval(oval.as_ref().native()) }
            .into_handle()
    }

    pub fn new_rect_xy<R: AsRef<Rect>>(rect: R, x_rad: scalar, y_rad: scalar) -> Self {
        unsafe { SkRRect::MakeRectXY(rect.as_ref().native(), x_rad, y_rad) }
            .into_handle()
    }

    pub fn new_nine_patch<R: AsRef<Rect>>(rect: R, left_rad: scalar, top_rad: scalar, right_rad: scalar, bottom_rad: scalar) -> Self {
        let mut r = Self::default();
        unsafe {
            r.native_mut()
                .setNinePatch(
                    rect.as_ref().native(),
                    left_rad, top_rad, right_rad, bottom_rad)
        }
        r
    }

    pub fn new_rect_radii<R: AsRef<Rect>>(rect: R, radii: &[Vector; 4]) -> Self {
        let mut r = Self::default();
        unsafe {
            r.native_mut()
                .setRectRadii(rect.as_ref().native(), radii.native().as_ptr())
        }
        r
    }

    pub fn rect(&self) -> Rect {
        Rect::from_native(unsafe { *self.native().rect() })
    }

    pub fn radii(&self, corner: RRectCorner) -> Vector {
        Vector::from_native(unsafe {
            self.native().radii(corner.native().to_owned())
        })
    }

    pub fn bounds(&self) -> Rect {
        Rect::from_native(unsafe {
            *self.native().getBounds()
        })
    }

    #[must_use]
    pub fn inset<V: Into<Vector>>(&self, delta: V) -> Self {
        let delta = delta.into();
        // inset1 does not link.
        let mut r = Self::default();
        unsafe { self.native().inset(delta.x, delta.y, r.native_mut()) };
        r
    }


    #[must_use]
    pub fn outset<V: Into<Vector>>(&self, delta: V) -> Self {
        // outset and outset1 does not link.
        self.inset(-delta.into())
    }

    #[must_use]
    pub fn offset<V: Into<Vector>>(&self, delta: V) -> Self {
        let delta = delta.into();
        // makeOffset and offset does not link.
        let mut copied = *self;
        unsafe { copied.native_mut().fRect.offset(delta.x, delta.y) }
        copied
    }

    pub fn contains<R: AsRef<Rect>>(&self, rect: R) -> bool {
        unsafe { self.native().contains(rect.as_ref().native()) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }

    #[must_use]
    pub fn transform(&self, matrix: &Matrix) -> Option<Self> {
        let mut r = Self::default();
        unsafe { self.native().transform(matrix.native(), r.native_mut()) }
            .if_true_some(r)
    }
}
