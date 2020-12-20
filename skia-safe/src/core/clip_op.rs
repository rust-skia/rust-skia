pub use skia_bindings::SkClipOp as ClipOp;
#[test]
fn test_clip_op_naming() {
    let _ = ClipOp::Difference;
}
