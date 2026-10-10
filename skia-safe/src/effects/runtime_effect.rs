//! Support for creating custom [`crate::Shader`]s and [`crate::ColorFilter`]s from Skia's SkSL
//! shading language. This API is experimental and subject to change.

use crate::{
    Blender, ColorFilter, Data, Matrix, Shader,
    interop::{self, AsStr},
    prelude::*,
};
use core::ffi;
use sb::{SkFlattenable, SkRuntimeEffect_Child};
use skia_bindings::{
    self as sb, ShaderBuilderUniformResult, SkRefCntBase, SkRuntimeEffect, SkRuntimeEffect_Uniform,
};
use std::{fmt, marker::PhantomData, ops::DerefMut, ptr};

/// Reflected description of a uniform variable in the effect's SkSL.
pub type Uniform = Handle<SkRuntimeEffect_Uniform>;
unsafe_send_sync!(Uniform);

#[deprecated(since = "0.35.0", note = "Use Uniform instead")]
pub type Variable = Uniform;

impl NativeDrop for SkRuntimeEffect_Uniform {
    fn drop(&mut self) {
        panic!("native type SkRuntimeEffect::Uniform can't be owned by Rust");
    }
}

impl fmt::Debug for Uniform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.native().fmt(f)
    }
}

impl Uniform {
    /// The name of the uniform variable in the effect's SkSL.
    pub fn name(&self) -> &str {
        self.native().name.as_str()
    }

    /// The offset in bytes of the uniform within the uniform data block.
    pub fn offset(&self) -> usize {
        self.native().offset
    }

    /// The SkSL type of the uniform.
    pub fn ty(&self) -> uniform::Type {
        self.native().type_
    }

    /// The number of elements in the uniform. 1 for non-array uniforms.
    pub fn count(&self) -> i32 {
        self.native().count
    }

    /// The flags of the uniform, see [`uniform::Flags`].
    pub fn flags(&self) -> uniform::Flags {
        uniform::Flags::from_bits(self.native().flags).unwrap()
    }

    /// Returns true if the uniform is declared as an array. [`Uniform::count()`] contains the
    /// array length.
    pub fn is_array(&self) -> bool {
        self.flags().contains(uniform::Flags::ARRAY)
    }

    /// Returns true if the uniform is declared with `layout(color)`. Colors should be supplied
    /// as unpremultiplied, extended-range (unclamped) sRGB. The uniform will be automatically
    /// transformed to unpremultiplied extended-range working-space colors.
    pub fn is_color(&self) -> bool {
        self.flags().contains(uniform::Flags::COLOR)
    }

    /// The size in bytes of the uniform (or the whole array, for array uniforms).
    pub fn size_in_bytes(&self) -> usize {
        unsafe { self.native().sizeInBytes() }
    }
}

pub mod uniform {
    //! Types that describe the uniforms of a [`crate::RuntimeEffect`], e.g. [`Type`] and
    //! [`Flags`].

    use skia_bindings as sb;

    pub use sb::SkRuntimeEffect_Uniform_Type as Type;
    variant_name!(Type::Float2x2);

    bitflags! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct Flags : u32 {
            const ARRAY = sb::SkRuntimeEffect_Uniform_Flags_kArray_Flag as _;
            const COLOR = sb::SkRuntimeEffect_Uniform_Flags_kColor_Flag as _;
            const VERTEX = sb::SkRuntimeEffect_Uniform_Flags_kVertex_Flag as _;
            const FRAGMENT = sb::SkRuntimeEffect_Uniform_Flags_kFragment_Flag as _;
            const HALF_PRECISION = sb::SkRuntimeEffect_Uniform_Flags_kHalfPrecision_Flag as _;
        }
    }
}

pub use sb::SkRuntimeEffect_ChildType as ChildType;
variant_name!(ChildType::Shader);

#[deprecated(since = "0.41.0", note = "Use Child")]
pub type Varying = Child;

/// Reflected description of a uniform child (shader, color filter, or blender) in the
/// effect's SkSL.
pub type Child = Handle<SkRuntimeEffect_Child>;
unsafe_send_sync!(Child);

impl NativeDrop for SkRuntimeEffect_Child {
    fn drop(&mut self) {
        panic!("native type SkRuntimeEffect::Child can't be owned in Rust");
    }
}

