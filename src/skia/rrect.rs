use crate::prelude::*;
use rust_skia::{
    C_SkRRect_equals,
    SkRRect,
    C_SkRRect_not_equals,
    SkRRect_Type,
    SkRRect_Corner,
    SkVector
};
use crate::skia::{
    Rect,
    Vector,
    Matrix,
    scalar
};

pub type RRectType = EnumHandle<SkRRect_Type>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkRRect_Type> {
    pub const Empty: Self = Self(SkRRect_Type::kEmpty_Type);
    pub const Rect: Self = Self(SkRRect_Type::kRect_Type);
    pub const Oval: Self = Self(SkRRect_Type::kOval_Type);
    pub const Simple: Self = Self(SkRRect_Type::kSimple_Type);
    pub const NinePatch: Self = Self(SkRRect_Type::kNinePatch_Type);
    pub const Complex: Self = Self(SkRRect_Type::kComplex_Type);
}

pub type RRectCorner = EnumHandle<SkRRect_Corner>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkRRect_Corner> {
    pub const UpperLeft: Self = Self(SkRRect_Corner::kUpperLeft_Corner);
    pub const UpperRight: Self = Self(SkRRect_Corner::kUpperRight_Corner);
    pub const LowerRight: Self = Self(SkRRect_Corner::kLowerRight_Corner);
    pub const LowerLeft: Self = Self(SkRRect_Corner::kLowerLeft_Corner);
}

pub type RRect = ValueHandle<SkRRect>;

impl NativePartialEq for RRect {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkRRect_equals(self.native(), rhs.native()) }
    }

    fn ne(&self, rhs: &Self) -> bool {
        unsafe { C_SkRRect_not_equals(self.native(), rhs.native()) }
    }
}

impl Default for RRect {
    fn default() -> Self {
        // SkRRect::MakeEmpty does not link, so we use new().
        unsafe { SkRRect::new() }
            .into_handle()
    }
}

impl RRect {

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

    pub fn from_rect(rect: &Rect) -> Self {
        unsafe { SkRRect::MakeRect(&rect.into_native()) }
            .into_handle()
    }

    pub fn from_oval(oval: &Rect) -> Self {
        unsafe { SkRRect::MakeOval(&oval.into_native()) }
            .into_handle()
    }

    pub fn from_rect_xy(rect: &Rect, x_rad: scalar, y_rad: scalar) -> Self {
        unsafe { SkRRect::MakeRectXY(&rect.into_native(), x_rad, y_rad) }
            .into_handle()
    }

    pub fn from_nine_patch(rect: &Rect, left_rad: scalar, top_rad: scalar, right_rad: scalar, bottom_rad: scalar) -> Self {
        let mut r = Self::default();
        unsafe {
            r.native_mut()
                .setNinePatch(
                    &rect.into_native(),
                    left_rad, top_rad, right_rad, bottom_rad)
        }
        r
    }

    pub fn from_rect_radii(rect: &Rect, radii: &[Vector; 4]) -> Self {
        let mut r = Self::default();
        unsafe {
            r.native_mut()
                .setRectRadii(rect.native(), radii.native().as_ptr())
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

    // TODO: use Vector for dx, dy?
    #[warn(unused)]
    pub fn inset(&self, dx: scalar, dy: scalar) -> Self {
        // inset1 does not link.
        let mut r = Self::default();
        unsafe { self.native().inset(dx, dy, r.native_mut()) };
        r
    }


    // TODO: use Vector for dx, dy?
    #[warn(unused)]
    pub fn outset(&self, dx: scalar, dy: scalar) -> Self {
        // outset and outset1 does not link.
        self.inset(-dx, -dy)
    }

    // TODO: use Vector for dx, dy?
    #[warn(unused)]
    pub fn offset(&self, dx: scalar, dy: scalar) -> Self {
        // makeOffset and offset does not link.
        let mut cloned = self.clone();
        unsafe { cloned.native_mut().fRect.offset(dx, dy) }
        cloned
    }

    pub fn contains(&self, rect: &Rect) -> bool {
        unsafe { self.native().contains(&rect.into_native()) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }

    #[warn(unused)]
    pub fn transform(&self, matrix: &Matrix) -> Option<Self> {
        let mut r = Self::default();
        unsafe { self.native().transform(matrix.native(), r.native_mut()) }
            .if_true_some(r)
    }
}
