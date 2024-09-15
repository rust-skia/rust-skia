use std::fmt;

use super::{NodeTag, Tagged};
use crate::{prelude::*, Path};
use skia_bindings as sb;

pub type SvgPath = RCHandle<sb::SkSVGPath>;

impl NativeRefCountedBase for sb::SkSVGPath {
    type Base = sb::SkRefCntBase;
}

impl fmt::Debug for SvgPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgPath")
            .field("path", &self.get_path())
            .finish()
    }
}

impl SvgPath {
    skia_macros::attrs! {
        SkSVGPath => {
            path: Path [get(value) => Path::from_native_ref(value), set(value) => value.into_native()]
        }
    }
}

impl Tagged for SvgPath {
    const TAG: NodeTag = NodeTag::Path;
}
