use crate::prelude::*;
use crate::skia::{
    Data,
    Canvas,
    Rect
};
use rust_skia::{
    C_SkPicture_approximateOpCount,
    C_SkPicture_playback,
    SkPicture,
    C_SkPicture_MakeFromData,
    C_SkPicture_cullRect,
    C_SkPicture_MakePlaceholder,
    C_SkPicture_serialize,
    C_SkPicture_approximateBytesUsed,
    SkRefCntBase
};

pub type Picture = RCHandle<SkPicture>;

impl NativeRefCountedBase for SkPicture {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl RCHandle<SkPicture> {
    pub fn from_data(data: &Data) -> Picture {
        Picture::from_ptr(unsafe {
            C_SkPicture_MakeFromData(data.native())
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

    pub fn new_placeholder(cull: &Rect) -> Picture {
        Picture::from_ptr(unsafe {
            C_SkPicture_MakePlaceholder(&cull.into_native())
        }).unwrap()
    }

    pub fn approximate_op_count(&self) -> usize {
        unsafe {
            C_SkPicture_approximateOpCount(self.native()).try_into().unwrap()
        }
    }

    pub fn approximate_bytes_used(&self) -> usize {
        unsafe {
            let mut value = 0;
            C_SkPicture_approximateBytesUsed(self.native(), &mut value);
            value
        }
    }
}