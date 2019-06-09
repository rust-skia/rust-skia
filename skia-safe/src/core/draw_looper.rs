use crate::prelude::*;
use crate::{scalar, BlurStyle, Color, Paint, Rect, Vector};
use skia_bindings::{
    C_SkDrawLooper_asABlurShadow, SkDrawLooper, SkDrawLooper_BlurShadowRec, SkRefCntBase,
};

pub type DrawLooper = RCHandle<SkDrawLooper>;

impl NativeRefCountedBase for SkDrawLooper {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[repr(C)]
pub struct BlurShadowRec {
    pub sigma: scalar,
    pub offset: Vector,
    pub color: Color,
    pub blur: BlurStyle,
}

impl NativeTransmutable<SkDrawLooper_BlurShadowRec> for BlurShadowRec {}
#[test]
fn test_blur_shadow_rec_layout() {
    BlurShadowRec::test_layout()
}

impl RCHandle<SkDrawLooper> {

    pub fn can_compute_fast_bounds(&self, paint: &Paint) -> bool {
        unsafe { self.native().canComputeFastBounds(paint.native()) }
    }

    pub fn compute_fast_bounds(&self, paint: &Paint, src: impl AsRef<Rect>) -> Rect {
        let mut r = Rect::default();
        unsafe {
            self.native()
                .computeFastBounds(paint.native(), src.as_ref().native(), r.native_mut())
        };
        r
    }

    pub fn as_a_blur_shadow(&self) -> Option<BlurShadowRec> {
        let mut br = BlurShadowRec::default();
        unsafe { C_SkDrawLooper_asABlurShadow(self.native(), br.native_mut()) }.if_true_some(br)
    }

    // TODO: Deserialize
}
