use crate::prelude::*;
use crate::{BlendMode, DrawLooper, Vector};
use skia_bindings::{
    C_SkLayerDrawLooper_Builder_detach, SkLayerDrawLooper_Builder, SkLayerDrawLooper_LayerInfo,
    SkPaint,
};

bitflags! {
    pub struct LayerDrawLooperBits : u32 {
        const STYLE = skia_bindings::SkLayerDrawLooper_Bits_kStyle_Bit as _;
        const PATH_EFFECT = skia_bindings::SkLayerDrawLooper_Bits_kPathEffect_Bit as _;
        const MASK_FILTER = skia_bindings::SkLayerDrawLooper_Bits_kMaskFilter_Bit as _;
        const SHADER = skia_bindings::SkLayerDrawLooper_Bits_kShader_Bit as _;
        const COLOR_FILTER = skia_bindings::SkLayerDrawLooper_Bits_kColorFilter_Bit as _;
        const XFER_MODE = skia_bindings::SkLayerDrawLooper_Bits_kXfermode_Bit as _;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LayerDrawLooperBitFlags {
    Bits(LayerDrawLooperBits),
    EntirePaint,
}

impl From<LayerDrawLooperBits> for LayerDrawLooperBitFlags {
    fn from(bits: LayerDrawLooperBits) -> Self {
        LayerDrawLooperBitFlags::Bits(bits)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LayerDrawLooperLayerInfo {
    pub paint_bits: LayerDrawLooperBitFlags,
    pub color_mode: BlendMode,
    pub offset: Vector,
    pub post_translate: bool,
}

pub type LayerDrawLooperBuilder = Handle<SkLayerDrawLooper_Builder>;

impl NativeDrop for SkLayerDrawLooper_Builder {
    fn drop(&mut self) {
        unsafe { SkLayerDrawLooper_Builder::destruct(self) }
    }
}

impl Default for LayerDrawLooperBuilder {
    fn default() -> LayerDrawLooperBuilder {
        Self::from_native(unsafe { SkLayerDrawLooper_Builder::new() })
    }
}

impl From<&LayerDrawLooperLayerInfo> for SkLayerDrawLooper_LayerInfo {
    fn from(li: &LayerDrawLooperLayerInfo) -> Self {
        let paint_bits: i32 = match li.paint_bits {
            LayerDrawLooperBitFlags::Bits(bits) => bits.bits().try_into().unwrap(),
            LayerDrawLooperBitFlags::EntirePaint => -1,
        };

        Self {
            fPaintBits: paint_bits,
            fColorMode: li.color_mode.into_native(),
            fOffset: li.offset.into_native(),
            fPostTranslate: li.post_translate,
        }
    }
}

impl LayerDrawLooperBuilder {
    pub fn add_layer(&mut self, layer_info: &LayerDrawLooperLayerInfo) -> &mut Handle<SkPaint> {
        unsafe { transmute_ref_mut(&mut *self.native_mut().addLayer(&layer_info.into())) }
    }

    pub fn add_layer_offset<IV: Into<Option<Vector>>>(&mut self, delta: IV) {
        let delta = delta.into().unwrap_or_default();
        unsafe {
            self.native_mut().addLayer1(delta.x, delta.y);
        }
    }

    pub fn add_layer_on_top(
        &mut self,
        layer_info: &LayerDrawLooperLayerInfo,
    ) -> &mut Handle<SkPaint> {
        unsafe { transmute_ref_mut(&mut *self.native_mut().addLayerOnTop(&layer_info.into())) }
    }

    pub fn detach(&mut self) -> DrawLooper {
        DrawLooper::from_ptr(unsafe { C_SkLayerDrawLooper_Builder_detach(self.native_mut()) })
            .unwrap()
    }
}