impl fmt::Debug for Child {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Child")
            .field("name", &self.name())
            .field("type", &self.ty())
            .field("index", &self.index())
            .finish()
    }
}

impl Child {
    /// The name of the child in the effect's SkSL.
    pub fn name(&self) -> &str {
        self.native().name.as_str()
    }

    /// The [`ChildType`] of the child.
    pub fn ty(&self) -> ChildType {
        self.native().type_
    }

    /// The index of the child in [`RuntimeEffect::children()`].
    pub fn index(&self) -> usize {
        self.native().index.try_into().unwrap()
    }
}

/// [`RuntimeEffect`] supports creating custom [`Shader`] and [`ColorFilter`] objects using
/// Skia's SkSL shading language.
pub type RuntimeEffect = RCHandle<SkRuntimeEffect>;

impl NativeRefCountedBase for SkRuntimeEffect {
    type Base = SkRefCntBase;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
/// Options for creating a [`RuntimeEffect`].
pub struct Options<'a> {
    /// For testing purposes, disables optimization and inlining. (Normally, runtime effects
    /// don't run the inliner directly, but they still get an inlining pass once they are
    /// painted.)
    pub force_unoptimized: bool,
    /// When possible this name will be used to identify the created runtime effect.
    pub name: &'a str,
}

impl fmt::Debug for RuntimeEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeEffect")
            .field("uniform_size", &self.uniform_size())
            .field("uniforms", &self.uniforms())
            .field("children", &self.children())
            .field("allow_shader", &self.allow_shader())
            .field("allow_color_filter", &self.allow_color_filter())
            .field("allow_blender", &self.allow_blender())
            .finish()
    }
}

