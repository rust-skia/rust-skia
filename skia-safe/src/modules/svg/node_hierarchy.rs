use crate::prelude::*;
use std::{
    fmt::{Debug, DebugStruct, Formatter, Result},
    ops::{Deref, DerefMut},
};

pub trait NodeSubtype {
    type Base: NativeRefCounted;
}

/// All [`NodeSubType`] are defined to be a [`RcHandle<N>`] with `N` deriving from
/// [`sb::SkRefCntBase`]
impl<T: NodeSubtype> NativeRefCountedBase for T {
    type Base = skia_bindings::SkRefCntBase;
}

impl<T: NativeRefCounted + NodeSubtype> RCHandle<T> {
    pub(super) fn as_base(&self) -> &RCHandle<T::Base> {
        unsafe { transmute_ref(self) }
    }
}

impl<T: NativeRefCounted + NodeSubtype> Deref for RCHandle<T> {
    type Target = RCHandle<T::Base>;

    fn deref(&self) -> &Self::Target {
        self.as_base()
    }
}

/// This implementation of [`DerefMut`] causes subsequent UB when the containing [`RcHandle`] gets
/// overwritten with a base type that does not match the actual underlying type.
impl<T: NativeRefCounted + NodeSubtype> DerefMut for RCHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_ref_mut(self) }
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
