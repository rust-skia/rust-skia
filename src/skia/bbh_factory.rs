use crate::prelude::*;
use rust_skia::SkBBHFactory;

pub type BBHFactory = Handle<SkBBHFactory>;

impl NativeDrop for SkBBHFactory {
    fn drop(&mut self) {
        // TODO: make implementation complete
        unimplemented!()
    }
}

// TODO: make implementation complete
impl Handle<SkBBHFactory> {
}