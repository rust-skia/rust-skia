use crate::{
    prelude::*,
    skia::{
        Data,
        Canvas,
        Rect
    }
};
use rust_skia::{
    C_SkPicture_playback,
    SkPicture,
    C_SkPicture_MakeFromData,
    C_SkPicture_cullRect,
    SkRect,
    C_SkPicture_MakePlaceholder,
    C_SkPicture_serialize
};

#[derive(RCCloneDrop)]
pub struct Picture(pub(crate) *mut SkPicture);

impl RefCounted for Picture {
    fn _ref(&self) {
        unsafe { (*self.0)._base._base.ref_(); }
    }

    fn _unref(&self) {
        unsafe { (*self.0)._base._base.unref(); }
    }
}

impl Picture {
    pub fn from_data(data: &Data) -> Picture {
        Picture(unsafe { C_SkPicture_MakeFromData(data.0) })
    }

    pub fn new_placeholder(cull: &Rect) -> Picture {
        Picture(unsafe { C_SkPicture_MakePlaceholder(&cull.to_native()) })
    }

    pub fn playback(&self, canvas: &Canvas) {
        unsafe { C_SkPicture_playback(self.0, canvas.native) }
    }

    pub fn cull_rect(&self) -> Rect {
        Rect::from_native(unsafe { C_SkPicture_cullRect(self.0) })
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { (*self.0).uniqueID() }
    }

    pub fn serialize(&self) -> Data {
        Data(unsafe { C_SkPicture_serialize(self.0) })
    }
}