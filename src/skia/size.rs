use crate::prelude::*;
use crate::skia::scalar;
use rust_skia::{
    SkSize,
    SkISize,
    C_SkSize_toFloor
};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct ISize {
    pub width: i32,
    pub height: i32
}

impl NativeTransmutable<SkISize> for ISize {}

#[test]
fn isize_layout() {
    ISize::test_layout()
}

impl ISize {
    pub fn new(w: i32, h: i32) -> ISize {
        ISize { width: w, height: h }
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    pub fn is_empty(&self) -> bool {
        self.width <= 0 || self.height <= 0
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Size {
    pub width: scalar,
    pub height: scalar
}

impl NativeTransmutable<SkSize> for Size {}

#[test]
fn test_size_layout() {
   Size::test_layout()
}

impl Size {
    pub fn new(w: scalar, h: scalar) -> Size {
        Size { width: w, height: h }
    }

    pub fn from_isize(src: ISize) -> Size {
        Self::new(src.width as _, src.height as _)
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0.0 && self.height == 0.0
    }

    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    pub fn to_round(&self) -> ISize {
        ISize::from_native(unsafe { self.native().toRound() })
    }

    pub fn to_ceil(&self) -> ISize {
        ISize::from_native(unsafe { self.native().toCeil() })
    }

    pub fn to_floor(&self) -> ISize {
        // does not link:
        // ISize::from_native(unsafe { self.native().toFloor() })
        ISize::from_native(unsafe {
            C_SkSize_toFloor(self.native())
        })
    }
}

//
// From
//

impl From<(i32, i32)> for ISize {
    fn from(source: (i32, i32)) -> Self {
        Self::new(source.0, source.1)
    }
}

impl From<(scalar, scalar)> for Size {
    fn from(source: (scalar, scalar)) -> Self {
        Self::new(source.0, source.1)
    }
}

impl From<ISize> for Size {
    fn from(size: ISize) -> Self {
        Self::new(size.width as _, size.height as _)
    }
}