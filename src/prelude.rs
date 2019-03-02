use std::intrinsics::transmute;
#[cfg(test)]
use rust_skia::{SkSurface, SkData, SkColorSpace};
use rust_skia::{
    SkNVRefCnt,
    SkRefCnt,
    SkRefCntBase,
};
// export all traits for the use of points / vectors, sizes,
// etc. into the prelude.
pub use crate::skia_euclid::{
    SkiaPoint,
    SkiaPointFloat,
    SkiaSize,
    SkiaSizeFloat,
    SkiaRect,
    SkiaRectFloat
};
use std::{ptr, mem};
use std::ops::{Index, IndexMut};

pub trait ToOption {
    type Target;
    fn to_option(self) -> Option<Self::Target>;
}

impl<T> ToOption for *const T {
    type Target = *const T;

    fn to_option(self) -> Option<Self::Target> {
        if !self.is_null() {
            Some(self)
        } else {
            None
        }
    }
}

impl<T> ToOption for *mut T {
    type Target = *mut T;

    fn to_option(self) -> Option<Self::Target> {
        if !self.is_null() {
            Some(self)
        } else {
            None
        }
    }
}

impl ToOption for bool {
    type Target = ();

    fn to_option(self) -> Option<Self::Target> {
        if self { Some(()) } else { None }
    }
}

pub trait IfBoolSome {
    fn if_true_some<V>(self, v: V) -> Option<V>;
    fn if_false_some<V>(self, v: V) -> Option<V>;
}

impl IfBoolSome for bool {
    fn if_true_some<V>(self, v: V) -> Option<V> {
        self.to_option().and(Some(v))
    }

    fn if_false_some<V>(self, v: V) -> Option<V> {
        (!self).if_true_some(v)
    }
}

pub trait RefCount {
    fn ref_cnt(&self) -> i32;
}

impl RefCount for SkRefCntBase {
    // the problem here is that the binding generator represents std::atomic as an u8 (we
    // are lucky that the C alignment rules make space for an i32), so to get the ref
    // counter, we need to get the u8 pointer to fRefCnt and interpret it as an i32 pointer.
    fn ref_cnt(&self) -> i32 {
        let ptr: *const i32 = unsafe { transmute(&self.fRefCnt) };
        unsafe { *ptr }
    }
}
impl RefCount for SkRefCnt {
    fn ref_cnt(&self) -> i32 {
        self._base.ref_cnt()
    }
}

impl RefCount for SkNVRefCnt {
    fn ref_cnt(&self) -> i32 {
        let ptr: *const i32 = unsafe { transmute(&self.fRefCnt) };
        unsafe { *ptr }
    }
}

#[cfg(test)]
impl RefCount for SkData {
    fn ref_cnt(&self) -> i32 {
        self._base.ref_cnt()
    }
}

#[cfg(test)]
impl RefCount for SkSurface {
    fn ref_cnt(&self) -> i32 {
        self._base.ref_cnt()
    }
}

#[cfg(test)]
impl RefCount for SkColorSpace {
    fn ref_cnt(&self) -> i32 {
        self._base.ref_cnt()
    }
}

pub trait NativeRefCounted: Sized {
    fn _ref(&self);
    fn _unref(&self);
    fn _ref_cnt(&self) -> usize {
        unimplemented!();
    }
}

impl NativeRefCounted for SkRefCntBase {
    fn _ref(&self) {
        unsafe { self.ref_() }
    }

    fn _unref(&self) {
        unsafe { self.unref() }
    }

    fn _ref_cnt(&self) -> usize {
        let ptr: *const i32 = unsafe { transmute(&self.fRefCnt) };
        unsafe { *ptr as usize }
    }
}


/// Implements NativeRefCounted by just providing a reference to the base class
/// that implements a RefCount.
pub trait NativeRefCountedBase {
    type Base: NativeRefCounted;
    fn ref_counted_base(&self) -> &Self::Base;
}

impl<Native, Base: NativeRefCounted> NativeRefCounted for Native
    where Native: NativeRefCountedBase<Base=Base> {
    fn _ref(&self) {
        self.ref_counted_base()._ref();
    }

    fn _unref(&self) {
        self.ref_counted_base()._unref();
    }

    fn _ref_cnt(&self) -> usize {
        self.ref_counted_base()._ref_cnt()
    }
}

