use crate::{
    prelude::*, scalar, Blender, Color, ColorChannel, ColorFilter, CubicResampler, IPoint, IRect,
    ISize, Image, ImageFilter, Matrix, Picture, Point3, Rect, Region, SamplingOptions, Shader,
    TileMode, Vector,
};
use skia_bindings::{self as sb, SkImageFilter, SkImageFilters_CropRect};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct CropRect(Rect);

native_transmutable!(SkImageFilters_CropRect, CropRect, crop_rect_layout);

impl CropRect {
    pub const NO_CROP_RECT: CropRect = CropRect(Rect {
        left: scalar::NEG_INFINITY,
        top: scalar::NEG_INFINITY,
        right: scalar::INFINITY,
        bottom: scalar::INFINITY,
    });

    pub fn rect(&self) -> Option<Rect> {
        if *self == Self::NO_CROP_RECT {
            None
        } else {
            Some(self.0)
        }
    }
}

impl Default for CropRect {
    fn default() -> Self {
        CropRect::NO_CROP_RECT
    }
}

impl From<Option<CropRect>> for CropRect {
    fn from(opt: Option<CropRect>) -> Self {
        opt.unwrap_or(Self::NO_CROP_RECT)
    }
}

impl From<&CropRect> for CropRect {
    fn from(cr: &CropRect) -> Self {
        *cr
    }
}

impl From<IRect> for CropRect {
    fn from(r: IRect) -> Self {
        Self(Rect::from(r))
    }
}

impl From<&IRect> for CropRect {
    fn from(r: &IRect) -> Self {
        Self::from(*r)
    }
}

impl From<Rect> for CropRect {
    fn from(r: Rect) -> Self {
        Self(r)
    }
}

impl From<&Rect> for CropRect {
    fn from(r: &Rect) -> Self {
        Self(*r)
    }
}

