use super::{IntoPtr, NativeAccess, NativeRefCounted, RCHandle};
use std::{
    fmt::{Debug, DebugStruct, Formatter, Result},
    ops::{Deref, DerefMut},
};

pub struct Inherits<N: NativeRefCounted, B> {
    pub(crate) base: B,
    pub(crate) data: RCHandle<N>,
}

impl<N: NativeRefCounted, B> Deref for Inherits<N, B> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<N: NativeRefCounted, B> DerefMut for Inherits<N, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<N: NativeRefCounted, B> NativeAccess for Inherits<N, B> {
    type Native = N;

    fn native(&self) -> &Self::Native {
        self.data.native()
    }

    fn native_mut(&mut self) -> &mut Self::Native {
        self.data.native_mut()
    }
}

impl<N: NativeRefCounted, B> IntoPtr<N> for Inherits<N, B> {
    fn into_ptr(self) -> *mut N {
        self.data.into_ptr()
    }
}

impl<N: NativeRefCounted, B> Debug for Inherits<N, B>
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
