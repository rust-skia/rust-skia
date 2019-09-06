use crate::prelude::*;
use crate::{scalar, BlurStyle, Color, NativeFlattenable, Paint, Rect, Vector};
use skia_bindings::{
    C_SkDrawLooper_Deserialize, C_SkDrawLooper_asABlurShadow, SkDrawLooper,
    SkDrawLooper_BlurShadowRec, SkFlattenable, SkRefCntBase,
};

pub type DrawLooper = RCHandle<SkDrawLooper>;

impl NativeRefCountedBase for SkDrawLooper {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl NativeFlattenable for SkDrawLooper {
    fn native_flattenable(&self) -> &SkFlattenable {
        &self._base
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { C_SkDrawLooper_Deserialize(data.as_ptr() as _, data.len()) }
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
    // TODO: Context
    // TODO: makeContext

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

    // TODO: apply
}