impl RuntimeEffect {
    /// Creates a runtime effect for use as a [`ColorFilter`].
    ///
    /// Color filter SkSL requires an entry point that looks like:
    ///
    /// ```text
    /// vec4 main(vec4 inColor) { ... }
    /// ```
    ///
    /// - `sksl` the SkSL source code of the effect
    /// - `options` options for creating the effect
    ///
    /// Returns the effect, or the compiler error message on failure.
    pub fn make_for_color_filter(
        sksl: impl AsRef<str>,
        options: Option<&Options<'_>>,
    ) -> Result<RuntimeEffect, String> {
        let str = interop::String::from_str(sksl);
        let options = options.copied().unwrap_or_default();
        let options = Self::construct_native_options(&options);
        let mut error = interop::String::default();
        RuntimeEffect::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_MakeForColorFilter(str.native(), &options, error.native_mut())
        })
        .ok_or_else(|| error.to_string())
    }

    /// Creates a runtime effect for use as a [`Shader`].
    ///
    /// Shader SkSL requires an entry point that looks like:
    ///
    /// ```text
    /// vec4 main(vec2 inCoords) { ... }
    /// ```
    ///
    /// The color that is returned should be premultiplied.
    ///
    /// - `sksl` the SkSL source code of the effect
    /// - `options` options for creating the effect
    ///
    /// Returns the effect, or the compiler error message on failure.
    pub fn make_for_shader(
        sksl: impl AsRef<str>,
        options: Option<&Options<'_>>,
    ) -> Result<RuntimeEffect, String> {
        let str = interop::String::from_str(sksl);
        let options = options.copied().unwrap_or_default();
        let options = Self::construct_native_options(&options);
        let mut error = interop::String::default();
        RuntimeEffect::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_MakeForShader(str.native(), &options, error.native_mut())
        })
        .ok_or_else(|| error.to_string())
    }

    /// Creates a runtime effect for use as a [`Blender`].
    ///
    /// Blend SkSL requires an entry point that looks like:
    ///
    /// ```text
    /// vec4 main(vec4 srcColor, vec4 dstColor) { ... }
    /// ```
    ///
    /// - `sksl` the SkSL source code of the effect
    /// - `options` options for creating the effect
    ///
    /// Returns the effect, or the compiler error message on failure.
    pub fn make_for_blender(
        sksl: impl AsRef<str>,
        options: Option<&Options<'_>>,
    ) -> Result<RuntimeEffect, String> {
        let str = interop::String::from_str(sksl);
        let options = options.copied().unwrap_or_default();
        let options = Self::construct_native_options(&options);
        let mut error = interop::String::default();
        RuntimeEffect::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_MakeForBlender(str.native(), &options, error.native_mut())
        })
        .ok_or_else(|| error.to_string())
    }

    fn construct_native_options(options: &Options<'_>) -> sb::SkRuntimeEffect_Options {
        construct(|opt| unsafe {
            sb::C_SkRuntimeEffect_Options_Construct(
                opt,
                options.force_unoptimized,
                options.name.as_ptr() as *const ffi::c_char,
                options.name.len(),
            )
        })
    }

    /// Creates a [`Shader`] from this effect.
    ///
    /// - `uniforms` a [`Data`] block of size [`RuntimeEffect::uniform_size()`], containing
    ///   values for all uniform variables
    /// - `children` the child shaders/color filters/blenders required by the effect, in the
    ///   order given by [`RuntimeEffect::children()`]
    /// - `local_matrix` an optional local matrix applied to the shader
    pub fn make_shader<'a>(
        &self,
        uniforms: impl Into<Data>,
        children: &[ChildPtr],
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Shader> {
        let mut children: Vec<_> = children
            .iter()
            .map(|child_ptr| child_ptr.native())
            .collect();
        let children_ptr = children
            .first_mut()
            .map(|c| c.deref_mut() as *mut _)
            .unwrap_or(ptr::null_mut());
        Shader::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_makeShader(
                self.native(),
                uniforms.into().into_ptr(),
                children_ptr,
                children.len(),
                local_matrix.into().native_ptr_or_null(),
            )
        })
    }

    /// Creates a [`ColorFilter`] from this effect.
    ///
    /// - `inputs` a [`Data`] block of size [`RuntimeEffect::uniform_size()`], containing
    ///   values for all uniform variables
    /// - `children` the child color filters/shaders required by the effect
    pub fn make_color_filter<'a>(
        &self,
        inputs: impl Into<Data>,
        children: impl Into<Option<&'a [ChildPtr]>>,
    ) -> Option<ColorFilter> {
        let mut children: Vec<_> = children
            .into()
            .map(|c| c.iter().map(|child_ptr| child_ptr.native()).collect())
            .unwrap_or_default();
        let children_ptr = children
            .first_mut()
            .map(|c| c.deref_mut() as *mut _)
            .unwrap_or(ptr::null_mut());
        ColorFilter::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_makeColorFilter(
                self.native(),
                inputs.into().into_ptr(),
                children_ptr,
                children.len(),
            )
        })
    }

    /// Creates a [`Blender`] from this effect.
    ///
    /// - `uniforms` a [`Data`] block of size [`RuntimeEffect::uniform_size()`], containing
    ///   values for all uniform variables
    /// - `children` the child blenders/color filters/shaders required by the effect
    pub fn make_blender<'a>(
        &self,
        uniforms: impl Into<Data>,
        children: impl Into<Option<&'a [ChildPtr]>>,
    ) -> Option<Blender> {
        let mut children: Vec<_> = children
            .into()
            .map(|c| c.iter().map(|child_ptr| child_ptr.native()).collect())
            .unwrap_or_default();
        let children_ptr = children
            .first_mut()
            .map(|c| c.deref_mut() as *mut _)
            .unwrap_or(ptr::null_mut());
        Blender::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_makeBlender(
                self.native(),
                uniforms.into().into_ptr(),
                children_ptr,
                children.len(),
            )
        })
    }

    // TODO: wrap MakeTraced

    /// Returns the SkSL source of the runtime effect shader.
    pub fn source(&self) -> &str {
        let mut len = 0;
        let ptr = unsafe { sb::C_SkRuntimeEffect_source(self.native(), &mut len) };
        std::str::from_utf8(unsafe { safer::from_raw_parts(ptr, len) }).unwrap()
    }

    #[deprecated(since = "0.35.0", note = "Use uniform_size() instead")]
    pub fn input_size(&self) -> usize {
        self.uniform_size()
    }

    /// Combined size of all uniform variables. When calling
    /// [`RuntimeEffect::make_color_filter()`] or [`RuntimeEffect::make_shader()`], provide a
    /// [`Data`] of this size, containing values for all of those variables.
    pub fn uniform_size(&self) -> usize {
        unsafe { self.native().uniformSize() }
    }

    #[deprecated(since = "0.35.0", note = "Use uniforms() instead")]
    pub fn inputs(&self) -> &[Uniform] {
        self.uniforms()
    }

    /// The descriptions of all uniform variables in the effect's SkSL.
    pub fn uniforms(&self) -> &[Uniform] {
        unsafe {
            let mut count: usize = 0;
            let ptr = sb::C_SkRuntimeEffect_uniforms(self.native(), &mut count);
            safer::from_raw_parts(Uniform::from_native_ptr(ptr), count)
        }
    }

    /// The descriptions of all child effects in the effect's SkSL.
    pub fn children(&self) -> &[Child] {
        unsafe {
            let mut count: usize = 0;
            let ptr = sb::C_SkRuntimeEffect_children(self.native(), &mut count);
            safer::from_raw_parts(Child::from_native_ptr(ptr), count)
        }
    }

    #[deprecated(since = "0.35.0", note = "Use find_uniform()")]
    pub fn find_input(&self, name: impl AsRef<str>) -> Option<&Uniform> {
        self.find_uniform(name)
    }

    /// Returns the description of the named uniform variable, or `None` if not found.
    pub fn find_uniform(&self, name: impl AsRef<str>) -> Option<&Uniform> {
        let name = name.as_ref().as_bytes();
        unsafe { sb::C_SkRuntimeEffect_findUniform(self.native(), name.as_ptr() as _, name.len()) }
            .into_non_null()
            .map(|ptr| Uniform::from_native_ref(unsafe { ptr.as_ref() }))
    }

    /// Returns the description of the named child, or `None` if not found.
    pub fn find_child(&self, name: impl AsRef<str>) -> Option<&Child> {
        let name = name.as_ref().as_bytes();
        unsafe { sb::C_SkRuntimeEffect_findChild(self.native(), name.as_ptr() as _, name.len()) }
            .into_non_null()
            .map(|ptr| Child::from_native_ref(unsafe { ptr.as_ref() }))
    }

    /// Returns whether this effect can be used as a [`Shader`].
    pub fn allow_shader(&self) -> bool {
        unsafe { sb::C_SkRuntimeEffect_allowShader(self.native()) }
    }

    /// Returns whether this effect can be used as a [`ColorFilter`].
    pub fn allow_color_filter(&self) -> bool {
        unsafe { sb::C_SkRuntimeEffect_allowColorFilter(self.native()) }
    }

    /// Returns whether this effect can be used as a [`Blender`].
    pub fn allow_blender(&self) -> bool {
        unsafe { sb::C_SkRuntimeEffect_allowBlender(self.native()) }
    }
}

