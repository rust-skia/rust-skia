use crate::skia::ColorFilter;
use skia_bindings::{C_SkTableColorFilter_Make, C_SkTableColorFilter_MakeARGB};

pub enum TableColorFilter {}

impl TableColorFilter {

    pub fn from_table(table: &[u8; 256]) -> ColorFilter {
        ColorFilter::from_ptr(unsafe {
            C_SkTableColorFilter_Make(table.as_ptr())
        }).unwrap()
    }

    pub fn from_argb(table_a: &[u8; 256], table_r: &[u8; 256], table_g: &[u8; 256], table_b: &[u8; 256] ) -> ColorFilter {
        ColorFilter::from_ptr(unsafe {
            C_SkTableColorFilter_MakeARGB(table_a.as_ptr(), table_r.as_ptr(), table_g.as_ptr(), table_b.as_ptr())
        }).unwrap()
    }
}