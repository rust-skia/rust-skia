use crate::prelude::*;
use crate::{BlendMode, DrawLooper, Vector};
use skia_bindings::{
    C_SkLayerDrawLooper_Builder_destruct, C_SkLayerDrawLooper_Builder_detach,
    SkLayerDrawLooper_Builder, SkLayerDrawLooper_LayerInfo, SkPaint,
};

bitflags! {
    pub struct Bits : u32 {
        const STYLE = skia_bindings::SkLayerDrawLooper_Bits_kStyle_Bit as _;
        const PATH_EFFECT = skia_bindings::SkLayerDrawLooper_Bits_kPathEffect_Bit as _;
        const MASK_FILTER = skia_bindings::SkLayerDrawLooper_Bits_kMaskFilter_Bit as _;
        const SHADER = skia_bindings::SkLayerDrawLooper_Bits_kShader_Bit as _;
        const COLOR_FILTER = skia_bindings::SkLayerDrawLooper_Bits_kColorFilter_Bit as _;
        const XFER_MODE = skia_bindings::SkLayerDrawLooper_Bits_kXfermode_Bit as _;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BitFlags {
    Bits(Bits),
    EntirePaint,
}

impl From<Bits> for BitFlags {
    fn from(bits: Bits) -> Self {
        BitFlags::Bits(bits)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LayerInfo {
    pub paint_bits: BitFlags,
    pub color_mode: BlendMode,
    pub offset: Vector,
    pub post_translate: bool,
}

pub type Builder = Handle<SkLayerDrawLooper_Builder>;

impl NativeDrop for SkLayerDrawLooper_Builder {
    fn drop(&mut self) {
        unsafe { C_SkLayerDrawLooper_Builder_destruct(self) }
    }
}

impl Default for Builder {
    fn default() -> Builder {
        Self::from_native(unsafe { SkLayerDrawLooper_Builder::new() })
    }
}

impl LayerInfo {
    fn to_native(&self) -> SkLayerDrawLooper_LayerInfo {
        let paint_bits: i32 = match self.paint_bits {
            BitFlags::Bits(bits) => bits.bits().try_into().unwrap(),
            BitFlags::EntirePaint => -1,
        };

        SkLayerDrawLooper_LayerInfo {
            fPaintBits: paint_bits,
            fColorMode: self.color_mode.into_native(),
            fOffset: self.offset.into_native(),
            fPostTranslate: self.post_translate,
        }
    }
}

impl Builder {
    pub fn add_layer(&mut self, layer_info: &LayerInfo) -> &mut Handle<SkPaint> {
        unsafe { transmute_ref_mut(&mut *self.native_mut().addLayer(&layer_info.to_native())) }
    }

    pub fn add_layer_offset(&mut self, delta: impl Into<Option<Vector>>) {
        let delta = delta.into().unwrap_or_default();
        unsafe {
            self.native_mut().addLayer1(delta.x, delta.y);
        }
    }

    pub fn add_layer_on_top(&mut self, layer_info: &LayerInfo) -> &mut Handle<SkPaint> {
        unsafe { transmute_ref_mut(&mut *self.native_mut().addLayerOnTop(&layer_info.to_native())) }
    }

    pub fn detach(&mut self) -> DrawLooper {
        DrawLooper::from_ptr(unsafe { C_SkLayerDrawLooper_Builder_detach(self.native_mut()) })
            .unwrap()
    }
}
