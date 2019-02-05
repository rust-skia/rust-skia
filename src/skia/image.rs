use rust_skia::{SkImage, C_SkImage_encodeToData};
use super::data::Data;

pub struct Image {
    pub(crate) native: *mut SkImage
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { (*self.native)._base._base.unref() }
    }
}

impl Image {

    pub fn encode_to_data(&self) -> Data {
        Data { native: unsafe { C_SkImage_encodeToData(self.native) } }
    }
}
