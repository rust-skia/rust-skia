//! Encoding of images into the PNG format, with configurable filtering and compression.

use std::io;

use crate::{Pixmap, encode, interop::RustWStream, prelude::*};
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
native_transmutable!(sb::SkPngEncoder_FilterFlag, FilterFlag);

/// PNG encoding options.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Options {
    /// Selects which filtering strategies to use.
    ///
    /// If a single filter is chosen, libpng will use that filter for every row.
    ///
    /// If multiple filters are chosen, libpng will use a heuristic to guess which filter will
    /// encode smallest, then apply that filter. This happens on a per row basis, different rows can
    /// use different filters.
    ///
    /// Using a single filter (or less filters) is typically faster. Trying all of the filters may
    /// help minimize the output file size.
    ///
    /// Our default value matches libpng's default.
    pub filter_flags: FilterFlag,
    /// Must be in `0..=9` where 9 corresponds to maximal compression. This value is passed directly
    /// to zlib. 0 is a special case to skip zlib entirely, creating dramatically larger pngs.
    ///
    /// Our default value matches libpng's default.
    pub z_lib_level: i32,
    /// Represents comments in the `tEXt` ancillary chunk of the png.
    ///
    /// The 2i-th entry is the keyword for the i-th comment, and the (2i + 1)-th entry is the text
    /// for the i-th comment.
    pub comments: Vec<encode::Comment>,
    // TODO: fHdrMetadata (skhdr::Metadata is private)
    // TODO: If SkGainmapInfo get out of private/ : fGainmap fGainmapInfo
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

#[deprecated(since = "0.90.0", note = "use encode::Comment")]
pub type Comment = encode::Comment;

/// Encode the `src` pixels to the `dst` stream.
/// `options` may be used to control the encoding behavior.
///
/// Returns `true` on success. Returns `false` on an invalid or unsupported `src`.
pub fn encode<W: io::Write>(pixmap: &Pixmap, writer: &mut W, options: &Options) -> bool {
    let Some(comments) = encode::comments::to_data_table(&options.comments) else {
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

/// Returns the encoded data for the pixmap, or `None` on failure.
pub fn encode_pixmap(src: &Pixmap, options: &Options) -> Option<crate::Data> {
    crate::Data::from_ptr(unsafe {
        sb::C_SkPngEncoder_EncodePixmap(
            src.native(),
            encode::comments::to_data_table(&options.comments)?.into_ptr(),
            options.filter_flags.into_native(),
            options.z_lib_level,
        )
    })
}

/// Encode the provided image and return the resulting bytes. If the image was created as
/// a texture-backed image on a GPU context, that `context` must be provided so the pixels
/// can be read before being encoded. For raster-backed images, `context` can be `None`.
/// `options` may be used to control the encoding behavior.
///
/// Returns `None` if the pixels could not be read or encoding otherwise fails.
pub fn encode_image<'a>(
    context: impl Into<Option<&'a mut crate::gpu::DirectContext>>,
    img: &crate::Image,
    options: &Options,
) -> Option<crate::Data> {
    crate::Data::from_ptr(unsafe {
        sb::C_SkPngEncoder_EncodeImage(
            context.into().native_ptr_or_null_mut(),
            img.native(),
            encode::comments::to_data_table(&options.comments)?.into_ptr(),
            options.filter_flags.into_native(),
            options.z_lib_level,
        )
    })
}

// TODO: Make
