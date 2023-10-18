use crate::{interop::RustWStream, prelude::*, Data, Pixmap};
use skia_bindings::{SkJpegEncoder_AlphaOption, SkJpegEncoder_Downsample};
use std::io;

pub type AlphaOption = SkJpegEncoder_AlphaOption;
variant_name!(AlphaOption::BlendOnBlack);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Downsample {
    BothDirections,
    Horizontal,
    No,
}

impl Downsample {
    fn native(&self) -> SkJpegEncoder_Downsample {
        match self {
            Downsample::BothDirections => SkJpegEncoder_Downsample::k420,
            Downsample::Horizontal => SkJpegEncoder_Downsample::k422,
            Downsample::No => SkJpegEncoder_Downsample::k444,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub quality: u32,
    pub downsample: Downsample,
    pub alpha_option: AlphaOption,
    pub xmp_metadata: Option<String>,
    // TODO: ICCProfile
    // TODO: ICCProfileDescription
}

impl Default for Options {
    fn default() -> Self {
        Self {
            quality: 100,
            downsample: Downsample::BothDirections,
            alpha_option: AlphaOption::Ignore,
            xmp_metadata: None,
        }
    }
}

pub fn encode<W: io::Write>(pixmap: &Pixmap, writer: &mut W, options: &Options) -> bool {
    let xml_metadata = options.xmp_metadata.as_ref().map(Data::new_str);
    let mut stream = RustWStream::new(writer);

    unsafe {
        skia_bindings::C_SkJpegEncoder_Encode(
            stream.stream_mut(),
            pixmap.native(),
            options.quality as _,
            options.downsample.native(),
            options.alpha_option,
            xml_metadata.as_ref().native_ptr_or_null(),
        )
    }
}

// TODO: encode YUVAPixmaps

pub fn encode_image<'a>(
    context: impl Into<Option<&'a mut crate::gpu::DirectContext>>,
    img: &crate::Image,
    options: &Options,
) -> Option<crate::Data> {
    let xml_metadata = options.xmp_metadata.as_ref().map(Data::new_str);

    Data::from_ptr(unsafe {
        skia_bindings::C_SkJpegEncoder_EncodeImage(
            context.into().native_ptr_or_null_mut(),
            img.native(),
            options.quality as _,
            options.downsample.native(),
            options.alpha_option,
            xml_metadata.as_ref().native_ptr_or_null(),
        )
    })
}

// TODO: Make (Pixmap + SkYUVAPixmaps)
