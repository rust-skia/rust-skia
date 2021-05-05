pub use skia_bindings::SkTileMode as TileMode;

#[test]
fn test_tile_mode_naming() {
    let _ = TileMode::Mirror;
}
