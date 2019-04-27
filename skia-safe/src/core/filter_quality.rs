use crate::prelude::*;
use skia_bindings::SkFilterQuality;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FilterQuality {
    None = SkFilterQuality::kNone_SkFilterQuality as _,
    Low = SkFilterQuality::kLow_SkFilterQuality as _,
    Medium = SkFilterQuality::kMedium_SkFilterQuality as _,
    High = SkFilterQuality::kHigh_SkFilterQuality as _,
}

impl NativeTransmutable<SkFilterQuality> for FilterQuality {}
#[test]
fn test_filter_quality_layout() {
    FilterQuality::test_layout()
}
