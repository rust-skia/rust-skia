use crate::prelude::*;
use crate::{
    gradient_shader, scalar, Color, ColorFilter, Image, Matrix, NativeFlattenable, Point, TileMode,
};
use skia_bindings as sb;
use skia_bindings::{SkFlattenable, SkRefCntBase, SkShader};

pub use skia_bindings::SkShader_GradientType as GradientTypeInternal;
#[test]
fn test_shader_gradient_type_naming() {
    let _ = GradientTypeInternal::Linear;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GradientType {
    Color,
    Linear(Point, Point),
    Radial(Point, scalar),
    Conical([(Point, scalar); 2]),
    Sweep(Point),
}

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
unsafe impl Send for Shader {}
unsafe impl Sync for Shader {}

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
        &self.base()
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { sb::C_SkShader_Deserialize(data.as_ptr() as _, data.len()) }
    }
}

impl Default for RCHandle<SkShader> {
    fn default() -> Self {
        shaders::empty()
    }
}

impl RCHandle<SkShader> {
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
            image.map(|i| (i, matrix, (tile_mode[0], tile_mode[1])))
        }
    }

    pub fn is_a_image(&self) -> bool {
        unsafe { sb::C_SkShader_isAImage(self.native()) }
    }

    pub fn with_local_matrix(&self, matrix: &Matrix) -> Self {
        Self::from_ptr(unsafe {
            sb::C_SkShader_makeWithLocalMatrix(self.native(), matrix.native())
        })
        .unwrap()
    }

    pub fn with_color_filter(&self, color_filter: impl Into<ColorFilter>) -> Self {
        Self::from_ptr(unsafe {
            sb::C_SkShader_makeWithColorFilter(self.native(), color_filter.into().into_ptr())
        })
        .unwrap()
    }
}

pub mod shaders {
    use crate::prelude::*;
    use crate::{BlendMode, Color, Color4f, ColorSpace, Matrix, Shader};
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

    pub fn blend(mode: BlendMode, dst: impl Into<Shader>, src: impl Into<Shader>) -> Shader {
        Shader::from_ptr(unsafe {
            sb::C_SkShaders_Blend(mode, dst.into().into_ptr(), src.into().into_ptr())
        })
        .unwrap()
    }

    pub fn lerp(t: f32, dst: impl Into<Shader>, src: impl Into<Shader>) -> Option<Shader> {
        Shader::from_ptr(unsafe {
            sb::C_SkShaders_Lerp(t, dst.into().into_ptr(), src.into().into_ptr())
        })
    }

    #[deprecated(since = "0.29.0", note = "removed without replacement")]
    pub fn lerp2(_red: Shader, _dst: Shader, _src: Shader, _local_matrix: Option<&Matrix>) -> ! {
        panic!("removed without replacement");
    }
}
