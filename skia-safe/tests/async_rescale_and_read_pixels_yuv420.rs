#![cfg(all(
    feature = "ganesh",
    feature = "vulkan",
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios")
))]

mod common;

use std::cell::RefCell;
use std::rc::Rc;

use common::vulkan::Vulkan;
use skia_safe::gpu::{self, DirectContext};
use skia_safe::image::{RescaleGamma, RescaleMode};
use skia_safe::{AlphaType, ColorType, IRect, ISize, ImageInfo, Surface, YUVColorSpace};

const SIZE: ISize = ISize::new(64, 32);

#[test]
fn calls_back_when_the_context_is_dropped_before_submitting() {
    with_gpu_surface(|context, mut surface| {
        let calls = Rc::new(RefCell::new(0));
        surface.async_rescale_and_read_pixels_yuv420(
            YUVColorSpace::JPEG,
            None,
            IRect::from_size(SIZE),
            SIZE,
            RescaleGamma::Src,
            RescaleMode::Nearest,
            {
                let calls = calls.clone();
                move |_| *calls.borrow_mut() += 1
            },
        );

        drop(surface);
        drop(context);

        assert_eq!(*calls.borrow(), 1);
        assert_eq!(Rc::strong_count(&calls), 1);
    });
}

fn with_gpu_surface(test: impl FnOnce(DirectContext, Surface)) {
    let Some(vulkan) = Vulkan::new() else {
        eprintln!("skipped: no Vulkan device available");
        return;
    };
    let Some(mut context) = vulkan.direct_context() else {
        eprintln!("skipped: no Vulkan context available");
        return;
    };
    let surface = render_target(&mut context);
    test(context, surface);
}

fn render_target(context: &mut DirectContext) -> Surface {
    let image_info = ImageInfo::new(SIZE, ColorType::RGBA8888, AlphaType::Premul, None);
    gpu::surfaces::render_target(
        context,
        gpu::Budgeted::Yes,
        &image_info,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap()
}
