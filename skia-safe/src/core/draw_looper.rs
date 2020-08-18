use crate::prelude::*;
use crate::{scalar, BlurStyle, Color, NativeFlattenable, Paint, Rect, Vector};
use skia_bindings as sb;
use skia_bindings::{SkDrawLooper, SkDrawLooper_BlurShadowRec, SkFlattenable, SkRefCntBase};

#[deprecated(since = "0.33.0", note = "No longer supported.")]
pub type DrawLooper = RCHandle<SkDrawLooper>;

impl NativeBase<SkRefCntBase> for SkDrawLooper {}

impl NativeRefCountedBase for SkDrawLooper {
    type Base = SkRefCntBase;
}

impl NativeBase<SkFlattenable> for SkDrawLooper {}

impl NativeFlattenable for SkDrawLooper {
    fn native_flattenable(&self) -> &SkFlattenable {
        &self.base()
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { sb::C_SkDrawLooper_Deserialize(data.as_ptr() as _, data.len()) }
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
        unsafe { sb::C_SkDrawLooper_asABlurShadow(self.native(), br.native_mut()) }.if_true_some(br)
    }

    // TODO: apply
}
