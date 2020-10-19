use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::skia_textlayout_ParagraphCache;

pub type ParagraphCache = Handle<skia_textlayout_ParagraphCache>;

impl NativeDrop for skia_textlayout_ParagraphCache {
    fn drop(&mut self) {
        unsafe { sb::C_ParagraphCache_destruct(self) }
    }
}

impl Handle<skia_textlayout_ParagraphCache> {
    pub fn new() -> ParagraphCache {
        ParagraphCache::from_native_c(unsafe { skia_textlayout_ParagraphCache::new() })
    }

    pub fn abandon(&mut self) {
        unsafe { self.native_mut().abandon() }
    }

    pub fn reset(&mut self) {
        unsafe { self.native_mut().reset() }
    }

    pub fn print_statistics(&mut self) {
        unsafe { self.native_mut().printStatistics() }
    }

    pub fn turn_on(&mut self, value: bool) {
        self.native_mut().fCacheIsOn = value
    }

    pub fn count(&mut self) -> i32 {
        unsafe { sb::C_ParagraphCache_count(self.native_mut()) }
    }
}
