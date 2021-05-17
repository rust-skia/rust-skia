use crate::{prelude::*, scalar, MaskFilter};
use skia_bindings::{self as sb, SkMaskFilter};

pub fn new_gamma_table(gamma: scalar) -> [u8; 256] {
    construct(|t| unsafe { sb::SkTableMaskFilter_MakeGammaTable(t as *mut u8, gamma) })
}

pub fn new_clip_table(min: u8, max: u8) -> [u8; 256] {
    construct(|t| unsafe { sb::SkTableMaskFilter_MakeClipTable(t as *mut u8, min, max) })
}

impl MaskFilter {
    pub fn table(table: &[u8; 256]) -> MaskFilter {
        new(table)
    }

    pub fn gamma(gamma: scalar) -> MaskFilter {
        new_gamma(gamma)
    }

    pub fn clip(min: u8, max: u8) -> MaskFilter {
        new_clip(min, max)
    }
}

pub fn new(table: &[u8; 256]) -> MaskFilter {
    MaskFilter::from_ptr(unsafe { sb::SkTableMaskFilter_Create(table.as_ptr()) }).unwrap()
}

pub fn new_gamma(gamma: scalar) -> MaskFilter {
    MaskFilter::from_ptr(unsafe { sb::SkTableMaskFilter_CreateGamma(gamma) }).unwrap()
}

pub fn new_clip(min: u8, max: u8) -> MaskFilter {
    MaskFilter::from_ptr(unsafe { sb::SkTableMaskFilter_CreateClip(min, max) }).unwrap()
}
