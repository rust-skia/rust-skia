use crate::prelude::*;
use skia_bindings::SkTileMode;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum TileMode {
    Clamp = SkTileMode::kClamp as _,
    Repeat = SkTileMode::kRepeat as _,
    Mirror = SkTileMode::kMirror as _,
    Decal = SkTileMode::kDecal as _,
}

impl NativeTransmutable<SkTileMode> for TileMode {}
#[test]
fn test_tile_mode_layout() {
    TileMode::test_layout()
}

impl Default for TileMode {
    fn default() -> Self {
        TileMode::Clamp
    }
}
