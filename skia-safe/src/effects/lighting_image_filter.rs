use crate::prelude::*;
use crate::{scalar, Color, ImageFilter, ImageFilterCropRect, Point3};
use skia_bindings::{
    C_SkLightingImageFilter_MakeDistantLitDiffuse, C_SkLightingImageFilter_MakeDistantLitSpecular,
    C_SkLightingImageFilter_MakePointLitDiffuse, C_SkLightingImageFilter_MakePointLitSpecular,
    C_SkLightingImageFilter_MakeSpotLitDiffuse, C_SkLightingImageFilter_MakeSpotLitSpecular,
};

pub enum LightingImageFilter {}

impl LightingImageFilter {
    pub fn distant_lit_diffuse<
        'a,
        IP3: Into<Point3>,
        IC: Into<Color>,
        CR: Into<Option<&'a ImageFilterCropRect>>,
    >(
        direction: IP3,
        light_color: IC,
        surface_scale: scalar,
        kd: scalar,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkLightingImageFilter_MakeDistantLitDiffuse(
                direction.into().native(),
                light_color.into().into_native(),
                surface_scale,
                kd,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }

    pub fn point_lit_diffuse<
        'a,
        IP3: Into<Point3>,
        IC: Into<Color>,
        CR: Into<Option<&'a ImageFilterCropRect>>,
    >(
        location: IP3,
        light_color: IC,
        surface_scale: scalar,
        kd: scalar,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkLightingImageFilter_MakePointLitDiffuse(
                location.into().native(),
                light_color.into().into_native(),
                surface_scale,
                kd,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn spot_lit_diffuse<
        'a,
        IP3L: Into<Point3>,
        IP3T: Into<Point3>,
        IC: Into<Color>,
        CR: Into<Option<&'a ImageFilterCropRect>>,
    >(
        location: IP3L,
        target: IP3T,
        specular_exponent: scalar,
        cutoff_angle: scalar,
        light_color: IC,
        surface_scale: scalar,
        kd: scalar,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkLightingImageFilter_MakeSpotLitDiffuse(
                location.into().native(),
                target.into().native(),
                specular_exponent,
                cutoff_angle,
                light_color.into().into_native(),
                surface_scale,
                kd,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }

    pub fn distant_lit_specular<
        'a,
        IP3: Into<Point3>,
        IC: Into<Color>,
        CR: Into<Option<&'a ImageFilterCropRect>>,
    >(
        direction: IP3,
        light_color: IC,
        surface_scale: scalar,
        ks: scalar,
        shininess: scalar,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkLightingImageFilter_MakeDistantLitSpecular(
                direction.into().native(),
                light_color.into().into_native(),
                surface_scale,
                ks,
                shininess,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }

    pub fn point_lit_specular<
        'a,
        IP3: Into<Point3>,
        IC: Into<Color>,
        CR: Into<Option<&'a ImageFilterCropRect>>,
    >(
        location: IP3,
        light_color: IC,
        surface_scale: scalar,
        ks: scalar,
        shininess: scalar,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkLightingImageFilter_MakePointLitSpecular(
                location.into().native(),
                light_color.into().into_native(),
                surface_scale,
                ks,
                shininess,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn spot_lit_specular<
        'a,
        IP3L: Into<Point3>,
        IP3T: Into<Point3>,
        IC: Into<Color>,
        CR: Into<Option<&'a ImageFilterCropRect>>,
    >(
        location: IP3L,
        target: IP3T,
        specular_exponent: scalar,
        cutoff_angle: scalar,
        light_color: IC,
        surface_scale: scalar,
        ks: scalar,
        shininess: scalar,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkLightingImageFilter_MakeSpotLitSpecular(
                location.into().native(),
                target.into().native(),
                specular_exponent,
                cutoff_angle,
                light_color.into().into_native(),
                surface_scale,
                ks,
                shininess,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}
