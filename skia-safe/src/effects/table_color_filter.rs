use crate::prelude::*;
use crate::skia::ColorFilter;
use skia_bindings::{C_SkTableColorFilter_Make, C_SkTableColorFilter_MakeARGB};

pub enum TableColorFilter {}

impl TableColorFilter {

    pub fn from_table(table: &[u8; 256]) -> ColorFilter {
        ColorFilter::from_ptr(unsafe {
            C_SkTableColorFilter_Make(table.as_ptr())
        }).unwrap()
    }

    pub fn from_argb(table_a: Option<&[u8; 256]>, table_r: Option<&[u8; 256]>, table_g: Option<&[u8; 256]>, table_b: Option<&[u8; 256]> ) -> ColorFilter {
        ColorFilter::from_ptr(unsafe {
            C_SkTableColorFilter_MakeARGB(
                table_a.as_ptr_or_null() as _,
                table_r.as_ptr_or_null() as _,
                table_g.as_ptr_or_null() as _,
                table_b.as_ptr_or_null() as _)
        }).unwrap()
    }
}