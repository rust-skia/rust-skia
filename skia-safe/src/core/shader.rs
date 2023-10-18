use crate::{
    gradient_shader, prelude::*, scalar, Color, ColorFilter, Image, Matrix, NativeFlattenable,
    TileMode,
};
use skia_bindings::{self as sb, SkFlattenable, SkRefCntBase, SkShader};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct GradientInfo<'a> {
    pub colors: &'a [Color],
    pub color_offsets: &'a [scalar],
    pub tile_mode: TileMode,
    pub gradient_flags: gradient_shader::Flags,
}

impl<'a> GradientInfo<'a> {
    pub fn color_count(&self) -> usize {
        self.colors.len()
    }
}

pub type Shader = RCHandle<SkShader>;
unsafe_send_sync!(Shader);
require_type_equality!(sb::SkShader_INHERITED, SkFlattenable);

impl NativeBase<SkRefCntBase> for SkShader {}
impl NativeBase<SkFlattenable> for SkShader {}

impl NativeRefCountedBase for SkShader {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        self.base()
    }
}

impl NativeFlattenable for SkShader {
    fn native_flattenable(&self) -> &SkFlattenable {
        self.base()
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { sb::C_SkShader_Deserialize(data.as_ptr() as _, data.len()) }
    }
}

impl Default for Shader {
    fn default() -> Self {
        shaders::empty()
    }
}

impl fmt::Debug for Shader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Shader")
            .field("is_opaque", &self.is_opaque())
            .field("image", &self.image())
            .finish()
    }
}

impl Shader {
    pub fn is_opaque(&self) -> bool {
        unsafe { sb::C_SkShader_isOpaque(self.native()) }
    }

    pub fn image(&self) -> Option<(Image, Matrix, (TileMode, TileMode))> {
        unsafe {
            let mut matrix = Matrix::default();
            let mut tile_mode = [TileMode::default(); 2];
            let image = Image::from_unshared_ptr(
                self.native()
                    .isAImage(matrix.native_mut(), tile_mode.as_mut_ptr()),
            );
            #[allow(clippy::tuple_array_conversions)]
            image.map(|i| (i, matrix, (tile_mode[0], tile_mode[1])))
        }
    }

    pub fn is_a_image(&self) -> bool {
        unsafe { sb::C_SkShader_isAImage(self.native()) }
    }

    #[must_use]
    pub fn with_local_matrix(&self, matrix: &Matrix) -> Self {
        Self::from_ptr(unsafe {
            sb::C_SkShader_makeWithLocalMatrix(self.native(), matrix.native())
        })
        .unwrap()
    }

    #[must_use]
    pub fn with_color_filter(&self, color_filter: impl Into<ColorFilter>) -> Self {
        Self::from_ptr(unsafe {
            sb::C_SkShader_makeWithColorFilter(self.native(), color_filter.into().into_ptr())
        })
        .unwrap()
    }
}

pub mod shaders {
    use crate::{prelude::*, Blender, Color, Color4f, ColorSpace, Rect, Shader};
    use skia_bindings as sb;

    pub fn empty() -> Shader {
        Shader::from_ptr(unsafe { sb::C_SkShaders_Empty() }).unwrap()
    }

    pub fn color(color: impl Into<Color>) -> Shader {
        let color = color.into();
        Shader::from_ptr(unsafe { sb::C_SkShaders_Color(color.into_native()) }).unwrap()
    }

    pub fn color_in_space(color: impl AsRef<Color4f>, space: impl Into<ColorSpace>) -> Shader {
        Shader::from_ptr(unsafe {
            sb::C_SkShaders_Color2(color.as_ref().native(), space.into().into_ptr())
        })
        .unwrap()
    }

    pub fn blend(
        blender: impl Into<Blender>,
        dst: impl Into<Shader>,
        src: impl Into<Shader>,
    ) -> Shader {
        Shader::from_ptr(unsafe {
            sb::C_SkShaders_Blend(
                blender.into().into_ptr(),
                dst.into().into_ptr(),
                src.into().into_ptr(),
            )
        })
        .unwrap()
    }

    pub fn coord_clamp(shader: impl Into<Shader>, rect: impl AsRef<Rect>) -> Option<Shader> {
        Shader::from_ptr(unsafe {
            sb::C_SkShaders_CoordClamp(shader.into().into_ptr(), rect.as_ref().native())
        })
    }
}
