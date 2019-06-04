use crate::prelude::*;
use crate::{scalar, Point, Size, Vector};
use skia_bindings::SkRSXform;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct RSXForm {
    pub scos: scalar,
    pub ssin: scalar,
    pub tx: scalar,
    pub ty: scalar,
}

impl NativeTransmutable<SkRSXform> for RSXForm {}
#[test]
fn test_rsx_form_layout() {
    RSXForm::test_layout()
}

impl RSXForm {
    pub fn new(scos: scalar, ssin: scalar, t: impl Into<Vector>) -> Self {
        let t = t.into();
        Self {
            scos,
            ssin,
            tx: t.x,
            ty: t.y,
        }
    }

    pub fn from_radians(
        scale: scalar,
        radians: scalar,
        t: impl Into<Vector>,
        a: impl Into<Point>,
    ) -> Self {
        let t = t.into();
        let a = a.into();
        RSXForm::from_native(unsafe {
            SkRSXform::MakeFromRadians(scale, radians, t.x, t.y, a.x, a.y)
        })
    }

    pub fn rect_stays_rect(&self) -> bool {
        // unsafe { self.native().rectStaysRect() }
        self.scos == 0.0 || self.ssin == 0.0
    }

    pub fn set_identity(&mut self) {
        // does not link:
        // unsafe { self.native_mut().setIdentity() }
        self.scos = 1.0;
        self.ssin = 0.0;
        self.tx = 0.0;
        self.ty = 0.0;
    }

    pub fn set(&mut self, scos: scalar, ssin: scalar, t: impl Into<Vector>) {
        let t = t.into();
        unsafe { self.native_mut().set(scos, ssin, t.x, t.y) }
    }

    pub fn to_quad(&self, size: impl Into<Size>) -> [Point; 4] {
        let size = size.into();
        let mut quad: [Point; 4] = Default::default();
        unsafe {
            self.native()
                .toQuad(size.width, size.height, quad.native_mut().as_mut_ptr())
        }
        quad
    }

    pub fn to_tri_strip(&self, size: impl Into<Size>) -> [Point; 4] {
        let size = size.into();
        let mut strip: [Point; 4] = Default::default();
        unsafe {
            self.native()
                .toTriStrip(size.width, size.height, strip.native_mut().as_mut_ptr())
        }
        strip
    }
}
