use std::fmt;

use super::{NodeTag, SvgNode, Tagged};
use crate::{prelude::*, Path};
use skia_bindings as sb;

pub type SvgPath = RCHandle<sb::SkSVGPath>;

impl NativeBase<sb::SkRefCnt> for sb::SkSVGPath {}
impl NativeRefCounted for sb::SkSVGPath {
    fn _ref(&self) {
        self.base()._base._ref();
    }

    fn _unref(&self) {
        self.base()._base._unref();
    }

    fn unique(&self) -> bool {
        self.base()._base.unique()
    }
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
