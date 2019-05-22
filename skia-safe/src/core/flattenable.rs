use std::ffi::CStr;
use crate::Data;

// TODO: implement trat for the types that derive from it.
pub trait Flattenable {
    fn type_name(&self) -> &CStr;
    fn serialize() -> Data;
    fn deserialize(data: &[u8]) -> Self;
}
