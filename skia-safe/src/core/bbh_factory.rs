use crate::prelude::*;
use skia_bindings::SkBBHFactory;

// TODO: complete the implementation
pub type BBHFactory = Handle<SkBBHFactory>;
unsafe impl Sync for BBHFactory {}
unsafe impl Send for BBHFactory {}

impl NativeDrop for SkBBHFactory {
    fn drop(&mut self) {
        unimplemented!()
    }
}

// TODO: complete the implementation
impl Handle<SkBBHFactory> {}
