use crate::prelude::*;
use crate::{scalar, Color, Color4f, ColorSpace, DrawLooper, Vector};
use skia_bindings::{C_SkBlurDrawLooper_Make, C_SkBlurDrawLooper_Make2, SkDrawLooper};

impl RCHandle<SkDrawLooper> {
    pub fn blur(color: impl Into<Color>, sigma: scalar, delta: impl Into<Vector>) -> Option<Self> {
        new(color, sigma, delta)
    }

    pub fn blur_with_color_space(
        color: impl AsRef<Color4f>,
        color_space: &ColorSpace,
        sigma: scalar,
        delta: impl Into<Vector>,
    ) -> Option<Self> {
        new_with_color_space(color, color_space, sigma, delta)
    }
}

// TODO: we already support a similar passing of either a Color or a pair of Color4f/ColorSpace in GradientShaderColors, can we use that here?

pub fn new(color: impl Into<Color>, sigma: scalar, delta: impl Into<Vector>) -> Option<DrawLooper> {
    let delta = delta.into();
    DrawLooper::from_ptr(unsafe {
        C_SkBlurDrawLooper_Make(color.into().into_native(), sigma, delta.x, delta.y)
    })
}

// note: default of ColorSpace is set to SRGB if null here,
// but we don't want to support that and be explicit about the colorspace when a Color4f is provided.
// TODO: is Color4f + &ColorSpace worth using pair?
pub fn new_with_color_space(
    color: impl AsRef<Color4f>,
    color_space: &ColorSpace,
    sigma: scalar,
    delta: impl Into<Vector>,
) -> Option<DrawLooper> {
    let delta = delta.into();
    DrawLooper::from_ptr(unsafe {
        // TODO: the rule that the passing side should increase the ref counter falls apart here.
        //       Can we ensure that the ref count is increased when it actually is needed?
        C_SkBlurDrawLooper_Make2(
            *color.as_ref().native(),
            color_space.native(),
            sigma,
            delta.x,
            delta.y,
        )
    })
}
