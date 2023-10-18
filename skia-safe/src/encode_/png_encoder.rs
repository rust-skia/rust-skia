use std::{ffi::CString, io};

use crate::{interop::RustWStream, prelude::*, DataTable, Pixmap};
use skia_bindings as sb;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FilterFlag: u32 {
        const ZERO = sb::SkPngEncoder_FilterFlag::kZero as _;
        const NONE = sb::SkPngEncoder_FilterFlag::kNone as _;
        const SUB = sb::SkPngEncoder_FilterFlag::kSub as _;
        const UP = sb::SkPngEncoder_FilterFlag::kUp as _;
        const AVG = sb::SkPngEncoder_FilterFlag::kAvg as _;
        const PAETH = sb::SkPngEncoder_FilterFlag::kPaeth as _;
        const ALL = Self::NONE.bits() | Self::SUB.bits() | Self::UP.bits() | Self::AVG.bits() | Self::PAETH.bits();
    }
}
native_transmutable!(sb::SkPngEncoder_FilterFlag, FilterFlag, filter_flag_layout);

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Options {
    pub filter_flags: FilterFlag,
    pub z_lib_level: i32,
    pub comments: Vec<Comment>,
    // TODO: ICCProfile
    // TODO: ICCProfileDescription
}

impl Default for Options {
    fn default() -> Self {
        Self {
            filter_flags: FilterFlag::ALL,
            z_lib_level: 6,
            comments: vec![],
        }
    }
}

impl Options {
    fn comments_to_data_table(&self) -> Option<DataTable> {
        let mut comments = Vec::with_capacity(self.comments.len() * 2);
        for c in self.comments.iter() {
            comments.push(CString::new(c.keyword.as_str()).ok()?);
            comments.push(CString::new(c.text.as_str()).ok()?);
        }
        let slices: Vec<&[u8]> = comments.iter().map(|c| c.as_bytes_with_nul()).collect();
        Some(DataTable::from_slices(&slices))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub keyword: String,
    pub text: String,
}

impl Comment {
    pub fn new(keyword: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            text: text.into(),
        }
    }
}

pub fn encode<W: io::Write>(pixmap: &Pixmap, writer: &mut W, options: &Options) -> bool {
    let Some(comments) = options.comments_to_data_table() else {
        return false;
    };

    let mut stream = RustWStream::new(writer);

    unsafe {
        sb::C_SkPngEncoder_Encode(
            stream.stream_mut(),
            pixmap.native(),
            comments.into_ptr(),
            options.filter_flags.into_native(),
            options.z_lib_level,
        )
    }
}

pub fn encode_image<'a>(
    context: impl Into<Option<&'a mut crate::gpu::DirectContext>>,
    img: &crate::Image,
    options: &Options,
) -> Option<crate::Data> {
    crate::Data::from_ptr(unsafe {
        sb::C_SkPngEncoder_EncodeImage(
            context.into().native_ptr_or_null_mut(),
            img.native(),
            options.comments_to_data_table()?.into_ptr(),
            options.filter_flags.into_native(),
            options.z_lib_level,
        )
    })
}

// TODO: Make
