use crate::interop::AsStr;
use crate::prelude::*;
use crate::{interop, ColorFilter, Data, Matrix, Shader};
use skia_bindings as sb;
use skia_bindings::{
    SkRefCntBase, SkRuntimeEffect, SkRuntimeEffect_Uniform, SkRuntimeEffect_Varying,
};
use std::ffi::CStr;
use std::slice;

pub type Uniform = Handle<SkRuntimeEffect_Uniform>;

#[deprecated(since = "0.35.0", note = "Use Uniform instead")]
pub type Variable = Uniform;

unsafe impl Send for Uniform {}
unsafe impl Sync for Uniform {}

impl NativeDrop for SkRuntimeEffect_Uniform {
    fn drop(&mut self) {
        panic!("native type SkRuntimeEffect::Uniform can't be owned by Rust");
    }
}

impl Uniform {
    pub fn name(&self) -> &str {
        self.native().fName.as_str()
    }

    pub fn offset(&self) -> usize {
        self.native().fOffset
    }

    pub fn ty(&self) -> uniform::Type {
        self.native().fType
    }

    pub fn count(&self) -> i32 {
        self.native().fCount
    }

    pub fn flags(&self) -> uniform::Flags {
        uniform::Flags::from_bits(self.native().fFlags).unwrap()
    }

    pub fn marker(&self) -> u32 {
        self.native().fMarker
    }

    #[cfg(feature = "gpu")]
    pub fn gpu_type(&self) -> crate::private::gpu::SLType {
        self.native().fGPUType
    }

    pub fn is_array(&self) -> bool {
        self.flags().contains(uniform::Flags::ARRAY)
    }

    pub fn size_in_bytes(&self) -> usize {
        unsafe { self.native().sizeInBytes() }
    }
}

pub mod uniform {
    use skia_bindings as sb;

    pub use sb::SkRuntimeEffect_Uniform_Type as Type;
    #[test]
    fn test_type_naming() {
        let _ = Type::Float2x2;
    }

    bitflags! {
        pub struct Flags : u32 {
            const ARRAY = sb::SkRuntimeEffect_Uniform_Flags_kArray_Flag as _;
            const MARKER = sb::SkRuntimeEffect_Uniform_Flags_kMarker_Flag as _;
            const MARKER_NORMALS = sb::SkRuntimeEffect_Uniform_Flags_kMarkerNormals_Flag as _;
            const SRGB_UNPREMUL = sb::SkRuntimeEffect_Uniform_Flags_kSRGBUnpremul_Flag as _;
        }
    }
}

pub type Varying = Handle<SkRuntimeEffect_Varying>;
unsafe impl Send for Varying {}
unsafe impl Sync for Varying {}

impl NativeDrop for SkRuntimeEffect_Varying {
    fn drop(&mut self) {
        panic!("native type SkRuntimeEffect::Varying can't be owned by Rust");
    }
}

impl Varying {
    pub fn name(&self) -> &str {
        self.native().fName.as_str()
    }

    pub fn width(&self) -> i32 {
        self.native().fWidth
    }
}

pub type RuntimeEffect = RCHandle<SkRuntimeEffect>;

impl NativeRefCountedBase for SkRuntimeEffect {
    type Base = SkRefCntBase;
}

pub fn new(sksl: impl AsRef<str>) -> Result<RuntimeEffect, String> {
    let str = interop::String::from_str(sksl);
    let mut error = interop::String::default();
    let effect = RuntimeEffect::from_ptr(unsafe {
        sb::C_SkRuntimeEffect_Make(str.native(), error.native_mut())
    });
    match effect {
        Some(runtime_effect) => Ok(runtime_effect),
        None => Err(error.as_str().to_owned()),
    }
}

impl RuntimeEffect {
    pub fn make_shader<'a>(
        &mut self,
        inputs: impl Into<Data>,
        children: impl IntoIterator<Item = Shader>,
        local_matrix: impl Into<Option<&'a Matrix>>,
        is_opaque: bool,
    ) -> Option<Shader> {
        let mut children: Vec<_> = children
            .into_iter()
            .map(|shader| shader.into_ptr())
            .collect();
        Shader::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_makeShader(
                self.native_mut(),
                inputs.into().into_ptr(),
                children.as_mut_ptr(),
                children.len(),
                local_matrix.into().native_ptr_or_null(),
                is_opaque,
            )
        })
    }

    #[deprecated(since = "0.33.0", note = "removed without replacement")]
    pub fn make_color_filter_with_children(
        &mut self,
        _inputs: impl Into<Data>,
        _children: impl IntoIterator<Item = ColorFilter>,
    ) -> ! {
        panic!("removed without replacement")
    }

    pub fn make_color_filter(&mut self, inputs: impl Into<Data>) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_makeColorFilter(self.native_mut(), inputs.into().into_ptr())
        })
    }

    pub fn source(&self) -> &str {
        unsafe { (*sb::C_SkRuntimeEffect_source(self.native())).as_str() }
    }

    #[deprecated(since = "0.29.0", note = "removed without replacement")]
    pub fn index(&self) -> ! {
        unimplemented!("removed without replacement")
    }

    #[deprecated(since = "0.35.0", note = "Use uniform_size() instead")]
    pub fn input_size(&self) -> usize {
        self.uniform_size()
    }

    pub fn uniform_size(&self) -> usize {
        unsafe { self.native().uniformSize() }
    }

    #[deprecated(since = "0.35.0", note = "Use uniforms() instead")]
    pub fn inputs(&self) -> &[Uniform] {
        self.uniforms()
    }

    pub fn uniforms(&self) -> &[Uniform] {
        unsafe {
            let mut count: usize = 0;
            let ptr = sb::C_SkRuntimeEffect_uniforms(self.native(), &mut count);
            slice::from_raw_parts(Uniform::from_native_ref(&*ptr), count)
        }
    }

    pub fn children(&self) -> impl Iterator<Item = &str> {
        unsafe {
            let mut count: usize = 0;
            let ptr = sb::C_SkRuntimeEffect_children(self.native(), &mut count);
            let slice = slice::from_raw_parts(ptr, count);
            slice.iter().map(|str| str.as_str())
        }
    }

    pub fn varyings(&self) -> &[Varying] {
        unsafe {
            let mut count: usize = 0;
            let ptr = sb::C_SkRuntimeEffect_varyings(self.native(), &mut count);
            slice::from_raw_parts(Varying::from_native_ref(&*ptr), count)
        }
    }

    #[deprecated(since = "0.35.0", note = "Use find_uniform()")]
    pub fn find_input(&self, name: impl AsRef<CStr>) -> Option<&Uniform> {
        self.find_uniform(name)
    }

    pub fn find_uniform(&self, name: impl AsRef<CStr>) -> Option<&Uniform> {
        unsafe { self.native().findUniform(name.as_ref().as_ptr()) }
            .into_option()
            .map(|ptr| Uniform::from_native_ref(unsafe { &*ptr }))
    }

    pub fn find_child(&self, name: impl AsRef<CStr>) -> Option<usize> {
        unsafe {
            self.native()
                .findChild(name.as_ref().as_ptr())
                .try_into()
                .ok()
        }
    }
}

// TODO: wrap SkRuntimeShaderBuilder
