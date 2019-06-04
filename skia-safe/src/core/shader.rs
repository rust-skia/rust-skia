use crate::prelude::*;
use crate::{Matrix, Image, Color, scalar, Point, ColorFilter, ColorSpace, Color4f, BlendMode, Bitmap, Rect, Picture, GradientShaderFlags};
use skia_bindings::{SkShader, SkRefCntBase, SkShader_TileMode, SkShader_GradientType, SkShader_GradientInfo, C_SkShader_asAGradient, C_SkShader_makeWithLocalMatrix, C_SkShader_makeWithColorFilter, C_SkShader_MakeEmptyShader, C_SkShader_MakeColorShader, C_SkShader_MakeColorShader2, C_SkShader_MakeCompose, C_SkShader_MakeMixer, C_SkShader_MakeBitmapShader, C_SkShader_MakePictureShader, C_SkShader_makeAsALocalMatrixShader, C_SkShader_isAImage};
use std::mem;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum TileMode {
    Clamp = SkShader_TileMode::kClamp_TileMode as _,
    Repeat = SkShader_TileMode::kRepeat_TileMode as _,
    Mirror = SkShader_TileMode::kMirror_TileMode as _,
    Decal = SkShader_TileMode::kDecal_TileMode as _
}

impl NativeTransmutable<SkShader_TileMode> for TileMode {}
#[test] fn test_shader_tile_mode_layout() { TileMode::test_layout() }

impl Default for TileMode {
    fn default() -> Self {
        TileMode::Clamp
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
#[allow(dead_code)]
enum GradientTypeInternal {
    None = SkShader_GradientType::kNone_GradientType as _,
    Color = SkShader_GradientType::kColor_GradientType as _,
    Linear = SkShader_GradientType::kLinear_GradientType as _,
    Radial = SkShader_GradientType::kRadial_GradientType as _,
    Sweep = SkShader_GradientType::kSweep_GradientType as _,
    Conical = SkShader_GradientType::kConical_GradientType as _,
}

impl NativeTransmutable<SkShader_GradientType> for GradientTypeInternal {}
#[test] fn test_shader_grandient_type_layout() { GradientTypeInternal::test_layout() }

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GradientType {
    Color,
    Linear(Point, Point),
    Radial(Point, scalar),
    Conical([(Point, scalar); 2]),
    Sweep(Point)
}

#[derive(Clone, PartialEq, Debug)]
pub struct GradientInfo<'a> {
    pub colors: &'a [Color],
    pub color_offsets: &'a [scalar],
    pub tile_mode: TileMode,
    pub gradient_flags: GradientShaderFlags
}

impl<'a> GradientInfo<'a> {
    pub fn color_count(&self) -> usize {
        self.colors.len()
    }
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
        Self::new_empty()
    }
}

impl RCHandle<SkShader> {

    pub fn local_matrix(&self) -> &Matrix {
        Matrix::from_native_ref(unsafe {
            &*self.native().getLocalMatrix()
        })
    }

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

    pub fn as_a_gradient<'a>(&self, colors: &'a mut [Color], color_offsets: &'a mut [scalar])
        -> Option<(GradientType, GradientInfo<'a>)> {
        assert_eq!(colors.len(), color_offsets.len());
        let max_color_count = colors.len();
        unsafe {
            let mut info = SkShader_GradientInfo {
                fColorCount: max_color_count.try_into().unwrap(),
                fColors: colors.native_mut().as_mut_ptr(),
                fColorOffsets: color_offsets.as_mut_ptr(),
                fPoint: mem::zeroed(),
                fRadius: Default::default(),
                fTileMode: SkShader_TileMode::kClamp_TileMode,
                fGradientFlags: 0
            };

            let gradient_type = GradientTypeInternal::from_native(C_SkShader_asAGradient(self.native(), &mut info));
            match gradient_type {
                GradientTypeInternal::None =>
                    None,
                GradientTypeInternal::Color =>
                    Some(GradientType::Color),
                GradientTypeInternal::Linear =>
                    Some(GradientType::Linear(Point::from_native(info.fPoint[0]), Point::from_native(info.fPoint[1]))),
                GradientTypeInternal::Radial =>
                    Some(GradientType::Radial(Point::from_native(info.fPoint[0]), info.fRadius[0])),
                GradientTypeInternal::Sweep =>
                    Some(GradientType::Sweep(Point::from_native(info.fPoint[0]))),
                GradientTypeInternal::Conical =>
                    Some(GradientType::Conical([(Point::from_native(info.fPoint[0]), info.fRadius[0]), (Point::from_native(info.fPoint[1]), info.fRadius[1])])),
            }.map(move |t| {
                let returned_color_count: usize = info.fColorCount.try_into().unwrap();
                assert!(returned_color_count <= max_color_count);
                (t, GradientInfo {
                    colors: &colors[0..returned_color_count],
                    color_offsets: &color_offsets[0..returned_color_count],
                    tile_mode: TileMode::Clamp,
                    gradient_flags: GradientShaderFlags::from_bits_truncate(info.fGradientFlags)
                })
            })
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

    pub fn new_empty() -> Self {
        Self::from_ptr(unsafe { C_SkShader_MakeEmptyShader() }).unwrap()
    }

    pub fn from_color(color: impl Into<Color>) -> Self {
        let color = color.into();
        Self::from_ptr(unsafe {
            C_SkShader_MakeColorShader(color.into_native())
        }).unwrap()
    }

    pub fn from_color_in_space(color: impl AsRef<Color4f>, space: &ColorSpace) -> Self {
        Self::from_ptr(unsafe {
            C_SkShader_MakeColorShader2(color.as_ref().native(), space.shared_native())
        }).unwrap()
    }

    pub fn compose(dst: &Shader, src: &Shader, mode: BlendMode, lerp: Option<scalar>) -> Option<Self> {
        let lerp = lerp.unwrap_or(1.0);
        Self::from_ptr(unsafe {
            C_SkShader_MakeCompose(dst.shared_native(), src.shared_native(), mode.into_native(), lerp)
        })
    }

    pub fn mixer(dst: &Shader, src: &Shader, lerp: scalar) -> Option<Self> {
        Self::from_ptr(unsafe {
            C_SkShader_MakeMixer(dst.shared_native(), src.shared_native(), lerp)
        })
    }

    pub fn from_bitmap(src: &Bitmap, (tmx, tmy): (TileMode, TileMode), local_matrix: Option<&Matrix>) -> Self {
        Self::from_ptr(unsafe {
            C_SkShader_MakeBitmapShader(src.native(), tmx.into_native(), tmy.into_native(), local_matrix.native_ptr_or_null())
        }).unwrap()
    }

    pub fn from_picture(src: &Picture, (tmx, tmy): (TileMode, TileMode), local_matrix: Option<&Matrix>, tile: Option<&Rect>) -> Self {
        Self::from_ptr(unsafe {
            C_SkShader_MakePictureShader(src.shared_native(), tmx.into_native(), tmy.into_native(), local_matrix.native_ptr_or_null(), tile.native_ptr_or_null())
        }).unwrap()
    }

    pub fn as_a_local_matrix_shader(&self) -> Option<(Self, Matrix)> {
        let mut matrix = Matrix::default();
        Self::from_ptr(unsafe {
            C_SkShader_makeAsALocalMatrixShader(self.native(), matrix.native_mut())
        }).map(|s| (s, matrix))
    }
}
