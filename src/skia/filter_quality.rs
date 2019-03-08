use crate::prelude::*;
use rust_skia::SkFilterQuality;

pub type FilterQuality = EnumHandle<SkFilterQuality>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkFilterQuality> {
    pub const None: Self = Self(SkFilterQuality::kNone_SkFilterQuality);
    pub const Low: Self = Self(SkFilterQuality::kLow_SkFilterQuality);
    pub const Medium: Self = Self(SkFilterQuality::kMedium_SkFilterQuality);
    pub const High: Self = Self(SkFilterQuality::kHigh_SkFilterQuality);
}