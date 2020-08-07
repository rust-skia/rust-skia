use crate::prelude::*;
use crate::{image_filter::CropRect, image_filters, scalar, Color, IRect, ImageFilter, Point3};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn distant_lit_diffuse_lighting<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        direction: impl Into<Point3>,
        light_color: impl Into<Color>,
        surface_scale: scalar,
        kd: scalar,
    ) -> Option<Self> {
        image_filters::distant_lit_diffuse(
            direction,
            light_color,
            surface_scale,
            kd,
            self,
            crop_rect,
        )
    }

    pub fn point_lit_diffuse_lighting<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        location: impl Into<Point3>,
        light_color: impl Into<Color>,
        surface_scale: scalar,
        kd: scalar,
    ) -> Option<Self> {
        image_filters::point_lit_diffuse(location, light_color, surface_scale, kd, self, crop_rect)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn spot_lit_diffuse_lighting<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        location: impl Into<Point3>,
        target: impl Into<Point3>,
        specular_exponent: scalar,
        cutoff_angle: scalar,
        light_color: impl Into<Color>,
        surface_scale: scalar,
        kd: scalar,
    ) -> Option<Self> {
        image_filters::spot_lit_diffuse(
            location,
            target,
            specular_exponent,
            cutoff_angle,
            light_color,
            surface_scale,
            kd,
            self,
            crop_rect,
        )
    }

    pub fn distant_lit_specular_lighting<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        direction: impl Into<Point3>,
        light_color: impl Into<Color>,
        surface_scale: scalar,
        ks: scalar,
        shininess: scalar,
    ) -> Option<Self> {
        image_filters::distant_lit_specular(
            direction,
            light_color,
            surface_scale,
            ks,
            shininess,
            self,
            crop_rect,
        )
    }

    pub fn point_lit_specular_lighting<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        location: impl Into<Point3>,
        light_color: impl Into<Color>,
        surface_scale: scalar,
        ks: scalar,
        shininess: scalar,
    ) -> Option<Self> {
        image_filters::point_lit_specular(
            location,
            light_color,
            surface_scale,
            ks,
            shininess,
            self,
            crop_rect,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn spot_lit_specular_lighting<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        location: impl Into<Point3>,
        target: impl Into<Point3>,
        specular_exponent: scalar,
        cutoff_angle: scalar,
        light_color: impl Into<Color>,
        surface_scale: scalar,
        ks: scalar,
        shininess: scalar,
    ) -> Option<Self> {
        image_filters::spot_lit_specular(
            location,
            target,
            specular_exponent,
            cutoff_angle,
            light_color,
            surface_scale,
            ks,
            shininess,
            self,
            crop_rect,
        )
    }
}

#[deprecated(since = "0.19.0", note = "use image_filters::distant_lit_diffuse")]
pub fn distant_lit_diffuse<'a>(
    direction: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    kd: scalar,
    input: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkLightingImageFilter_MakeDistantLitDiffuse(
            direction.into().native(),
            light_color.into().into_native(),
            surface_scale,
            kd,
            input.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

#[deprecated(since = "0.19.0", note = "use image_filters::point_lit_diffuse")]
pub fn point_lit_diffuse<'a>(
    location: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    kd: scalar,
    input: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkLightingImageFilter_MakePointLitDiffuse(
            location.into().native(),
            light_color.into().into_native(),
            surface_scale,
            kd,
            input.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

#[deprecated(since = "0.19.0", note = "use image_filters::spot_lit_diffuse")]
#[allow(clippy::too_many_arguments)]
pub fn spot_lit_diffuse<'a>(
    location: impl Into<Point3>,
    target: impl Into<Point3>,
    specular_exponent: scalar,
    cutoff_angle: scalar,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    kd: scalar,
    input: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkLightingImageFilter_MakeSpotLitDiffuse(
            location.into().native(),
            target.into().native(),
            specular_exponent,
            cutoff_angle,
            light_color.into().into_native(),
            surface_scale,
            kd,
            input.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

#[deprecated(since = "0.19.0", note = "use image_filters::distant_lit_specular")]
pub fn distant_lit_specular<'a>(
    direction: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    ks: scalar,
    shininess: scalar,
    input: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkLightingImageFilter_MakeDistantLitSpecular(
            direction.into().native(),
            light_color.into().into_native(),
            surface_scale,
            ks,
            shininess,
            input.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

#[deprecated(since = "0.19.0", note = "use image_filters::point_lit_specular")]
pub fn point_lit_specular<'a>(
    location: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    ks: scalar,
    shininess: scalar,
    input: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkLightingImageFilter_MakePointLitSpecular(
            location.into().native(),
            light_color.into().into_native(),
            surface_scale,
            ks,
            shininess,
            input.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

#[deprecated(since = "0.19.0", note = "use image_filters::spot_lit_specular")]
#[allow(clippy::too_many_arguments)]
pub fn spot_lit_specular<'a>(
    location: impl Into<Point3>,
    target: impl Into<Point3>,
    specular_exponent: scalar,
    cutoff_angle: scalar,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    ks: scalar,
    shininess: scalar,
    input: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkLightingImageFilter_MakeSpotLitSpecular(
            location.into().native(),
            target.into().native(),
            specular_exponent,
            cutoff_angle,
            light_color.into().into_native(),
            surface_scale,
            ks,
            shininess,
            input.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
