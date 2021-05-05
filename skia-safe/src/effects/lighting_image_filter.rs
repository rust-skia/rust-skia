use crate::prelude::*;
use crate::{image_filters, scalar, Color, IRect, Point3};
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
            crop_rect.into().map(|r| r.into()),
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
        image_filters::point_lit_diffuse(
            location,
            light_color,
            surface_scale,
            kd,
            self,
            crop_rect.into().map(|r| r.into()),
        )
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
            crop_rect.into().map(|r| r.into()),
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
            crop_rect.into().map(|r| r.into()),
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
            crop_rect.into().map(|r| r.into()),
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
            crop_rect.into().map(|r| r.into()),
        )
    }
}
