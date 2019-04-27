use crate::prelude::*;
use crate::ColorFilter;
use skia_bindings::{C_SkTableColorFilter_Make, C_SkTableColorFilter_MakeARGB, SkColorFilter};

pub enum TableColorFilter {}

impl TableColorFilter {
    pub fn from_table(table: &[u8; 256]) -> ColorFilter {
        ColorFilter::from_ptr(unsafe { C_SkTableColorFilter_Make(table.as_ptr()) }).unwrap()
    }

    // TODO: consider to use Into<Option<&[u8; 256]>>
    pub fn from_argb(
        table_a: Option<&[u8; 256]>,
        table_r: Option<&[u8; 256]>,
        table_g: Option<&[u8; 256]>,
        table_b: Option<&[u8; 256]>,
    ) -> ColorFilter {
        ColorFilter::from_ptr(unsafe {
            C_SkTableColorFilter_MakeARGB(
                table_a.map(|t| t.as_ref()).as_ptr_or_null(),
                table_r.map(|t| t.as_ref()).as_ptr_or_null(),
                table_g.map(|t| t.as_ref()).as_ptr_or_null(),
                table_b.map(|t| t.as_ref()).as_ptr_or_null(),
            )
        })
        .unwrap()
    }
}

impl RCHandle<SkColorFilter> {
    pub fn from_table(table: &[u8; 256]) -> Self {
        TableColorFilter::from_table(table)
    }

    pub fn from_argb(
        table_a: Option<&[u8; 256]>,
        table_r: Option<&[u8; 256]>,
        table_g: Option<&[u8; 256]>,
        table_b: Option<&[u8; 256]>,
    ) -> Self {
        TableColorFilter::from_argb(table_a, table_r, table_g, table_b)
    }
}
