use crate::{prelude::*, ColorFilter};
use skia_bindings as sb;

impl ColorFilter {
    pub fn from_table(table: &[u8; 256]) -> Self {
        from_table(table)
    }

    pub fn from_argb(
        table_a: Option<&[u8; 256]>,
        table_r: Option<&[u8; 256]>,
        table_g: Option<&[u8; 256]>,
        table_b: Option<&[u8; 256]>,
    ) -> Self {
        from_argb(table_a, table_r, table_g, table_b)
    }
}

pub fn from_table(table: &[u8; 256]) -> ColorFilter {
    ColorFilter::from_ptr(unsafe { sb::C_SkTableColorFilter_Make(table.as_ptr()) }).unwrap()
}

#[allow(clippy::redundant_closure)]
pub fn from_argb(
    table_a: Option<&[u8; 256]>,
    table_r: Option<&[u8; 256]>,
    table_g: Option<&[u8; 256]>,
    table_b: Option<&[u8; 256]>,
) -> ColorFilter {
    ColorFilter::from_ptr(unsafe {
        sb::C_SkTableColorFilter_MakeARGB(
            table_a.map(|t| t.as_ref()).as_ptr_or_null(),
            table_r.map(|t| t.as_ref()).as_ptr_or_null(),
            table_g.map(|t| t.as_ref()).as_ptr_or_null(),
            table_b.map(|t| t.as_ref()).as_ptr_or_null(),
        )
    })
    .unwrap()
}
