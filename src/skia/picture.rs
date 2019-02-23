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

pub type Picture = RCHandle<SkPicture>;

impl RefCounted for SkPicture {
    fn _ref(&self) {
        unsafe { self._base._base.ref_(); }
    }

    fn _unref(&self) {
        unsafe { self._base._base.unref(); }
    }
}

impl Picture {
    pub fn from_data(data: &Data) -> Picture {
        Picture::from_ptr(unsafe {
            C_SkPicture_MakeFromData(data.native())
        }).unwrap()
    }

    pub fn new_placeholder(cull: &Rect) -> Picture {
        Picture::from_ptr(unsafe {
            C_SkPicture_MakePlaceholder(&cull.to_native())
        }).unwrap()
    }

    pub fn playback(&self, canvas: &mut Canvas) {
        unsafe { C_SkPicture_playback(self.native(), canvas.native_mut()) }
    }

    pub fn cull_rect(&self) -> Rect {
        Rect::from_native(unsafe {
            C_SkPicture_cullRect(self.native())
        })
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { self.native().uniqueID() }
    }

    pub fn serialize(&self) -> Data {
        Data::from_ptr(unsafe {
            C_SkPicture_serialize(self.native())
        }).unwrap()
    }
}