/// Indicates that the type has a native representation and
/// can convert to and from it. This is for cases in which we
/// can't use the From / Into traits, because we pull in the
/// rust type from another crate.
pub trait NativeRepresentation<NT> {
    fn into_native(self) -> NT;
    fn from_native(native: NT) -> Self;
}

pub trait ToNative<NT> {
    fn to_native(&self) -> NT;
}

impl<NT, RT: Copy + NativeRepresentation<NT>> ToNative<Vec<NT>> for [RT] {
    fn to_native(&self) -> Vec<NT> {
        self.iter().map(|v| v.into_native()).collect()
    }
}

/// Trait that enables access to a native representation by reference.
pub trait NativeAccess<N> {
    fn native(&self) -> &N;
    fn native_mut(&mut self) -> &mut N;
}

/// Trait that enables access to a native representation by value.
pub trait NativeAccessValue<N> {
    fn native(&self) -> N;
}

/// Implements Drop for native types we can not implement Drop for.
pub trait NativeDrop {
    fn drop(&mut self);
}

/// Clone for bindings types we can not implement Clone for.
pub trait NativeClone {
    fn clone(&self) -> Self;
}

impl<N: NativeDrop + NativeClone> Clone for Handle<N> {
    fn clone(&self) -> Self {
        Self::from_native(self.0.clone())
    }
}

/// A trait allowing a conversion from a native type to a handle type.
pub trait FromNative<N> {
    fn from_native(native: N) -> Self;
}

pub trait IntoHandle<H> {
    fn into_handle(self) -> H;
}

impl<H, N> IntoHandle<H> for N
    where H: FromNative<N> {
    fn into_handle(self) -> H {
        H::from_native(self)
    }
}

/// A trait to support conversions from tuples.
pub trait Liftable<S> {
    fn lift_from(source: S) -> Self;
}

pub trait Lift<T> {
    fn lift(self) -> T;
}

impl<V, T> Lift<T> for V
    where T: Liftable<V> {
    fn lift(self) -> T {
        T::lift_from(self)
    }
}

/// A representation type for a native enum type.
#[derive(Copy, Clone, PartialEq)]
pub struct EnumHandle<N: Copy + PartialEq>(pub(crate) N);

impl<N: Copy + PartialEq> FromNative<N> for EnumHandle<N> {
    fn from_native(n: N) -> EnumHandle<N> {
        EnumHandle(n)
    }
}

impl<N: Copy + PartialEq> NativeAccessValue<N> for EnumHandle<N> {
    fn native(&self) -> N {
        self.0
    }
}

/// Even though some types may have value semantics, equality
/// comparison may need to be customized.
pub trait NativePartialEq {
    fn eq(&self, rhs: &Self) -> bool;

    fn ne(&self, rhs: &Self) -> bool {
        !self.eq(rhs)
    }
}

/// A representation type for a native type that has full Copy & Clone value semantics.
#[derive(Copy, Clone)]
pub struct ValueHandle<N: Clone>(N);

impl<N: Clone> FromNative<N> for ValueHandle<N> {
    fn from_native(n: N) -> ValueHandle<N> {
        ValueHandle(n)
    }
}

impl<N: Clone> NativeAccess<N> for ValueHandle<N> {
    fn native(&self) -> &N {
        &self.0
    }

    fn native_mut(&mut self) -> &mut N {
        &mut self.0
    }
}

impl<N: NativePartialEq + Clone> PartialEq for ValueHandle<N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}

/// Wraps a native type that can be represented as a value
/// and needs a destructor.
#[repr(transparent)]
pub struct Handle<N: NativeDrop>(N);

impl<N: NativeDrop> FromNative<N> for Handle<N> {
    fn from_native(n: N) -> Handle<N> {
        Handle(n)
    }
}

impl<N: NativeDrop> Drop for Handle<N> {
    fn drop(&mut self) {
        self.0.drop()
    }
}

impl<N: NativeDrop> NativeAccess<N> for Handle<N> {
    fn native(&self) -> &N {
        &self.0
    }

    fn native_mut(&mut self) -> &mut N {
        &mut self.0
    }
}

impl<N: NativeDrop + NativePartialEq > PartialEq for Handle<N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }

    fn ne(&self, rhs: &Self) -> bool {
        self.0.ne(&rhs.0)
    }
}

pub trait NativeSliceAccess<N: NativeDrop + NativeClone> {
    fn native(&self) -> Vec<N>;
}

