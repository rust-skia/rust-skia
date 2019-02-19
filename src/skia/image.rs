use rust_skia::{SkImage, C_SkImage_encodeToData};
use super::data::Data;

pub struct Image(pub (crate) *mut SkImage);

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { (*self.0)._base._base.unref() }
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        unsafe { (*self.0)._base._base.ref_() }
        Image(self.0)
    }
}

impl Image {

    pub fn encode_to_data(&self) -> Data {
        Data(unsafe { C_SkImage_encodeToData(self.0) })
    }
}
