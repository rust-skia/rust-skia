use crate::prelude::*;
use crate::{scalar, Color, Color4f, ColorSpace, DrawLooper, Vector};
use skia_bindings::{C_SkBlurDrawLooper_Make, C_SkBlurDrawLooper_Make2, SkDrawLooper};

pub enum BlurDrawLooper {}

impl BlurDrawLooper {
    // TODO: we already support a similar passing of either a Color or a pair of Color4f/ColorSpace in GradientShaderColors, can we use that here?

    #[allow(clippy::new_ret_no_self)]
    pub fn new<IC: Into<Color>, IV: Into<Vector>>(
        color: IC,
        sigma: scalar,
        delta: IV,
    ) -> Option<DrawLooper> {
        let delta = delta.into();
        DrawLooper::from_ptr(unsafe {
            C_SkBlurDrawLooper_Make(color.into().into_native(), sigma, delta.x, delta.y)
        })
    }

    // note: default of ColorSpace is set to SRGB if null here, but we don't support that.
    // TODO: is Color4f + &ColorSpace worth using pair?
    pub fn new_with_color_space<C: AsRef<Color4f>, IV: Into<Vector>>(
        color: C,
        color_space: &ColorSpace,
        sigma: scalar,
        delta: IV,
    ) -> Option<DrawLooper> {
        let color = color.as_ref();
        let delta = delta.into();

        DrawLooper::from_ptr(unsafe {
            // TODO: the rule that the passing side should increase the ref counter falls apart here.
            //       Can we ensure that the ref count is increased when it actually is needed?
            C_SkBlurDrawLooper_Make2(
                color.as_ref().into_native(),
                color_space.native_mut_force(),
                sigma,
                delta.x,
                delta.y,
            )
        })
    }
}

impl RCHandle<SkDrawLooper> {
    pub fn blur<IC: Into<Color>, IV: Into<Vector>>(
        color: IC,
        sigma: scalar,
        delta: IV,
    ) -> Option<Self> {
        BlurDrawLooper::new(color, sigma, delta)
    }

    pub fn blur_with_color_space<C: AsRef<Color4f>, IV: Into<Vector>>(
        color: C,
        color_space: &ColorSpace,
        sigma: scalar,
        delta: IV,
    ) -> Option<Self> {
        BlurDrawLooper::new_with_color_space(color, color_space, sigma, delta)
    }
}