impl<N> NativeSliceAccess<N> for [Handle<N>]
    where N: NativeDrop + NativeClone {
    fn native(&self) -> Vec<N> {
        self.iter().map(|v| (v.native().clone())).collect()
    }
}

/// A trait that supports retrieving a pointer from an Option<Handle<Native>>.
/// Returns a null pointer if the Option is None.
pub trait NativePointerOrNull<N> {
    fn native_ptr_or_null(&self) -> *const N;
}

pub trait NativePointerOrNullMut<N> {
    fn native_ptr_or_null_mut(&mut self) -> *mut N;
}

impl<H, N> NativePointerOrNull<N> for Option<&H>
    where H: NativeAccess<N>
{
    fn native_ptr_or_null(&self) -> *const N {
        match self {
            Some(handle) => handle.native(),
            None => ptr::null()
        }
    }
}

impl<H, N> NativePointerOrNullMut<N> for Option<&mut H>
    where H: NativeAccess<N> {
    fn native_ptr_or_null_mut(&mut self) -> *mut N {
        match self {
            Some(handle) => handle.native_mut(),
            None => ptr::null_mut()
        }
    }
}

/// A representation type represented by a refcounted pointer to the native type.
pub struct RCHandle<Native: NativeRefCounted>(*mut Native);

impl<N: NativeRefCounted> RCHandle<N> {
    /// Increases the reference counter of the native type
    /// and returns a mutable reference.
    #[inline]
    pub fn shared_native(&self) -> &mut N {
        (unsafe { &*self.0 })._ref();
        unsafe { &mut *self.0 }
    }

    /// Creates an RCHandle from a pointer.
    /// Returns None if the pointer is null.
    /// Does not increase the reference count.
    #[inline]
    pub fn from_ptr(ptr: *mut N) -> Option<Self> {
        if !ptr.is_null() {
            Some(RCHandle(ptr))
        } else {
            None
        }
    }

    /// Creates an RCHandle from a pointer.
    /// Returns None if the pointer is null.
    /// Increases the reference count.
    pub fn from_unshared_ptr(ptr: *mut N) -> Option<Self> {
        if !ptr.is_null() {
            (unsafe { (*ptr)._ref() });
            Some(RCHandle(ptr))
        } else {
            None
        }
    }
}

impl<N: NativeRefCounted> NativeAccess<N> for RCHandle<N> {
    /// Returns a reference to the native representation.
    #[inline]
    fn native(&self) -> &N {
        unsafe { &*self.0 }
    }

    /// Returns a mutable reference to the native representation.
    #[inline]
    fn native_mut(&mut self) -> &mut N {
        unsafe { &mut *self.0 }
    }
}

impl<N: NativeRefCounted> Clone for RCHandle<N> {
    #[inline]
    fn clone(&self) -> Self {
        RCHandle(self.shared_native())
    }
}

impl <N: NativeRefCounted> Drop for RCHandle<N> {
    #[inline]
    fn drop(&mut self) {
        unsafe { &*self.0 }._unref();
    }
}

/// A trait for types that can be converted to a shared pointer that may be null.
pub trait ToSharedPointer<N> {
    fn shared_ptr(&self) -> *mut N;
}

impl<N: NativeRefCounted> ToSharedPointer<N> for Option<RCHandle<N>> {
    #[inline]
    fn shared_ptr(&self) -> *mut N {
        match self {
            Some(handle) => handle.shared_native(),
            None => ptr::null_mut()
        }
    }
}

impl<N: NativeRefCounted> ToSharedPointer<N> for Option<&RCHandle<N>> {
    #[inline]
    fn shared_ptr(&self) -> *mut N {
        match self {
            Some(handle) => handle.shared_native(),
            None => ptr::null_mut()
        }
    }
}


/// Trait to compute the elements of this type occupy memory in bytes.
pub trait ElementsSizeOf {
    fn elements_size_of(&self) -> usize;
}


impl<N: Sized> ElementsSizeOf for [N] {
    fn elements_size_of(&self) -> usize {
        mem::size_of::<N>() * self.len()
    }
}

/// Tag the type to automatically implement get() functions for
/// all Index implementations.
pub trait IndexGet {}

/// Tag the type to automatically implement get() and set() functions
/// for all Index & IndexMut implementation for that type.
pub trait IndexSet {}

pub trait IndexGetter<I, O : Copy> {
    fn get(&self, index: I) -> O;
}

