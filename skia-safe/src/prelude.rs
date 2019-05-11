use std::{ptr, mem};
use std::ops::{Index, IndexMut};
use std::hash::{Hasher, Hash};
#[cfg(test)]
use skia_bindings::{SkSurface, SkData, SkColorSpace};
use skia_bindings::{
    SkNVRefCnt,
    SkRefCnt,
    SkRefCntBase,
};

// Re-export TryFrom / TryInto to make them available in all modules that use prelude::*.
pub use std::convert::{TryFrom, TryInto};

/// Swiss army knife to convert any reference into any other.
pub unsafe fn transmute_ref<FromT, ToT>(from: &FromT) -> &ToT {
    // TODO: can we do this statically for all instantiations of transmute_ref?
    debug_assert_eq!(mem::size_of::<FromT>(), mem::size_of::<ToT>());
    &*(from as *const FromT as *const ToT)
}

pub unsafe fn transmute_ref_mut<FromT, ToT>(from: &mut FromT) -> &mut ToT {
    // TODO: can we do this statically for all instantiations of transmute_ref_mut?
    debug_assert_eq!(mem::size_of::<FromT>(), mem::size_of::<ToT>());
    &mut *(from as *mut FromT as *mut ToT)
}

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
    fn ref_cnt(&self) -> usize;
}

impl RefCount for SkRefCntBase {

    // the problem here is that the binding generator represents std::atomic as an u8 (we
    // are lucky that the C alignment rules make space for an i32), so to get the ref
    // counter, we need to get the u8 pointer to fRefCnt and interpret it as an i32 pointer.
    #[allow(clippy::cast_ptr_alignment)]
    fn ref_cnt(&self) -> usize {
        unsafe {
            let ptr: *const i32 =
                &self.fRefCnt as *const _ as *const i32;
            (*ptr).try_into().unwrap()
        }
    }
}
impl RefCount for SkRefCnt {

    fn ref_cnt(&self) -> usize {
        self._base.ref_cnt()
    }
}

impl RefCount for SkNVRefCnt {

    #[allow(clippy::cast_ptr_alignment)]
    fn ref_cnt(&self) -> usize {
        unsafe {
            let ptr: *const i32 =
                &self.fRefCnt as *const _ as *const i32;
            (*ptr).try_into().unwrap()
        }
    }
}

#[cfg(test)]
impl RefCount for SkData {
    fn ref_cnt(&self) -> usize {
        self._base.ref_cnt()
    }
}

#[cfg(test)]
impl RefCount for SkSurface {
    fn ref_cnt(&self) -> usize {
        self._base.ref_cnt()
    }
}

