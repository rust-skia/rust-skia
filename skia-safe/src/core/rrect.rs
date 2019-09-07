use crate::prelude::*;
use crate::{scalar, Matrix, Rect, Vector};
use skia_bindings as sb;
use skia_bindings::{SkRRect, SkRRect_Corner, SkRRect_Type};
use std::{mem, ptr};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Type {
    Empty = SkRRect_Type::kEmpty_Type as _,
    Rect = SkRRect_Type::kRect_Type as _,
    Oval = SkRRect_Type::kOval_Type as _,
    Simple = SkRRect_Type::kSimple_Type as _,
    NinePatch = SkRRect_Type::kNinePatch_Type as _,
    Complex = SkRRect_Type::kComplex_Type as _,
}

impl NativeTransmutable<SkRRect_Type> for Type {}
#[test]
fn test_rrect_type_layout() {
    Type::test_layout()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Corner {
    UpperLeft = SkRRect_Corner::kUpperLeft_Corner as _,
    UpperRight = SkRRect_Corner::kUpperRight_Corner as _,
    LowerRight = SkRRect_Corner::kLowerRight_Corner as _,
    LowerLeft = SkRRect_Corner::kLowerLeft_Corner as _,
}

impl NativeTransmutable<SkRRect_Corner> for Corner {}
#[test]
fn test_rrect_corner_layout() {
    Corner::test_layout()
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RRect(SkRRect);

impl NativeTransmutable<SkRRect> for RRect {}
#[test]
fn test_rrect_layout() {
    RRect::test_layout()
}

impl PartialEq for RRect {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkRRect_Equals(self.native(), rhs.native()) }
    }
}

impl Default for RRect {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<RRect> for RRect {
    fn as_ref(&self) -> &RRect {
        self
    }
}

impl RRect {
    pub fn new() -> Self {
        RRect::construct(|rr| unsafe { sb::C_SkRRect_Construct(rr) })
    }

    pub fn get_type(&self) -> Type {
        Type::from_native(unsafe { sb::C_SkRRect_getType(self.native()) })
    }

    pub fn is_empty(&self) -> bool {
        self.get_type() == Type::Empty
    }

    pub fn is_rect(&self) -> bool {
        self.get_type() == Type::Rect
    }

    pub fn is_oval(&self) -> bool {
        self.get_type() == Type::Oval
    }

    pub fn is_simple(&self) -> bool {
        self.get_type() == Type::Simple
    }

    pub fn is_nine_patch(&self) -> bool {
        self.get_type() == Type::NinePatch
    }

    pub fn is_complex(&self) -> bool {
        self.get_type() == Type::Complex
    }

    pub fn width(&self) -> scalar {
        self.rect().width()
    }

    pub fn height(&self) -> scalar {
        self.rect().height()
    }

    pub fn simple_radii(&self) -> Vector {
        self.radii(Corner::UpperLeft)
    }

    pub fn set_empty(&mut self) {
        *self = Self::new()
    }

    pub fn set_rect(&mut self, rect: impl AsRef<Rect>) {
        unsafe { sb::C_SkRRect_setRect(self.native_mut(), rect.as_ref().native()) }
    }

    pub fn new_empty() -> Self {
        Self::new()
    }

    // TODO: consider to rename all the following new_* function to from_* functions?
    //       is it possible to find a proper convention here (new_ vs from_?)?

    pub fn new_rect(rect: impl AsRef<Rect>) -> Self {
        let mut rr = Self::default();
        rr.set_rect(rect);
        rr
    }

    pub fn new_oval(oval: impl AsRef<Rect>) -> Self {
        let mut rr = Self::default();
        rr.set_oval(oval);
        rr
    }

    pub fn new_rect_xy(rect: impl AsRef<Rect>, x_rad: scalar, y_rad: scalar) -> Self {
        let mut rr = Self::default();
        rr.set_rect_xy(rect.as_ref(), x_rad, y_rad);
        rr
    }

