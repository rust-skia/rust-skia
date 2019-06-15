use crate::prelude::*;
use crate::{gpu, Canvas, IRect, ImageInfo, Matrix, Point, Rect, NativeFlattenable};
use skia_bindings::{C_SkDrawable_GpuDrawHandler_destruct, C_SkDrawable_GpuDrawHandler_draw, C_SkDrawable_snapGpuDrawHandler, SkDrawable, SkDrawable_GpuDrawHandler, SkRefCntBase, SkFlattenable, C_SkDrawable_Deserialize};

pub type Drawable = RCHandle<SkDrawable>;

impl NativeRefCountedBase for SkDrawable {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl NativeFlattenable for SkDrawable {
    fn native_flattenable(&self) -> &SkFlattenable {
        &self._base
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { C_SkDrawable_Deserialize(data.as_ptr() as _, data.len()) }
    }
}

impl RCHandle<SkDrawable> {
    pub fn draw(&mut self, canvas: &mut Canvas, matrix: Option<&Matrix>) {
        unsafe {
            self.native_mut()
                .draw(canvas.native_mut(), matrix.native_ptr_or_null())
        }
    }

    pub fn draw_at(&mut self, canvas: &mut Canvas, point: impl Into<Point>) {
        let point = point.into();
        unsafe {
            self.native_mut()
                .draw1(canvas.native_mut(), point.x, point.y)
        }
    }

    pub fn snap_gpu_draw_handler(
        &mut self,
        api: gpu::BackendAPI,
        matrix: &Matrix,
        clip_bounds: impl Into<IRect>,
        buffer_info: &ImageInfo,
    ) -> Option<GPUDrawHandler> {
        unsafe {
            C_SkDrawable_snapGpuDrawHandler(
                self.native_mut(),
                api.into_native(),
                matrix.native(),
                clip_bounds.into().native(),
                buffer_info.native(),
            )
        }
        .to_option()
        .map(GPUDrawHandler)
    }

    // TODO: clarify ref-counter situation here, return value is SkPicture*
    /*
    pub fn new_picture_snapshot(&mut self) -> Option<Picture> {
        unimplemented!()
    }
    */

    pub fn generation_id(&mut self) -> u32 {
        unsafe { self.native_mut().getGenerationID() }
    }

    pub fn bounds(&mut self) -> Rect {
        Rect::from_native(unsafe { self.native_mut().getBounds() })
    }

    pub fn notify_drawing_changed(&mut self) {
        unsafe { self.native_mut().notifyDrawingChanged() }
    }
}

pub struct GPUDrawHandler(*mut SkDrawable_GpuDrawHandler);

impl NativeAccess<SkDrawable_GpuDrawHandler> for GPUDrawHandler {
    fn native(&self) -> &SkDrawable_GpuDrawHandler {
        unsafe { &*self.0 }
    }

    fn native_mut(&mut self) -> &mut SkDrawable_GpuDrawHandler {
        unsafe { &mut *self.0 }
    }
}

impl Drop for GPUDrawHandler {
    fn drop(&mut self) {
        unsafe { C_SkDrawable_GpuDrawHandler_destruct(self.native_mut()) }
    }
}

impl GPUDrawHandler {
    pub fn draw(&mut self, info: &gpu::BackendDrawableInfo) {
        unsafe {
            C_SkDrawable_GpuDrawHandler_draw(self.native_mut(), info.native());
        }
    }
}
