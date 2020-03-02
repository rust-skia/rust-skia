use crate::interop::AsStr;
use crate::prelude::*;
use crate::{interop, ColorFilter, Data, Matrix, Shader};
use skia_bindings as sb;
use skia_bindings::{SkRefCntBase, SkRuntimeEffect, SkRuntimeEffect_Variable};
use std::slice;

pub type Variable = Handle<SkRuntimeEffect_Variable>;

impl NativeDrop for SkRuntimeEffect_Variable {
    fn drop(&mut self) {
        panic!("native type SkRuntimeEffect_Variable can't be owned in Rust");
    }
}

impl Handle<SkRuntimeEffect_Variable> {
    pub fn name(&self) -> &str {
        self.native().fName.as_str()
    }

    pub fn offset(&self) -> usize {
        self.native().fOffset
    }

    pub fn qualifier(&self) -> variable::Qualifier {
        self.native().fQualifier
    }

    pub fn ty(&self) -> variable::Type {
        self.native().fType
    }

    pub fn count(&self) -> i32 {
        self.native().fCount
    }

    pub fn flags(&self) -> variable::Flags {
        variable::Flags::from_bits(self.native().fFlags).unwrap()
    }

    #[cfg(feature = "gpu")]
    pub fn gpu_type(&self) -> crate::private::gpu::SLType {
        self.native().fGPUType
    }

    pub fn is_array(&self) -> bool {
        self.flags().contains(variable::Flags::ARRAY)
    }

    pub fn size_in_bytes(&self) -> usize {
        unsafe { self.native().sizeInBytes() }
    }
}

pub mod variable {
    use skia_bindings as sb;

    pub use sb::SkRuntimeEffect_Variable_Qualifier as Qualifier;
    #[test]
    fn test_qualifier_naming() {
        let _ = Qualifier::In;
    }

    pub use sb::SkRuntimeEffect_Variable_Type as Type;
    #[test]
    fn test_type_naming() {
        let _ = Type::Bool;
    }

    bitflags! {
        pub struct Flags : u32 {
            const ARRAY = sb::SkRuntimeEffect_Variable_Flags_kArray_Flag as _;
        }
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

impl RCHandle<SkRuntimeEffect> {
    pub fn make_shader<'a>(
        &mut self,
        inputs: Data,
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
                inputs.into_ptr(),
                children.as_mut_ptr(),
                children.len(),
                local_matrix.into().native_ptr_or_null(),
                is_opaque,
            )
        })
    }

    pub fn make_color_filter(&mut self, inputs: Data) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkRuntimeEffect_makeColorFilter(self.native_mut(), inputs.into_ptr())
        })
    }

    pub fn source(&self) -> &str {
        self.native().fSkSL.as_str()
    }

    pub fn index(&self) -> i32 {
        self.native().fIndex
    }

    pub fn input_size(&self) -> usize {
        unsafe { self.native().inputSize() }
    }

    pub fn uniform_size(&self) -> usize {
        self.native().fUniformSize
    }

    pub fn inputs(&self) -> &[Variable] {
        unsafe {
            let mut count: usize = 0;
            let ptr = sb::C_SkRuntimeEffect_inputs(self.native(), &mut count);
            slice::from_raw_parts(Variable::from_native_ref(&*ptr), count)
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

    // TODO: wrap toPipelineStage()
    // TODO: wrap toByteCode()
}