#[derive(Clone, Debug)]
/// Object that allows passing a [`Shader`], [`ColorFilter`], or [`Blender`] as a child to
/// [`RuntimeEffect::make_shader()`] and friends.
pub enum ChildPtr {
    Shader(Shader),
    ColorFilter(ColorFilter),
    Blender(Blender),
}

impl From<Shader> for ChildPtr {
    fn from(shader: Shader) -> Self {
        Self::Shader(shader)
    }
}

impl From<ColorFilter> for ChildPtr {
    fn from(color_filter: ColorFilter) -> Self {
        Self::ColorFilter(color_filter)
    }
}

impl From<Blender> for ChildPtr {
    fn from(blender: Blender) -> Self {
        Self::Blender(blender)
    }
}

// TODO: Create `ChildPtr` from a Flattenable?

impl ChildPtr {
    /// The [`ChildType`] of this child.
    pub fn ty(&self) -> ChildType {
        match self {
            ChildPtr::Shader(_) => ChildType::Shader,
            ChildPtr::ColorFilter(_) => ChildType::ColorFilter,
            ChildPtr::Blender(_) => ChildType::Blender,
        }
    }

    // We are treating [`ChildPtr`]s as a _reference_ to a smart pointer: no reference counters are
    // changed (no drop() is called either).
    //
    // Skia will copy the pointers and increase the reference counters if it uses the actual
    // objects.
    pub(crate) fn native(&self) -> Borrows<sb::SkRuntimeEffect_ChildPtr> {
        let flattenable: *mut SkFlattenable = match self {
            // casting to &T &mut T is UB, so we don't use the base() indirection and directly cast
            // to a pointer.
            ChildPtr::Shader(shader) => unsafe { shader.native_mut_force() as _ },
            ChildPtr::ColorFilter(color_filter) => unsafe { color_filter.native_mut_force() as _ },
            ChildPtr::Blender(blender) => unsafe { blender.native_mut_force() as _ },
        };

        sb::SkRuntimeEffect_ChildPtr {
            fChild: sb::sk_sp {
                fPtr: flattenable,
                _phantom_0: PhantomData,
            },
        }
        .borrows(self)
    }
}