#[cfg(test)]
impl RefCount for SkColorSpace {
    fn ref_cnt(&self) -> usize {
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

    #[allow(clippy::cast_ptr_alignment)]
    fn _ref_cnt(&self) -> usize {
        unsafe {
            let ptr: *const i32 =
                &self.fRefCnt as *const _ as *const i32;

            (*ptr).try_into().unwrap()
        }
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

/// Trait that enables access to a native representation by reference.
pub trait NativeAccess<N> {
    fn native(&self) -> &N;
    fn native_mut(&mut self) -> &mut N;
    unsafe fn native_mut_force(&self) -> &mut N {
        &mut *(self.native() as *const N as *mut N)
    }
}

/// Implements Drop for native types we can not implement Drop for.
pub trait NativeDrop {
    fn drop(&mut self);
}

/// Clone for bindings types we can not implement Clone for.
pub trait NativeClone {
    fn clone(&self) -> Self;
}

/// Even though some types may have value semantics, equality
/// comparison may need to be customized.
pub trait NativePartialEq {
    fn eq(&self, rhs: &Self) -> bool;
}

/// Implements Hash for the native type so that the wrapper type
/// can derive it from.
pub trait NativeHash {
    fn hash<H: Hasher>(&self, state: &mut H);
}

/// A trait allowing a conversion from a native type to a handle type.
pub trait FromNative<N> {
    fn from_native(native: N) -> Self;
}

/// Wraps a native type that can be represented as a value
/// and needs a Drop trait.
#[repr(transparent)]
pub struct Handle<N: NativeDrop>(N);

impl<N: NativeDrop> AsRef<Handle<N>> for Handle<N> {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl<N: NativeDrop> FromNative<N> for Handle<N> {
    fn from_native(n: N) -> Handle<N> {
        Handle(n)
    }
}

impl<N: NativeDrop> Handle<N> {
    /// Constructs a C++ object in place by calling an
    /// extern "C" function that expects a pointer that points to
    /// zeroed memory of the native type.
    pub fn construct_c(construct: unsafe extern "C" fn(*mut N) -> ()) -> Self {
        Self::construct(|instance| unsafe { construct(instance) })
    }

    pub fn construct<F: FnOnce(&mut N) -> ()>(construct: F) -> Self {
        unsafe {
            let mut instance = mem::zeroed();
            construct(&mut instance);
            Self::from_native(instance)
        }
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

impl<N: NativeDrop + NativeClone> Clone for Handle<N> {
    fn clone(&self) -> Self {
        Self::from_native(self.0.clone())
    }
}

impl<N: NativeDrop + NativePartialEq> PartialEq for Handle<N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.native().eq(rhs.native())
    }
}

impl<N: NativeDrop + NativeHash> Hash for Handle<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.native().hash(state);
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

pub trait NativePointerOrNullMut2<N> {
    fn native_ptr_or_null_mut(&mut self) -> *mut N;
}

pub trait NativePointerOrNull2<N> {
    fn native_ptr_or_null(&self) -> *const N;
}

impl<H, N> NativePointerOrNull2<N> for Option<&H>
    where H: NativeTransmutable<N>
{
    fn native_ptr_or_null(&self) -> *const N {
        match self {
            Some(handle) => handle.native(),
            None => ptr::null()
        }
    }
}

impl<H, N> NativePointerOrNullMut2<N> for Option<&mut H>
    where H: NativeTransmutable<N> {
    fn native_ptr_or_null_mut(&mut self) -> *mut N {
        match self {
            Some(handle) => handle.native_mut(),
            None => ptr::null_mut()
        }
    }
}

/// A representation type represented by a refcounted pointer to the native type.
pub struct RCHandle<Native: NativeRefCounted>(*mut Native);

impl<N: NativeRefCounted> AsRef<RCHandle<N>> for RCHandle<N> {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl<N: NativeRefCounted> RCHandle<N> {

    /// Increases the reference counter of the native type
    /// and returns a reference to it.
    #[inline]
    pub fn shared_native(&self) -> &N {
        unsafe {
            let r = &*self.0;
            r._ref();
            r
        }
    }

    /// Increases the reference counter of the native type
    /// and returns a reference to it.
    #[inline]
    pub fn shared_native_mut(&mut self) -> &mut N {
        unsafe {
            let r = &mut *self.0;
            r._ref();
            r
        }
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
    fn native(&self) -> &N {
        unsafe { &*self.0 }
    }

    /// Returns a mutable reference to the native representation.
    fn native_mut(&mut self) -> &mut N {
        unsafe { &mut *self.0 }
    }
}

impl<N: NativeRefCounted> Clone for RCHandle<N> {
    fn clone(&self) -> Self {

        // yes, we _do_ support shared mutability when
        // a ref-counted handle is cloned, so beware of spooky action at
        // a distance.
        RCHandle(self.shared_native() as *const N as _)
    }
}

impl <N: NativeRefCounted> Drop for RCHandle<N> {
    #[inline]
    fn drop(&mut self) {
        unsafe { &*self.0 }._unref();
    }
}

impl<N: NativeRefCounted + NativePartialEq> PartialEq for RCHandle<N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.native().eq(rhs.native())
    }
}

/// A trait for types that can be converted to a shared pointer that may be null.
pub trait ToSharedPointer<N> {
    fn shared_ptr(&self) -> *const N;
}

pub trait ToSharedPointerMut<N> {
    fn shared_ptr_mut(&mut self) -> *mut N;
}

impl<N: NativeRefCounted> ToSharedPointer<N> for Option<RCHandle<N>> {

    fn shared_ptr(&self) -> *const N {
        match self {
            Some(handle) => handle.shared_native(),
            None => ptr::null()
        }
    }
}

impl<N: NativeRefCounted> ToSharedPointerMut<N> for Option<RCHandle<N>> {
    fn shared_ptr_mut(&mut self) -> *mut N {
        match self {
            Some(handle) => handle.shared_native_mut(),
            None => ptr::null_mut()
        }
    }
}

impl<N: NativeRefCounted> ToSharedPointer<N> for Option<&RCHandle<N>> {

    fn shared_ptr(&self) -> *const N {
        match self {
            Some(handle) => handle.shared_native(),
            None => ptr::null()
        }
    }
}

impl<N: NativeRefCounted> ToSharedPointerMut<N> for Option<&mut RCHandle<N>> {
    fn shared_ptr_mut(&mut self) -> *mut N {
        match self {
            Some(handle) => handle.shared_native_mut(),
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

/// Trait to use native types that as a rust type
/// _inplace_ with the same size and field layout.
pub trait NativeTransmutable<NT: Sized> : Sized {

    /// Provides access to the native value through a
    /// transmuted reference to the Rust value.
    fn native(&self) -> &NT {
        unsafe { transmute_ref(self) }
    }

    /// Provides mutable access to the native value through a
    /// transmuted reference to the Rust value.
    fn native_mut(&mut self) -> &mut NT {
        unsafe { transmute_ref_mut(self) }
    }

    /// Copies the native value to an equivalent Rust value.
    fn from_native(nt: NT) -> Self {
        unsafe { mem::transmute_copy::<NT, Self>(&nt) }
    }

    /// Copies the rust type to an equivalent instance of the native type.
    fn into_native(self) -> NT {
        unsafe { mem::transmute_copy::<Self, NT>(&self) }
    }

    /// Provides access to the Rust value through a
    /// transmuted reference to the native value.
    fn from_native_ref(nt: &NT) -> &Self {
        unsafe { transmute_ref(nt) }
    }

    /// Runs a test that proves that the native and the rust
    /// type are of the same size.
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
        unsafe { &*(self as *const [ElementT] as *const [NT]) }
    }

    fn native_mut(&mut self) -> &mut [NT] {
        unsafe { &mut *(self as *mut [ElementT] as *mut [NT]) }
    }
}

impl<NT, RustT> NativeTransmutable<Option<NT>> for Option<RustT>
    where RustT: NativeTransmutable<NT> {}

impl<NT, RustT> NativeTransmutable<Option<&[NT]>> for Option<&[RustT]>
    where RustT: NativeTransmutable<NT> {}

pub trait NativeTransmutableOptionSliceAccessMut<NT: Sized> {
    fn native_mut(&mut self) -> &mut Option<&mut [NT]>;
}

impl<NT, RustT> NativeTransmutableOptionSliceAccessMut<NT> for Option<&mut [RustT]>
    where RustT: NativeTransmutable<NT> {

    fn native_mut(&mut self) -> &mut Option<&mut [NT]> {
        unsafe { transmute_ref_mut(self) }
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

impl<E> AsPointerOrNull<E> for Option<E> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(e) => e,
            None => ptr::null()
        }
    }
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