pub fn alpha_threshold(
    region: &Region,
    inner_min: scalar,
    outer_max: scalar,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_AlphaThreshold(
            region.native(),
            inner_min,
            outer_max,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

#[allow(clippy::too_many_arguments)]
pub fn arithmetic(
    k1: scalar,
    k2: scalar,
    k3: scalar,
    k4: scalar,
    enforce_pm_color: bool,
    background: impl Into<Option<ImageFilter>>,
    foreground: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Arithmetic(
            k1,
            k2,
            k3,
            k4,
            enforce_pm_color,
            background.into().into_ptr_or_null(),
            foreground.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn blend(
    mode: impl Into<Blender>,
    background: impl Into<Option<ImageFilter>>,
    foreground: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Blend(
            mode.into().into_ptr(),
            background.into().into_ptr_or_null(),
            foreground.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn blur(
    (sigma_x, sigma_y): (scalar, scalar),
    tile_mode: impl Into<Option<TileMode>>,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Blur(
            sigma_x,
            sigma_y,
            tile_mode.into().unwrap_or(TileMode::Decal),
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn color_filter(
    cf: impl Into<ColorFilter>,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_ColorFilter(
            cf.into().into_ptr(),
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn compose(
    outer: impl Into<ImageFilter>,
    inner: impl Into<ImageFilter>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Compose(outer.into().into_ptr(), inner.into().into_ptr())
    })
}

pub fn displacement_map(
    (x_channel_selector, y_channel_selector): (ColorChannel, ColorChannel),
    scale: scalar,
    displacement: impl Into<Option<ImageFilter>>,
    color: impl Into<ImageFilter>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_DisplacementMap(
            x_channel_selector,
            y_channel_selector,
            scale,
            displacement.into().into_ptr_or_null(),
            color.into().into_ptr(),
            crop_rect.into().native(),
        )
    })
}

pub fn drop_shadow(
    delta: impl Into<Vector>,
    (sigma_x, sigma_y): (scalar, scalar),
    color: impl Into<Color>,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    let delta = delta.into();
    let color = color.into();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_DropShadow(
            delta.x,
            delta.y,
            sigma_x,
            sigma_y,
            color.into_native(),
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn drop_shadow_only(
    delta: impl Into<Vector>,
    (sigma_x, sigma_y): (scalar, scalar),
    color: impl Into<Color>,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    let delta = delta.into();
    let color = color.into();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_DropShadowOnly(
            delta.x,
            delta.y,
            sigma_x,
            sigma_y,
            color.into_native(),
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn image<'a>(
    image: impl Into<Image>,
    src_rect: impl Into<Option<&'a Rect>>,
    dst_rect: impl Into<Option<&'a Rect>>,
    sampling_options: impl Into<Option<SamplingOptions>>,
) -> Option<ImageFilter> {
    let image = image.into();
    let image_rect = Rect::from_iwh(image.width(), image.height());
    let src_rect = src_rect.into().unwrap_or(&image_rect);
    let dst_rect = dst_rect.into().unwrap_or(&image_rect);
    let sampling_options: SamplingOptions = sampling_options.into().unwrap_or_else(|| {
        CubicResampler {
            b: 1.0 / 3.0,
            c: 1.0 / 3.0,
        }
        .into()
    });

    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Image(
            image.into_ptr(),
            src_rect.as_ref().native(),
            dst_rect.as_ref().native(),
            sampling_options.native(),
        )
    })
}

pub fn magnifier(
    src_rect: impl AsRef<Rect>,
    inset: scalar,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Magnifier(
            src_rect.as_ref().native(),
            inset,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn magnifier2(
    lens_bounds: impl AsRef<Rect>,
    zoom_amount: scalar,
    inset: scalar,
    sampling_options: SamplingOptions,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Magnifier2(
            lens_bounds.as_ref().native(),
            zoom_amount,
            inset,
            sampling_options.native(),
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

#[allow(clippy::too_many_arguments)]
pub fn matrix_convolution(
    kernel_size: impl Into<ISize>,
    kernel: &[scalar],
    gain: scalar,
    bias: scalar,
    kernel_offset: impl Into<IPoint>,
    tile_mode: TileMode,
    convolve_alpha: bool,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    let kernel_size = kernel_size.into();
    assert_eq!(
        (kernel_size.width * kernel_size.height) as usize,
        kernel.len()
    );
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_MatrixConvolution(
            kernel_size.native(),
            kernel.as_ptr(),
            gain,
            bias,
            kernel_offset.into().native(),
            tile_mode,
            convolve_alpha,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn matrix_transform(
    matrix: &Matrix,
    sampling_options: impl Into<SamplingOptions>,
    input: impl Into<Option<ImageFilter>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_MatrixTransform(
            matrix.native(),
            sampling_options.into().native(),
            input.into().into_ptr_or_null(),
        )
    })
}

pub fn merge(
    filters: impl IntoIterator<Item = Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    let filter_ptrs: Vec<*mut SkImageFilter> =
        filters.into_iter().map(|f| f.into_ptr_or_null()).collect();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Merge(
            filter_ptrs.as_ptr(),
            filter_ptrs.len().try_into().unwrap(),
            crop_rect.into().native(),
        )
    })
}

pub fn offset(
    delta: impl Into<Vector>,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    let delta = delta.into();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Offset(
            delta.x,
            delta.y,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn picture<'a>(
    picture: impl Into<Picture>,
    target_rect: impl Into<Option<&'a Rect>>,
) -> Option<ImageFilter> {
    let picture = picture.into();
    let picture_rect = picture.cull_rect();
    let target_rect = target_rect.into().unwrap_or(&picture_rect);

    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Picture(picture.into_ptr(), target_rect.native())
    })
}

pub use skia_bindings::SkImageFilters_Dither as Dither;
variant_name!(Dither::Yes);

pub fn shader(shader: impl Into<Shader>, crop_rect: impl Into<CropRect>) -> Option<ImageFilter> {
    shader_with_dither(shader, Dither::No, crop_rect)
}

pub fn shader_with_dither(
    shader: impl Into<Shader>,
    dither: Dither,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Shader(shader.into().into_ptr(), dither, crop_rect.into().native())
    })
}

pub fn tile(
    src: impl AsRef<Rect>,
    dst: impl AsRef<Rect>,
    input: impl Into<Option<ImageFilter>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Tile(
            src.as_ref().native(),
            dst.as_ref().native(),
            input.into().into_ptr_or_null(),
        )
    })
}

pub fn dilate(
    (radius_x, radius_y): (scalar, scalar),
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Dilate(
            radius_x,
            radius_y,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn erode(
    (radius_x, radius_y): (scalar, scalar),
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Erode(
            radius_x,
            radius_y,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn distant_lit_diffuse(
    direction: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    kd: scalar,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_DistantLitDiffuse(
            direction.into().native(),
            light_color.into().into_native(),
            surface_scale,
            kd,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn point_lit_diffuse(
    location: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    kd: scalar,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_PointLitDiffuse(
            location.into().native(),
            light_color.into().into_native(),
            surface_scale,
            kd,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

#[allow(clippy::too_many_arguments)]
pub fn spot_lit_diffuse(
    location: impl Into<Point3>,
    target: impl Into<Point3>,
    specular_exponent: scalar,
    cutoff_angle: scalar,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    kd: scalar,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_SpotLitDiffuse(
            location.into().native(),
            target.into().native(),
            specular_exponent,
            cutoff_angle,
            light_color.into().into_native(),
            surface_scale,
            kd,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn distant_lit_specular(
    direction: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    ks: scalar,
    shininess: scalar,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_ImageFilters_DistantLitSpecular(
            direction.into().native(),
            light_color.into().into_native(),
            surface_scale,
            ks,
            shininess,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

pub fn point_lit_specular(
    location: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    ks: scalar,
    shininess: scalar,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_PointLitSpecular(
            location.into().native(),
            light_color.into().into_native(),
            surface_scale,
            ks,
            shininess,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

#[allow(clippy::too_many_arguments)]
pub fn spot_lit_specular(
    location: impl Into<Point3>,
    target: impl Into<Point3>,
    specular_exponent: scalar,
    cutoff_angle: scalar,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    ks: scalar,
    shininess: scalar,
    input: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<CropRect>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_SpotLitSpecular(
            location.into().native(),
            target.into().native(),
            specular_exponent,
            cutoff_angle,
            light_color.into().into_native(),
            surface_scale,
            ks,
            shininess,
            input.into().into_ptr_or_null(),
            crop_rect.into().native(),
        )
    })
}

impl ImageFilter {
    pub fn alpha_threshold<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        region: &Region,
        inner_min: scalar,
        outer_max: scalar,
    ) -> Option<Self> {
        alpha_threshold(
            region,
            inner_min,
            outer_max,
            self,
            crop_rect.into().map(|r| r.into()),
        )
    }

    pub fn arithmetic<'a>(
        inputs: impl Into<ArithmeticFPInputs>,
        background: impl Into<Option<Self>>,
        foreground: impl Into<Option<Self>>,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<Self> {
        let inputs = inputs.into();
        arithmetic(
            inputs.k[0],
            inputs.k[1],
            inputs.k[2],
            inputs.k[3],
            inputs.enforce_pm_color,
            background,
            foreground,
            crop_rect.into().map(|r| r.into()),
        )
    }

    pub fn blur<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        sigma: (scalar, scalar),
        tile_mode: impl Into<Option<crate::TileMode>>,
    ) -> Option<Self> {
        blur(sigma, tile_mode, self, crop_rect.into().map(|r| r.into()))
    }

    pub fn color_filter<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        cf: impl Into<ColorFilter>,
    ) -> Option<Self> {
        color_filter(cf, self, crop_rect.into().map(|r| r.into()))
    }

    pub fn compose(outer: impl Into<ImageFilter>, inner: impl Into<ImageFilter>) -> Option<Self> {
        compose(outer, inner)
    }

    pub fn displacement_map_effect<'a>(
        channel_selectors: (ColorChannel, ColorChannel),
        scale: scalar,
        displacement: impl Into<ImageFilter>,
        color: impl Into<ImageFilter>,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<Self> {
        displacement_map(
            channel_selectors,
            scale,
            displacement.into(),
            color,
            crop_rect.into().map(|r| r.into()),
        )
    }

    pub fn distant_lit_diffuse_lighting<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        direction: impl Into<Point3>,
        light_color: impl Into<Color>,
        surface_scale: scalar,
        kd: scalar,
    ) -> Option<Self> {
        distant_lit_diffuse(
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
        point_lit_diffuse(
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
        spot_lit_diffuse(
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
        distant_lit_specular(
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
        point_lit_specular(
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
        spot_lit_specular(
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

    #[deprecated(since = "0.64.0", note = "use magnifier2() instead")]
    pub fn magnifier<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        src_rect: impl AsRef<Rect>,
        inset: scalar,
    ) -> Option<Self> {
        magnifier(src_rect, inset, self, crop_rect.into().map(|r| r.into()))
    }

    pub fn magnifier2<'a>(
        self,
        lens_bounds: impl AsRef<Rect>,
        zoom_amount: scalar,
        inset: scalar,
        sampling_options: SamplingOptions,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<Self> {
        magnifier2(
            lens_bounds,
            zoom_amount,
            inset,
            sampling_options,
            self,
            crop_rect.into().map(|r| r.into()),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn matrix_convolution<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        kernel_size: impl Into<ISize>,
        kernel: &[scalar],
        gain: scalar,
        bias: scalar,
        kernel_offset: impl Into<IPoint>,
        tile_mode: crate::TileMode,
        convolve_alpha: bool,
    ) -> Option<Self> {
        matrix_convolution(
            kernel_size,
            kernel,
            gain,
            bias,
            kernel_offset,
            tile_mode,
            convolve_alpha,
            self,
            crop_rect.into().map(|r| r.into()),
        )
    }

    pub fn merge<'a>(
        filters: impl IntoIterator<Item = Option<Self>>,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<Self> {
        merge(filters, crop_rect.into().map(|r| r.into()))
    }

    pub fn dilate<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        radii: (scalar, scalar),
    ) -> Option<Self> {
        dilate(radii, self, crop_rect.into().map(|r| r.into()))
    }

    pub fn erode<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        radii: (scalar, scalar),
    ) -> Option<Self> {
        erode(radii, self, crop_rect.into().map(|r| r.into()))
    }

    pub fn offset<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        delta: impl Into<Vector>,
    ) -> Option<Self> {
        offset(delta, self, crop_rect.into().map(|r| r.into()))
    }

    pub fn from_picture<'a>(
        picture: impl Into<Picture>,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<Self> {
        self::picture(picture, crop_rect)
    }

    pub fn tile(self, src: impl AsRef<Rect>, dst: impl AsRef<Rect>) -> Option<Self> {
        tile(src, dst, self)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ArithmeticFPInputs {
    pub k: [f32; 4],
    pub enforce_pm_color: bool,
}

impl From<([f32; 4], bool)> for ArithmeticFPInputs {
    fn from((k, enforce_pm_color): ([f32; 4], bool)) -> Self {
        ArithmeticFPInputs {
            k,
            enforce_pm_color,
        }
    }
}

impl ArithmeticFPInputs {
    pub fn new(k0: f32, k1: f32, k2: f32, k3: f32, enforce_pm_color: bool) -> Self {
        Self {
            k: [k0, k1, k2, k3],
            enforce_pm_color,
        }
    }
}

impl Picture {
    pub fn as_image_filter<'a>(
        &self,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<ImageFilter> {
        self.clone().into_image_filter(crop_rect)
    }

    pub fn into_image_filter<'a>(
        self,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<ImageFilter> {
        picture(self, crop_rect)
    }
}

#[cfg(test)]
mod tests {
    use super::CropRect;
    use crate::{IRect, Rect};

    fn cr(crop_rect: impl Into<CropRect>) -> CropRect {
        crop_rect.into()
    }

    #[test]
    fn test_crop_conversion_options() {
        assert_eq!(cr(None), CropRect::NO_CROP_RECT);
        assert_eq!(cr(CropRect::NO_CROP_RECT), CropRect::NO_CROP_RECT);
        #[allow(clippy::needless_borrow)]
        let cr_ref = cr(&CropRect::NO_CROP_RECT);
        assert_eq!(cr_ref, CropRect::NO_CROP_RECT);
        let irect = IRect {
            left: 1,
            top: 2,
            right: 3,
            bottom: 4,
        };
        assert_eq!(cr(irect), CropRect(Rect::from(irect)));
        #[allow(clippy::needless_borrow)]
        let cr_by_ref = cr(&irect);
        assert_eq!(cr_by_ref, CropRect(Rect::from(irect)));
        let rect = Rect {
            left: 1.0,
            top: 2.0,
            right: 3.0,
            bottom: 4.0,
        };
        assert_eq!(cr(rect), CropRect(rect));
        #[allow(clippy::needless_borrow)]
        let cr_by_ref = cr(&rect);
        assert_eq!(cr_by_ref, CropRect(rect));
    }
}
