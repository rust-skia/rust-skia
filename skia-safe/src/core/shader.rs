use crate::prelude::*;
use crate::{Matrix, Image, Color, scalar, Point, ColorFilter, ColorSpace, Color4f, BlendMode, TileMode};
use skia_bindings::{SkShader, SkRefCntBase, SkShader_GradientType, SkShader_GradientInfo, C_SkShader_asAGradient, C_SkShader_makeWithLocalMatrix, C_SkShader_makeWithColorFilter, C_SkShader_isAImage, SkTileMode, C_SkShaders_Empty, C_SkShaders_Color, C_SkShaders_Color2, C_SkShaders_Blend, C_SkShaders_Lerp, C_SkShaders_Lerp2};
use std::mem;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ShaderGradientType {
    None = SkShader_GradientType::kNone_GradientType as _,
    Color = SkShader_GradientType::kColor_GradientType as _,
    Linear = SkShader_GradientType::kLinear_GradientType as _,
    Radial = SkShader_GradientType::kRadial_GradientType as _,
    Sweep = SkShader_GradientType::kSweep_GradientType as _,
    Conical = SkShader_GradientType::kConical_GradientType as _,
}

impl NativeTransmutable<SkShader_GradientType> for ShaderGradientType {}
#[test] fn test_shader_grandient_type_layout() { ShaderGradientType::test_layout() }

pub struct ShaderGradientInfo<'a> {
    pub colors: &'a [Color],
    pub color_offsets: &'a [scalar],
    pub point: (Point, Point),
    pub radius: (scalar, scalar),
    pub tile_mode: TileMode
}

pub type Shader = RCHandle<SkShader>;

impl NativeRefCountedBase for SkShader {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl Default for RCHandle<SkShader> {
    fn default() -> Self {
        Shaders::empty()
    }
}

impl RCHandle<SkShader> {

    pub fn is_opaque(&self) -> bool {
        unsafe {
            skia_bindings::C_SkShader_isOpaque(self.native())
        }
    }

    pub fn image(&self) -> Option<(Image, Matrix, (TileMode, TileMode))> {
        unsafe {
            let mut matrix = Matrix::default();
            let mut tile_mode : [TileMode; 2] = mem::zeroed();
            let image =
                Image::from_unshared_ptr(self.native().isAImage(matrix.native_mut(), tile_mode.native_mut().as_mut_ptr()));
            image.map(|i| (i, matrix, (tile_mode[0], tile_mode[1])))
        }
    }

    pub fn is_a_image(&self) -> bool {
        unsafe {
            // does not link under Windows.
            // self.native().isAImage1()
            C_SkShader_isAImage(self.native())
        }
    }

    #[deprecated(since="0.6.0", note="skbug.com/8941")]
    pub fn as_a_gradient<'a>(&self, colors: &'a mut [Color], color_offsets: &'a mut [scalar])
        -> (ShaderGradientType, ShaderGradientInfo<'a>) {
        assert_eq!(colors.len(), color_offsets.len());
        let max_color_count = colors.len();
        unsafe {
            let mut info = SkShader_GradientInfo {
                fColorCount: max_color_count.try_into().unwrap(),
                fColors: colors.native_mut().as_mut_ptr(),
                fColorOffsets: color_offsets.as_mut_ptr(),
                fPoint: mem::zeroed(),
                fRadius: Default::default(),
                fTileMode: SkTileMode::kClamp,
                fGradientFlags: 0
            };

            let gradient_type = C_SkShader_asAGradient(self.native(), &mut info);
            let returned_color_count : usize = info.fColorCount.try_into().unwrap();
            assert!(returned_color_count <= max_color_count);
            let info = ShaderGradientInfo {
                colors: &colors[0..returned_color_count],
                color_offsets: &color_offsets[0..returned_color_count],
                point: (Point::from_native(info.fPoint[0]), Point::from_native(info.fPoint[1])),
                radius: (info.fRadius[0], info.fRadius[1]),
                // TODO: tile mode should be converted from the returned info record.
                tile_mode: TileMode::Clamp
            };
            (ShaderGradientType::from_native(gradient_type), info)
        }
    }

    pub fn with_local_matrix(&self, matrix: &Matrix) -> Self {
        Self::from_ptr(unsafe {
            C_SkShader_makeWithLocalMatrix(self.native(), matrix.native())
        }).unwrap()
    }

    pub fn with_color_filter(&self, color_filter: &ColorFilter) -> Self {
        Self::from_ptr(unsafe {
            C_SkShader_makeWithColorFilter(self.native(), color_filter.shared_native())
        }).unwrap()
    }
}

pub enum Shaders {}

impl Shaders {
    pub fn empty() -> Shader {
        Shader::from_ptr(unsafe {
            C_SkShaders_Empty()
        }).unwrap()
    }

    pub fn color<C: Into<Color>>(color: C) -> Shader {
        let color = color.into();
        Shader::from_ptr(unsafe {
            C_SkShaders_Color(color.into_native())
        }).unwrap()
    }

    pub fn color_in_space<C: AsRef<Color4f>>(color: C, space: &ColorSpace) -> Shader {
        Shader::from_ptr(unsafe {
            C_SkShaders_Color2(color.as_ref().native(), space.shared_native())
        }).unwrap()
    }

    pub fn blend(mode: BlendMode, dst: &Shader, src: &Shader) -> Shader {
        Shader::from_ptr(unsafe {
            C_SkShaders_Blend(mode.into_native(), dst.shared_native(), src.shared_native())
        }).unwrap()
    }

    pub fn lerp(t: f32, dst: &Shader, src: &Shader) -> Option<Shader> {
        Shader::from_ptr(unsafe {
            C_SkShaders_Lerp(t, dst.shared_native(), src.shared_native())
        })
    }

    // TODO: rename as soon it's clear from the documentation what it does.
    pub fn lerp2(red: &Shader, dst: &Shader, src: &Shader) -> Shader {
        Shader::from_ptr(unsafe {
            C_SkShaders_Lerp2(red.shared_native(), dst.shared_native(), src.shared_native())
        }).unwrap()
    }
}