    pub fn new_nine_patch(
        rect: impl AsRef<Rect>,
        left_rad: scalar,
        top_rad: scalar,
        right_rad: scalar,
        bottom_rad: scalar,
    ) -> Self {
        let mut r = Self::default();
        unsafe {
            r.native_mut().setNinePatch(
                rect.as_ref().native(),
                left_rad,
                top_rad,
                right_rad,
                bottom_rad,
            )
        }
        r
    }

    pub fn new_rect_radii(rect: impl AsRef<Rect>, radii: &[Vector; 4]) -> Self {
        let mut r = Self::default();
        unsafe {
            r.native_mut()
                .setRectRadii(rect.as_ref().native(), radii.native().as_ptr())
        }
        r
    }

    pub fn set_oval(&mut self, oval: impl AsRef<Rect>) {
        unsafe { sb::C_SkRRect_setOval(self.native_mut(), oval.as_ref().native()) }
    }

    pub fn set_rect_xy(&mut self, rect: impl AsRef<Rect>, x_rad: scalar, y_rad: scalar) {
        unsafe {
            self.native_mut()
                .setRectXY(rect.as_ref().native(), x_rad, y_rad)
        }
    }

    pub fn set_nine_patch(
        &mut self,
        rect: impl AsRef<Rect>,
        left_rad: scalar,
        top_rad: scalar,
        right_rad: scalar,
        bottom_rad: scalar,
    ) {
        unsafe {
            self.native_mut().setNinePatch(
                rect.as_ref().native(),
                left_rad,
                top_rad,
                right_rad,
                bottom_rad,
            )
        }
    }

    pub fn set_rect_radii(&mut self, rect: impl AsRef<Rect>, radii: &[Vector; 4]) {
        unsafe {
            self.native_mut()
                .setRectRadii(rect.as_ref().native(), radii.native().as_ptr())
        }
    }

    pub fn rect(&self) -> &Rect {
        Rect::from_native_ref(&self.native().fRect)
    }

    pub fn radii(&self, corner: Corner) -> Vector {
        Vector::from_native(self.native().fRadii[corner as usize])
    }

    pub fn bounds(&self) -> &Rect {
        self.rect()
    }

    pub fn inset(&mut self, delta: impl Into<Vector>) {
        *self = self.with_inset(delta)
    }

    pub fn with_inset(&self, delta: impl Into<Vector>) -> Self {
        let delta = delta.into();
        let mut r = Self::default();
        unsafe { self.native().inset(delta.x, delta.y, r.native_mut()) };
        r
    }

    pub fn outset(&mut self, delta: impl Into<Vector>) {
        *self = self.with_outset(delta)
    }

    pub fn with_outset(&self, delta: impl Into<Vector>) -> Self {
        self.with_inset(-delta.into())
    }

    pub fn offset(&mut self, delta: impl Into<Vector>) {
        Rect::from_native_ref_mut(&mut self.native_mut().fRect).offset(delta)
    }

    pub fn with_offset(&self, delta: impl Into<Vector>) -> Self {
        let mut copied = *self;
        copied.offset(delta);
        copied
    }

    pub fn contains(&self, rect: impl AsRef<Rect>) -> bool {
        unsafe { self.native().contains(rect.as_ref().native()) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }

    pub const SIZE_IN_MEMORY: usize = mem::size_of::<Self>();

    pub fn write_to_memory(&self, buffer: &mut Vec<u8>) {
        unsafe {
            let size = self.native().writeToMemory(ptr::null_mut());
            buffer.resize(size, 0);
            let written = self.native().writeToMemory(buffer.as_mut_ptr() as _);
            debug_assert_eq!(written, size);
        }
    }

    pub fn read_from_memory(&mut self, buffer: &[u8]) -> usize {
        unsafe {
            self.native_mut()
                .readFromMemory(buffer.as_ptr() as _, buffer.len())
        }
    }

    #[must_use]
    pub fn transform(&self, matrix: &Matrix) -> Option<Self> {
        let mut r = Self::default();
        unsafe { self.native().transform(matrix.native(), r.native_mut()) }.if_true_some(r)
    }

    pub fn dump(&self, as_hex: impl Into<Option<bool>>) {
        unsafe { self.native().dump(as_hex.into().unwrap_or_default()) }
    }

    pub fn dump_hex(&self) {
        self.dump(true)
    }
}
