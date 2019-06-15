use crate::prelude::*;
use crate::{Canvas, Matrix, Paint, Picture};
use skia_bindings::{C_SkMultiPictureDraw_destruct, SkMultiPictureDraw};

pub type MultiPictureDraw = Handle<SkMultiPictureDraw>;

impl NativeDrop for SkMultiPictureDraw {
    fn drop(&mut self) {
        // does not link under Windows:
        // unsafe { SkMultiPictureDraw::destruct(self) }
        unsafe { C_SkMultiPictureDraw_destruct(self) }
    }
}

impl Handle<SkMultiPictureDraw> {
    pub fn new(reserve: impl Into<Option<usize>>) -> Self {
        Handle::from_native(unsafe {
            SkMultiPictureDraw::new(reserve.into().unwrap_or_default().try_into().unwrap())
        })
    }

    pub fn add<'b>(
        mut self,
        canvas: &'b mut Canvas,
        picture: &'b Picture,
        matrix: Option<&'b Matrix>,
        paint: Option<&'b Paint>,
    ) -> Borrows<'b, MultiPictureDraw> {
        unsafe {
            self.native_mut().add(
                canvas.native_mut(),
                picture.native(),
                matrix.native_ptr_or_null(),
                paint.native_ptr_or_null(),
            )
        }
        self.borrows(canvas)
    }
}

impl<'a> Borrows<'a, MultiPictureDraw> {
    pub fn add<'b>(
        mut self,
        canvas: &'b mut Canvas,
        picture: &'b Picture,
        matrix: Option<&'b Matrix>,
        paint: Option<&'b Paint>,
    ) -> Borrows<'b, MultiPictureDraw>
    where
        'a: 'b,
    {
        unsafe {
            self.native_mut().add(
                canvas.native_mut(),
                picture.native(),
                matrix.native_ptr_or_null(),
                paint.native_ptr_or_null(),
            );
            self
        }
    }

    pub fn draw(mut self, flush: impl Into<Option<bool>>) -> MultiPictureDraw {
        unsafe {
            self.native_mut().draw(flush.into().unwrap_or(false));
            self.release()
        }
    }

    pub fn reset(mut self) -> MultiPictureDraw {
        unsafe {
            self.native_mut().reset();
            self.release()
        }
    }
}
