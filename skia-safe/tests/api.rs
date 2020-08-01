use skia_safe::{color_filters, BlendMode, Color, ColorFilter, Mutable, Sendable};

/// Test if RCHandle<> types can be wrapped into a Sendable and unwrapped.
#[test]
fn sendable() {
    let color = Color::CYAN;
    let mode = BlendMode::ColorBurn;
    let cf = color_filters::blend(color, mode).unwrap();
    let sendable = cf.wrap_send().ok().unwrap();
    let _unwrapped = sendable.unwrap();
}

/// Test if Sendable<> is actually sendable for RCHandle types.
#[test]
fn sendable_implements_send() {
    assert::send::<Sendable<ColorFilter>>();
}

pub mod assert {
    pub fn send<T: Send>() {}
    pub fn sync<T: Sync>() {}
}
