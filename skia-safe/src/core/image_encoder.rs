use crate::{Bitmap, Data, EncodedImageFormat, Pixmap};

impl Pixmap {
    pub fn encode(&self, format: EncodedImageFormat, quality: usize) -> Option<Data> {
        crate::encode::pixmap(self, format, quality)
    }
}

impl Bitmap {
    pub fn encode(&self, format: EncodedImageFormat, quality: usize) -> Option<Data> {
        crate::encode::bitmap(self, format, quality)
    }
}

pub mod encode {
    // TODO: wrap stream variants.

    use crate::prelude::*;
    use crate::{Bitmap, Data, EncodedImageFormat, Pixmap};
    use skia_bindings as sb;

    pub fn pixmap(src: &Pixmap, format: EncodedImageFormat, quality: usize) -> Option<Data> {
        Data::from_ptr(unsafe {
            sb::C_SkEncodePixmap(src.native(), format, quality.try_into().unwrap())
        })
    }

    pub fn bitmap(src: &Bitmap, format: EncodedImageFormat, quality: usize) -> Option<Data> {
        Data::from_ptr(unsafe {
            sb::C_SkEncodeBitmap(src.native(), format, quality.try_into().unwrap())
        })
    }
}
