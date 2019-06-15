use crate::prelude::*;
use crate::Data;
use skia_bindings::{C_SkFlattenable_getTypeName, C_SkFlattenable_serialize, SkFlattenable};
use std::ffi::CStr;

// TODO: getFactory()?
// TODO: NameToFactory()?
// TODO: FactoryToName()?
// TODO: Register()?
// TODO: getFlattenableType()?
// TODO: serialize() with SkSerialProcs?

pub trait Flattenable: Sized {
    fn type_name(&self) -> &CStr;
    fn serialize(&self) -> Data;
    fn deserialize(data: &[u8]) -> Option<Self>;
}

// TODO: find a way to hide these trait's functions from other crates.
pub trait NativeFlattenable {
    fn native_flattenable(&self) -> &SkFlattenable;
    fn native_deserialize(data: &[u8]) -> *mut Self;
}

impl<N> Flattenable for RCHandle<N>
where
    N: NativeFlattenable + NativeRefCountedBase,
{
    fn type_name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(C_SkFlattenable_getTypeName(
                self.native().native_flattenable(),
            ))
        }
    }

    fn serialize(&self) -> Data {
        Data::from_ptr(unsafe { C_SkFlattenable_serialize(self.native().native_flattenable()) })
            .unwrap()
    }

    fn deserialize(data: &[u8]) -> Option<Self> {
        RCHandle::from_ptr(N::native_deserialize(data))
    }
}
