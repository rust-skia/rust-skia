use std::io;

use crate::{Pixmap, encode, interop::RustWStream, prelude::*};
use skia_bindings as sb;

/// The compression level used when encoding PNGs.
///
/// Variants:
/// - [`CompressionLevel::Low`]: Low compression level - fast, but may result in bigger PNG files.
/// - [`CompressionLevel::Medium`]: Medium compression level - somewhere in-between `Low` and
///   `High`.
/// - [`CompressionLevel::High`]: High compression level - slow, but should results in smaller PNG
///   files.
pub type CompressionLevel = sb::SkPngRustEncoder_CompressionLevel;
variant_name!(CompressionLevel::Medium);

/// PNG encoding options.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Options {
    pub compression_level: CompressionLevel,
    /// Represents comments to be written into `tEXt` chunks of the png.
    ///
    /// The 2i-th entry is the keyword for the i-th comment, and the (2i + 1)-th entry is the text
    /// for the i-th comment.
    ///
    /// All entries are treated as strings encoded as Latin-1 (i.e. ISO-8859-1). The strings may,
    /// but don't have to be NUL-terminated (trailing NUL characters will be stripped). Encoding
    /// will fail if keyword or text don't meet the requirements of the PNG spec - text may have any
    /// length and contain any of the 191 Latin-1 characters (and/or the linefeed character), but
    /// keyword's length is restricted to at most 79 characters and it can't contain a non-breaking
    /// space character.
    pub comments: Vec<encode::Comment>,
    // TODO: fHdrMetadata (skhdr::Metadata is private)
}

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
        sb::C_SkPngRustEncoder_Encode(
            stream.stream_mut(),
            pixmap.native(),
            options.compression_level,
            comments.into_ptr(),
        )
    }
}

// TODO: Wrap `Make`