impl<T, I, O: Copy> IndexGetter<I, O> for T
    where T: Index<I, Output=O> + IndexGet
{
    fn get(&self, index: I) -> O {
        self[index]
    }
}

pub trait IndexSetter<I, O: Copy> {
    fn set(&mut self, index: I, value: O);
}

impl<T, I, O: Copy> IndexSetter<I, O> for T
    where T: IndexMut<I, Output=O> + IndexSet
{
    fn set(&mut self, index: I, value: O) {
        self[index] = value
    }
}

//
// Native types that are represented with a rust type
// _inplace_ with the same size and field layout.
//

pub trait NativeTransmutable<NT: Sized> : Sized {
    fn native(&self) -> &NT {
        unsafe { mem::transmute::<&Self, &NT>(&self) }
    }

    fn native_mut(&mut self) -> &mut NT {
        unsafe { mem::transmute::<&mut Self, &mut NT>(self) }
    }

    // TODO: this seems to actually copy, which is probably not what we want.
    fn from_native(nt: NT) -> Self {
        unsafe { mem::transmute_copy::<NT, Self>(&nt) }
    }

    // TODO: this seems to actually copy, which is probably not what we want.
    fn into_native(self) -> NT {
        unsafe { mem::transmute_copy::<Self, NT>(&self) }
    }

    fn test_layout() {
        assert_eq!(mem::size_of::<Self>(), mem::size_of::<NT>());
    }
}

pub trait NativeTransmutableSliceAccess<NT: Sized> {
    fn native(&self) -> &[NT];
    fn native_mut(&mut self) -> &mut [NT];
}

impl<NT, ElementT> NativeTransmutableSliceAccess<NT> for [ElementT]
    where ElementT: NativeTransmutable<NT> {

    fn native(&self) -> &[NT] {
        unsafe { mem::transmute::<&Self, &[NT]>(self) }
    }

    fn native_mut(&mut self) -> &mut [NT] {
        unsafe { mem::transmute::<&mut Self, &mut [NT]>(self) }
    }
}

//
// Convenience functions to access Option<&[]> as optional ptr (opt_ptr)
// that may be null.
//

pub trait AsPointerOrNull<PointerT> {
    fn as_ptr_or_null(&self) -> *const PointerT;
}

pub trait AsPointerOrNullMut<PointerT> {
    fn as_ptr_or_null(&self) -> *const PointerT;
    fn as_ptr_or_null_mut(&mut self) -> *mut PointerT;
}

impl<E> AsPointerOrNull<E> for Option<&[E]> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(slice) => slice.as_ptr(),
            None => ptr::null()
        }
    }
}

impl<E> AsPointerOrNullMut<E> for Option<&mut [E]> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(slice) => slice.as_ptr(),
            None => ptr::null()
        }
    }

    fn as_ptr_or_null_mut(&mut self) -> *mut E {
        match self {
            Some(slice) => slice.as_mut_ptr(),
            None => ptr::null_mut()
        }
    }
}

impl<E> AsPointerOrNull<E> for Option<&Vec<E>> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(v) => v.as_ptr(),
            None => ptr::null()
        }
    }
}

impl<E> AsPointerOrNullMut<E> for Option<Vec<E>> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(v) => v.as_ptr(),
            None => ptr::null()
        }
    }

    fn as_ptr_or_null_mut(&mut self) -> *mut E {
        match self {
            Some(v) => v.as_mut_ptr(),
            None => ptr::null_mut()
        }
    }
}

//
// Safe Conversions from to until try_from / try_into is stabalized.
//

pub trait TryFrom<FromT> : Sized {
    fn try_from(from: FromT) -> Option<Self>;
}

pub trait TryInto<IntoT> : Sized {
    fn try_into(self) -> Option<IntoT>;
}

impl<IntoT, T> TryInto<IntoT> for T
    where IntoT: TryFrom<T> {

    fn try_into(self) -> Option<IntoT> {
        IntoT::try_from(self)
    }
}

impl TryFrom<usize> for i32 {
    fn try_from(from: usize) -> Option<Self> {
        if from <= i32::max_value() as usize {
            Some(from as i32)
        } else {
            None
        }
    }
}

impl TryFrom<i32> for usize {
    fn try_from(from: i32) -> Option<Self> {
        if from >= 0 {
            Some(from as usize)
        } else {
            None
        }
    }
}