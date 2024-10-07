use super::{transmute_ref, NativeRefCounted, RCHandle};
use std::{
    fmt::{Debug, DebugStruct, Formatter, Result},
    ops::Deref,
};

pub trait HasBase {
    type Base: NativeRefCounted;
}

impl<T: NativeRefCounted + HasBase> RCHandle<T> {
    pub(super) fn as_base(&self) -> &RCHandle<T::Base> {
        unsafe { transmute_ref(self) }
    }
}

impl<T: NativeRefCounted + HasBase> Deref for RCHandle<T> {
    type Target = RCHandle<T::Base>;

    fn deref(&self) -> &Self::Target {
        self.as_base()
    }
}

impl<N: NativeRefCounted> Debug for RCHandle<N>
where
    Self: DebugAttributes,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut builder = f.debug_struct(Self::NAME);

        self._dbg(&mut builder);

        builder.finish()
    }
}

pub trait DebugAttributes {
    const NAME: &'static str;

    fn _dbg(&self, builder: &mut DebugStruct);
}
