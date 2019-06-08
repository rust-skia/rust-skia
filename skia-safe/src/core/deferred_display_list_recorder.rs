use crate::prelude::*;
use crate::{Canvas, SurfaceCharacterization};
use skia_bindings::SkDeferredDisplayList;

// TODO: complete implementation
pub enum DeferredDisplayListRecorder {}
// TODO: declared in core/private/, so be opaque I guess.
pub type DeferredDisplayList = Handle<SkDeferredDisplayList>;

impl NativeDrop for SkDeferredDisplayList {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl DeferredDisplayListRecorder {
    pub fn new(characterization: &SurfaceCharacterization) -> Self {
        unimplemented!()
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        unimplemented!()
    }

    pub fn detach(&mut self) -> DeferredDisplayList {
        unimplemented!()
    }

    // TODO: makePromiseTexture()?
    // TODO: makeYUVAPromiseTexture()?
}
