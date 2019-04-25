use crate::prelude::*;
use crate::core::{
    Data,
    Canvas,
    Rect
};
use skia_bindings::{C_SkPicture_approximateOpCount, C_SkPicture_playback, SkPicture, C_SkPicture_MakeFromData, C_SkPicture_cullRect, C_SkPicture_MakePlaceholder, C_SkPicture_serialize, C_SkPicture_approximateBytesUsed, SkRefCntBase, C_SkPicture_makeShader};
use crate::{TileMode, Matrix, Shader};

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

    pub fn playback(&self, mut canvas: impl AsMut<Canvas>) {
        unsafe { C_SkPicture_playback(self.native(), canvas.as_mut().native_mut()) }
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

    pub fn new_placeholder<C: AsRef<Rect>>(cull: C) -> Picture {
        Picture::from_ptr(unsafe {
            C_SkPicture_MakePlaceholder(cull.as_ref().native())
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

    pub fn as_shader<'a, 'b, TM: Into<Option<(TileMode, TileMode)>>, LM: Into<Option<&'a Matrix>>, TR: Into<Option<&'b Rect>>>(
        &self, tm: TM, local_matrix: LM, tile_rect: TR) -> Shader {
        let tm = tm.into();
        let local_matrix = local_matrix.into();
        let tile_rect = tile_rect.into();
        let tmx = tm.map(|tm| tm.0).unwrap_or_default();
        let tmy = tm.map(|tm| tm.1).unwrap_or_default();

        Shader::from_ptr(unsafe {
            C_SkPicture_makeShader(self.native(), tmx.into_native(), tmy.into_native(), local_matrix.native_ptr_or_null(), tile_rect.native_ptr_or_null())
        }).unwrap()
    }
}