// TODO: wrap SkRuntimeEffectBuilder, SkRuntimeColorFilterBuilder,
// SkRuntimeBlendBuilder

pub type RuntimeShaderBuilder = Handle<sb::SkRuntimeShaderBuilder>;
unsafe_send_sync!(RuntimeShaderBuilder);

#[derive(Debug)]
pub enum ShaderBuilderError {
    UniformSizeNotSupported,
}
impl fmt::Display for ShaderBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShaderBuilderError::UniformSizeNotSupported => write!(f, "Unsupported uniform size"),
        }
    }
}
impl std::error::Error for ShaderBuilderError {}

impl NativeDrop for sb::SkRuntimeShaderBuilder {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkRuntimeShaderBuilder_destruct(self);
        }
    }
}

impl RuntimeShaderBuilder {
    /// Creates a builder for `effect` that manages creating an input data block and provides
    /// named access to the uniform variables in that block. The equivalent of Skia's
    /// SkRuntimeEffectBuilder.
    pub fn new(effect: RuntimeEffect) -> Self {
        Self::construct(|builder| unsafe {
            let effect: *mut SkRuntimeEffect = effect.into_ptr() as _;
            sb::C_SkRuntimeShaderBuilder_Construct(builder, effect)
        })
    }

    /// Creates the [`Shader`] from the configured builder.
    ///
    /// - `local_matrix` the local matrix applied to the resulting shader
    pub fn make_shader(&self, local_matrix: &Matrix) -> Option<Shader> {
        unsafe {
            let instance = self.native_mut_force();
            let shader =
                sb::C_SkRuntimeShaderBuilder_makeShader(instance, local_matrix.native() as _);
            Shader::from_ptr(shader)
        }
    }
    /// Set float uniform values by name.
    ///
    /// Supported types are `float`, `float2`, `float3`, `float4`, `float2x2`, `float3x3`, `float4x4`.
    ///
    /// The data array must have the correct length for the corresponding uniform type:
    /// - `float` `[f32; 1]`
    /// - `float2` `[f32; 2]`
    /// - `float3` `[f32; 3]`
    /// - `float4` `[f32; 4]`
    /// - `float2x2` `[f32; 4]`
    /// - `float3x3` `[f32; 9]`
    /// - `float4x4` `[f32; 16]`
    ///
    pub fn set_uniform_float(
        &mut self,
        name: impl AsRef<str>,
        data: &[f32],
    ) -> Result<(), ShaderBuilderError> {
        let name = name.as_ref();
        let result = unsafe {
            sb::C_SkRuntimeShaderBuilder_setUniformFloat(
                self.native_mut() as _,
                name.as_bytes().as_ptr() as _,
                name.len(),
                data.as_ptr() as _,
                data.len(),
            )
        };
        match result {
            ShaderBuilderUniformResult::Ok => Ok(()),
            ShaderBuilderUniformResult::Error => Err(ShaderBuilderError::UniformSizeNotSupported),
        }
    }
    /// Set int uniform values by name.
    ///
    /// Supported types are `int`, `int2`, `int3`, `int4`.
    ///
    /// The data array must have the correct length for the corresponding uniform type:
    /// - `int` `[i32; 1]`
    /// - `int2` `[i32; 2]`
    /// - `int3` `[i32; 3]`
    /// - `int4` `[i32; 4]`
    ///
    ///
    pub fn set_uniform_int(
        &mut self,
        name: impl AsRef<str>,
        data: &[i32],
    ) -> Result<(), ShaderBuilderError> {
        let name = name.as_ref();
        let result = unsafe {
            sb::C_SkRuntimeShaderBuilder_setUniformInt(
                self.native_mut() as _,
                name.as_bytes().as_ptr() as _,
                name.len(),
                data.as_ptr() as _,
                data.len(),
            )
        };
        match result {
            ShaderBuilderUniformResult::Ok => Ok(()),
            ShaderBuilderUniformResult::Error => Err(ShaderBuilderError::UniformSizeNotSupported),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // <https://github.com/rust-skia/rust-skia/discussions/1133>
    #[allow(unused)]
    fn none_cases_compile() {
        RuntimeEffect::make_for_color_filter("", None);
        RuntimeEffect::make_for_shader("", None);
        RuntimeEffect::make_for_blender("", None);
    }
}
