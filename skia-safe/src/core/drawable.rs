use crate::prelude::*;
use crate::{Canvas, Matrix, Point, gpu, IRect, ImageInfo, Picture, Rect};
use skia_bindings::{SkDrawable, SkRefCntBase};

pub type Drawable = RCHandle<SkDrawable>;

impl NativeRefCountedBase for SkDrawable {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

// TODO: complete implementation
impl RCHandle<SkDrawable> {

    pub fn draw(&mut self, canvas: &Canvas, matrix: Option<&Matrix>) -> &mut Self {
        unimplemented!()
    }

    pub fn draw_at(&mut self, canvas: &Canvas, point: impl Into<Point>) -> &mut Self {
        unimplemented!()
    }

    pub fn snap_gpu_draw_handler(&mut self,
                                 api: gpu::BackendAPI,
                                 matrix: &Matrix,
                                 clip_bounds: impl Into<IRect>,
                                 buffer_info: &ImageInfo) -> GPUDrawHandler {
        unimplemented!()
    }

    // TODO: clarify ref-counter situation here, return value is SkPicture*
    pub fn new_picture_snapshot(&mut self) -> Option<Picture> {
        unimplemented!()
    }

    pub fn generation_id(&mut self) -> u32 {
        unimplemented!()
    }

    pub fn bounds(&mut self) -> Rect {
        unimplemented!()
    }

    pub fn notify_drawing_changed(&mut self) -> &mut Self {
        unimplemented!()
    }

    // TODO: Deserialize()
}

pub enum GPUDrawHandler {}

// TODO: complete implementation
impl GPUDrawHandler {
    /* TODO:
    pub fn draw(info: &crate::gpu::BackendDrawableInfo) {
        unimplemented!()
    }
    */
}
