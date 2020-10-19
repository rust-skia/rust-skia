use crate::prelude::*;
use crate::{Canvas, Data, Rect};
use crate::{Matrix, Shader, TileMode};
use skia_bindings as sb;
use skia_bindings::{SkPicture, SkRefCntBase};

pub type Picture = RCHandle<SkPicture>;
unsafe impl Sync for Picture {}
unsafe impl Send for Picture {}

impl NativeRefCountedBase for SkPicture {
    type Base = SkRefCntBase;
}

impl RCHandle<SkPicture> {
    // TODO: wrap MakeFromStream

    // TODO: may support SkSerialProces in MakeFromData?

    pub fn from_data(data: &Data) -> Option<Picture> {
        Picture::from_ptr(unsafe { sb::C_SkPicture_MakeFromData(data.native()) })
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Picture> {
        Picture::from_ptr(unsafe {
            sb::C_SkPicture_MakeFromData2(bytes.as_ptr() as _, bytes.len())
        })
    }

    // TODO: AbortCallback and the function that use it.

    pub fn playback(&self, canvas: &mut Canvas) {
        unsafe { sb::C_SkPicture_playback(self.native(), canvas.native_mut()) }
    }

    pub fn cull_rect(&self) -> Rect {
        Rect::from_native_c(unsafe { sb::C_SkPicture_cullRect(self.native()) })
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { sb::C_SkPicture_uniqueID(self.native()) }
    }

    // TODO: support SkSerialProcs in serialize()?

    pub fn serialize(&self) -> Data {
        Data::from_ptr(unsafe { sb::C_SkPicture_serialize(self.native()) }).unwrap()
    }

    pub fn new_placeholder(cull: impl AsRef<Rect>) -> Picture {
        Picture::from_ptr(unsafe { sb::C_SkPicture_MakePlaceholder(cull.as_ref().native()) })
            .unwrap()
    }

    pub fn approximate_op_count(&self) -> usize {
        self.approximate_op_count_nested(false)
    }

    pub fn approximate_op_count_nested(&self, nested: impl Into<Option<bool>>) -> usize {
        let nested = nested.into().unwrap_or(false);
        unsafe {
            sb::C_SkPicture_approximateOpCount(self.native(), nested)
                .try_into()
                .unwrap()
        }
    }

    pub fn approximate_bytes_used(&self) -> usize {
        unsafe {
            let mut value = 0;
            sb::C_SkPicture_approximateBytesUsed(self.native(), &mut value);
            value
        }
    }

    pub fn to_shader<'a, 'b>(
        &self,
        tm: impl Into<Option<(TileMode, TileMode)>>,
        local_matrix: impl Into<Option<&'a Matrix>>,
        tile_rect: impl Into<Option<&'b Rect>>,
    ) -> Shader {
        let tm = tm.into();
        let local_matrix = local_matrix.into();
        let tile_rect = tile_rect.into();
        let tmx = tm.map(|tm| tm.0).unwrap_or_default();
        let tmy = tm.map(|tm| tm.1).unwrap_or_default();

        Shader::from_ptr(unsafe {
            sb::C_SkPicture_makeShader(
                self.native(),
                tmx,
                tmy,
                local_matrix.native_ptr_or_null(),
                tile_rect.native_ptr_or_null(),
            )
        })
        .unwrap()
    }
}
