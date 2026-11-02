#![allow(dead_code)]
use std::{
    cell::UnsafeCell,
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr, slice,
};

use skia_bindings::{
    C_SkRefCntBase_ref, C_SkRefCntBase_unique, C_SkRefCntBase_unref, SkRefCnt, SkRefCntBase, sk_sp,
};

/// Convert any reference into any other.
pub(crate) unsafe fn transmute_ref<FromT, ToT>(from: &FromT) -> &ToT {
    unsafe {
        assert_layout_compatible::<FromT, ToT>();
        &*(from as *const FromT as *const ToT)
    }
}

pub(crate) unsafe fn transmute_ref_mut<FromT, ToT>(from: &mut FromT) -> &mut ToT {
    unsafe {
        assert_layout_compatible::<FromT, ToT>();
        &mut *(from as *mut FromT as *mut ToT)
    }
}

pub(crate) trait IntoNonNull {
    type Target;
    fn into_non_null(self) -> Option<ptr::NonNull<Self::Target>>;
}

impl<T> IntoNonNull for *const T {
    type Target = T;

    fn into_non_null(self) -> Option<ptr::NonNull<Self::Target>> {
        ptr::NonNull::new(self as *mut T)
    }
}

impl<T> IntoNonNull for *mut T {
    type Target = T;

    fn into_non_null(self) -> Option<ptr::NonNull<Self::Target>> {
        ptr::NonNull::new(self)
    }
}

#[cfg(test)]
pub(crate) trait RefCount {
    fn ref_cnt(&self) -> usize;
}

#[cfg(test)]
impl RefCount for SkRefCntBase {
    // the problem here is that the binding generator represents std::atomic as an u8 (we
    // are lucky that the C alignment rules make space for an i32), so to get the ref
    // counter, we need to get the u8 pointer to fRefCnt and interpret it as an i32 pointer.
    #[allow(clippy::cast_ptr_alignment)]
    fn ref_cnt(&self) -> usize {
        unsafe {
            let ptr: *const i32 = &self.fRefCnt as *const _ as *const i32;
            (*ptr).try_into().unwrap()
        }
    }
}

impl NativeBase<SkRefCntBase> for SkRefCnt {}

#[cfg(test)]
impl RefCount for SkRefCnt {
    fn ref_cnt(&self) -> usize {
        self.base().ref_cnt()
    }
}

#[cfg(test)]
impl RefCount for skia_bindings::SkNVRefCnt {
    #[allow(clippy::cast_ptr_alignment)]
    fn ref_cnt(&self) -> usize {
        unsafe {
            let ptr: *const i32 = &self.fRefCnt as *const _ as *const i32;
            (*ptr).try_into().unwrap()
        }
    }
}

pub trait NativeRefCounted: Sized {
    /// Increases the reference count.
    ///
    /// This function takes a raw pointer instead of `&self`. C++ writes the count through a
    /// `const` pointer, and Rust forbids writes to memory behind a shared reference.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a live object.
    unsafe fn _ref(ptr: *const Self);
    /// Decreases the reference count and deletes the object when the count reaches zero.
    ///
    /// This function takes a raw pointer instead of `&self`. C++ may delete the object during
    /// the call, and Rust forbids freeing memory while a reference to it is still in use.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a live object that holds a reference the caller owns. The caller gives
    /// up that reference and must not use `ptr` afterwards.
    unsafe fn _unref(ptr: *const Self);
    /// Returns `true` if the caller holds the only reference.
    ///
    /// This function takes a raw pointer instead of `&self`, like [`Self::_ref`]. Other threads
    /// may change the count during the call, and a shared reference would assert that the
    /// object's memory does not change.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a live object.
    unsafe fn unique(ptr: *const Self) -> bool;
    fn _ref_cnt(&self) -> usize {
        unimplemented!();
    }
}

impl NativeRefCounted for SkRefCntBase {
    unsafe fn _ref(ptr: *const Self) {
        unsafe { C_SkRefCntBase_ref(ptr) }
    }

    unsafe fn _unref(ptr: *const Self) {
        unsafe { C_SkRefCntBase_unref(ptr) }
    }

    unsafe fn unique(ptr: *const Self) -> bool {
        unsafe { C_SkRefCntBase_unique(ptr) }
    }

    #[allow(clippy::cast_ptr_alignment)]
    fn _ref_cnt(&self) -> usize {
        unsafe {
            let ptr: *const i32 = &self.fRefCnt as *const _ as *const i32;
            (*ptr).try_into().unwrap()
        }
    }
}

/// Implements NativeRefCounted by just providing a reference to the base class
/// that implements a RefCount.
/// TODO: use NativeBase
pub trait NativeRefCountedBase {
    type Base: NativeRefCounted;

    /// Returns a pointer to the ref counter base class of the ref counted type.
    ///
    /// The default implementation assumes that the base class is at the same address as the ref
    /// counted type. It casts the pointer without creating a reference, so the result keeps the
    /// permissions of `ptr`. [`NativeRefCounted::_unref`] relies on that, because C++ writes
    /// through the base pointer when it deletes the object.
    fn ref_counted_base_ptr(ptr: *const Self) -> *const Self::Base
    where
        Self: Sized,
    {
        ptr.cast()
    }

    /// Returns the ref counter base class of the ref counted type.
    fn ref_counted_base(&self) -> &Self::Base
    where
        Self: Sized,
    {
        unsafe { &*Self::ref_counted_base_ptr(self) }
    }
}

impl<Native, Base: NativeRefCounted> NativeRefCounted for Native
where
    Native: NativeRefCountedBase<Base = Base>,
{
    unsafe fn _ref(ptr: *const Self) {
        unsafe { Base::_ref(Self::ref_counted_base_ptr(ptr)) }
    }

    unsafe fn _unref(ptr: *const Self) {
        unsafe { Base::_unref(Self::ref_counted_base_ptr(ptr)) }
    }

    unsafe fn unique(ptr: *const Self) -> bool {
        unsafe { Base::unique(Self::ref_counted_base_ptr(ptr)) }
    }

    fn _ref_cnt(&self) -> usize {
        self.ref_counted_base()._ref_cnt()
    }
}

/// Trait that enables access to a native representation of a wrapper type.
pub trait NativeAccess {
    type Native;
    /// Provides shared access to the native type of the wrapper.
    fn native(&self) -> &Self::Native;

    /// Returns a pointer for C++ calls that may write.
    ///
    /// The default derives the pointer from [`Self::native`], which permits only reads.
    /// [`RefHandle`] and [`RCHandle`] override it to return their own pointer.
    unsafe fn native_mut_force(&self) -> *mut Self::Native {
        self.native() as *const Self::Native as *mut Self::Native
    }
}

/// Trait that enables exclusive access to a native representation of a wrapper type.
///
/// [`RefHandle`] does not implement this trait. Its native objects live on the C++ heap and may
/// store pointers into their own memory, so calls that modify them go through
/// [`RefHandle::native_ptr`] instead.
pub trait NativeAccessMut: NativeAccess {
    /// Provides exclusive access to the native type of the wrapper.
    fn native_mut(&mut self) -> &mut Self::Native;
}

/// Implements Drop for native types we can not implement Drop for.
pub trait NativeDrop {
    fn drop(&mut self);
}

/// Deletes a native object that C++ allocated on the heap and that a [`RefHandle`] owns.
///
/// This trait takes a raw pointer instead of `&mut self`. The object is freed during the call,
/// and Rust forbids freeing memory while a reference to it is still in use.
pub trait NativeDelete {
    /// Deletes the object that `ptr` points to.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a live object that the caller owns. The caller must not use `ptr`
    /// afterwards.
    unsafe fn delete(ptr: *mut Self);
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

/// Wraps a native type that can be represented in Rust memory.
///
/// This type requires an implementation of the `NativeDrop` trait.
#[repr(transparent)]
pub struct Handle<N: NativeDrop>(
    // UnsafeCell is used for disabling niche optimization, because we don't care about the proper
    // representations of bindgen types as long the size and alignment matches.
    UnsafeCell<N>,
    // Keeps Handle !Send even when N: Send. UnsafeCell already makes it !Sync.
    PhantomData<*const ()>,
);

impl<N: NativeDrop> AsRef<Handle<N>> for Handle<N> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<N: NativeDrop> Handle<N> {
    /// Wrap a native instance into a handle.
    #[must_use]
    pub(crate) fn from_native_c(n: N) -> Self {
        Handle(n.into(), PhantomData)
    }

    /// Create a reference to the Rust wrapper from a reference to the native type.
    #[must_use]
    pub(crate) fn from_native_ref(n: &N) -> &Self {
        unsafe { transmute_ref(n) }
    }

    /// Create a mutable reference to the Rust wrapper from a reference to the native type.
    #[must_use]
    pub(crate) fn from_native_ref_mut(n: &mut N) -> &mut Self {
        unsafe { transmute_ref_mut(n) }
    }

    /// Converts a pointer to a native value into a pointer to the Rust value.
    #[must_use]
    pub(crate) fn from_native_ptr(np: *const N) -> *const Self {
        np as _
    }

    /// Converts a pointer to a mutable native value into a pointer to the mutable Rust value.
    #[allow(unused)]
    #[must_use]
    pub(crate) fn from_native_ptr_mut(np: *mut N) -> *mut Self {
        np as _
    }

    /// Constructs a C++ object in place by calling a
    /// function that expects a pointer that points to
    /// uninitialized memory of the native type.
    #[must_use]
    pub(crate) fn construct(construct: impl FnOnce(*mut N)) -> Self {
        Self::try_construct(|i| {
            construct(i);
            true
        })
        .unwrap()
    }

    pub(crate) fn try_construct(construct: impl FnOnce(*mut N) -> bool) -> Option<Self> {
        self::try_construct(construct).map(|n| Self::from_native_c(n.into_inner()))
    }

    /// Replaces the native instance with the one from this Handle, and returns the replaced one
    /// wrapped in a Rust Handle without dropping either one.
    #[must_use]
    pub(crate) fn replace_native(mut self, native: &mut N) -> Self {
        mem::swap(self.0.get_mut(), native);
        self
    }

    /// Consumes the wrapper and returns the native type.
    #[must_use]
    pub(crate) fn into_native(mut self) -> N {
        let r = mem::replace(&mut self.0, unsafe { mem::zeroed() });
        mem::forget(self);
        r.into_inner()
    }
}

pub(crate) trait ReplaceWith<Other> {
    fn replace_with(&mut self, other: Other) -> Other;
}

impl<N: NativeDrop> ReplaceWith<Handle<N>> for N {
    fn replace_with(&mut self, other: Handle<N>) -> Handle<N> {
        other.replace_native(self)
    }
}

/// Constructs a C++ object in place by calling a lambda that is meant to initialize
/// the pointer to the Rust memory provided as a pointer.
#[must_use]
pub(crate) fn construct<N>(construct: impl FnOnce(*mut N)) -> N {
    try_construct(|i| {
        construct(i);
        true
    })
    .unwrap()
    .into_inner()
}

#[must_use]
pub(crate) fn try_construct<N>(construct: impl FnOnce(*mut N) -> bool) -> Option<UnsafeCell<N>> {
    let mut instance = mem::MaybeUninit::uninit();
    construct(instance.as_mut_ptr()).then(|| unsafe { instance.assume_init() }.into())
}

impl<N: NativeDrop> Drop for Handle<N> {
    fn drop(&mut self) {
        self.0.get_mut().drop()
    }
}

impl<N: NativeDrop> NativeAccess for Handle<N> {
    type Native = N;

    fn native(&self) -> &N {
        unsafe { &*(self.0.get()) }
    }
}

impl<N: NativeDrop> NativeAccessMut for Handle<N> {
    fn native_mut(&mut self) -> &mut N {
        self.0.get_mut()
    }
}

impl<N: NativeDrop + NativeClone> Clone for Handle<N> {
    fn clone(&self) -> Self {
        Self::from_native_c(self.native().clone())
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

pub(crate) trait NativeSliceAccess<N: NativeDrop> {
    fn native(&self) -> &[N];
    fn native_mut(&mut self) -> &mut [N];
}

impl<N: NativeDrop> NativeSliceAccess<N> for [Handle<N>] {
    fn native(&self) -> &[N] {
        let ptr = self.first().native_ptr_or_null();
        unsafe { slice::from_raw_parts(ptr, self.len()) }
    }

    fn native_mut(&mut self) -> &mut [N] {
        let ptr = self.first_mut().native_ptr_or_null_mut();
        unsafe { slice::from_raw_parts_mut(ptr, self.len()) }
    }
}

/// A trait that supports retrieving a pointer from an Option<Handle<Native>>.
/// Returns a null pointer if the Option is None.
pub(crate) trait NativePointerOrNull {
    type Native;

    fn native_ptr_or_null(&self) -> *const Self::Native;
    unsafe fn native_ptr_or_null_mut_force(&self) -> *mut Self::Native;
}

pub(crate) trait NativePointerOrNullMut {
    type Native;

    fn native_ptr_or_null_mut(&mut self) -> *mut Self::Native;
}

impl<H, N> NativePointerOrNull for Option<&H>
where
    H: NativeAccess<Native = N>,
{
    type Native = N;

    fn native_ptr_or_null(&self) -> *const N {
        match self {
            Some(handle) => handle.native(),
            None => ptr::null(),
        }
    }

    unsafe fn native_ptr_or_null_mut_force(&self) -> *mut N {
        unsafe {
            match self {
                Some(handle) => handle.native_mut_force(),
                None => ptr::null_mut(),
            }
        }
    }
}

impl<H, N> NativePointerOrNullMut for Option<&mut H>
where
    H: NativeAccessMut<Native = N>,
{
    type Native = N;

    fn native_ptr_or_null_mut(&mut self) -> *mut N {
        match self {
            Some(handle) => handle.native_mut(),
            None => ptr::null_mut(),
        }
    }
}

pub(crate) trait NativePointerOrNullMut2<N> {
    fn native_ptr_or_null_mut(&mut self) -> *mut N;
}

pub(crate) trait NativePointerOrNull2<N> {
    fn native_ptr_or_null(&self) -> *const N;
}

impl<H, N> NativePointerOrNull2<N> for Option<&H>
where
    H: NativeTransmutable<N>,
{
    fn native_ptr_or_null(&self) -> *const N {
        match self {
            Some(handle) => handle.native(),
            None => ptr::null(),
        }
    }
}

impl<H, N> NativePointerOrNullMut2<N> for Option<&mut H>
where
    H: NativeTransmutable<N>,
{
    fn native_ptr_or_null_mut(&mut self) -> *mut N {
        match self {
            Some(handle) => handle.native_mut(),
            None => ptr::null_mut(),
        }
    }
}

/// A wrapper type that represents a native type with a pointer to
/// the native object.
#[repr(transparent)]
pub struct RefHandle<N: NativeDelete>(ptr::NonNull<N>);

impl<N: NativeDelete> Drop for RefHandle<N> {
    fn drop(&mut self) {
        unsafe { N::delete(self.0.as_ptr()) }
    }
}

impl<N: NativeDelete + NativePartialEq> PartialEq for RefHandle<N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.native().eq(rhs.native())
    }
}

impl<N: NativeDelete> NativeAccess for RefHandle<N> {
    type Native = N;

    fn native(&self) -> &N {
        unsafe { self.0.as_ref() }
    }

    /// Returns the handle's own pointer. A pointer derived from [`Self::native`] would only
    /// permit reads.
    unsafe fn native_mut_force(&self) -> *mut N {
        self.0.as_ptr()
    }
}

impl<N: NativeDelete> RefHandle<N> {
    /// Creates a RefHandle from a native pointer.
    ///
    /// From this time on, the handle owns the object that the pointer points
    /// to and will call its NativeDrop implementation if it goes out of scope.
    pub(crate) fn from_ptr(ptr: *mut N) -> Option<Self> {
        ptr::NonNull::new(ptr).map(Self)
    }

    pub(crate) fn into_ptr(self) -> *mut N {
        let p = self.0.as_ptr();
        mem::forget(self);
        p
    }

    /// Returns a raw pointer to the native object for calls that may modify it.
    ///
    /// Some native types store pointers into their own memory, and C++ writes through them. A
    /// `&mut N` would promise Rust that nothing else writes to the object while it exists, so
    /// calls into such types use this pointer with the raw `sb::<Type>_*` functions instead of the
    /// bindgen methods that take `&mut self`.
    pub(crate) fn native_ptr(&mut self) -> *mut N {
        self.0.as_ptr()
    }
}

/// A wrapper type represented by a reference counted pointer
/// to the native type.
#[repr(transparent)]
pub struct RCHandle<Native: NativeRefCounted>(ptr::NonNull<Native>);

/// A reference counted handle is cheap to clone, so we do support a conversion
/// from a reference to a ref counter to an owned handle.
impl<N: NativeRefCounted> From<&RCHandle<N>> for RCHandle<N> {
    fn from(rch: &RCHandle<N>) -> Self {
        rch.clone()
    }
}

impl<N: NativeRefCounted> AsRef<RCHandle<N>> for RCHandle<N> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<N: NativeRefCounted> RCHandle<N> {
    /// Creates an reference counted handle from a native pointer.
    ///
    /// Takes ownership of the object the pointer points to, does not increase the reference count.
    ///
    /// Returns `None` if the pointer is `null`.
    #[inline]
    pub(crate) fn from_ptr(ptr: *mut N) -> Option<Self> {
        ptr::NonNull::new(ptr).map(Self)
    }

    #[inline]
    pub(crate) fn from_ptr_const(ptr: *const N) -> Option<Self> {
        ptr::NonNull::new(ptr as *mut N).map(Self)
    }

    /// Creates an reference counted handle from a pointer.
    ///
    /// Returns `None` if the pointer is `null`.
    ///
    /// Shares ownership with the object referenced to by the pointer, therefore increases the
    /// reference count.
    #[inline]
    pub(crate) fn from_unshared_ptr(ptr: *mut N) -> Option<Self> {
        ptr::NonNull::new(ptr).map(|ptr| {
            unsafe { N::_ref(ptr.as_ptr()) };
            Self(ptr)
        })
    }

    /// Create a reference to the Rust wrapper from a reference to a pointer that points
    /// to the native type.
    pub(crate) fn from_unshared_ptr_ref(n: &*mut N) -> &Option<Self> {
        unsafe { transmute_ref(n) }
    }

    /// Create a reference to a all non-null sk_sp<N> slice.
    pub(crate) fn from_non_null_sp_slice(sp_slice: &[sk_sp<N>]) -> &[Self] {
        assert_layout_compatible::<sk_sp<N>, Self>();
        assert!(sp_slice.iter().all(|v| !v.fPtr.is_null()));
        unsafe { mem::transmute(sp_slice) }
    }

    /// Returns the pointer to the handle.
    #[allow(unused)]
    pub(crate) fn as_ptr(&self) -> &ptr::NonNull<N> {
        &self.0
    }

    /// Returns a raw pointer to the native object for calls that may modify it.
    ///
    /// Some native types store pointers into their own memory, and C++ writes through them. See
    /// [`RefHandle::native_ptr`] for why such calls must not go through a `&mut N`.
    pub(crate) fn native_ptr(&mut self) -> *mut N {
        self.0.as_ptr()
    }
}

#[cfg(test)]
mod rc_handle_tests {
    use std::ptr;

    use skia_bindings::{SkFontMgr, SkTypeface};

    use crate::{FontMgr, Typeface, prelude::NativeAccessMut};

    #[test]
    fn rc_native_ref_null() {
        let f: *mut SkTypeface = ptr::null_mut();
        let r = Typeface::from_unshared_ptr(f);
        assert!(r.is_none())
    }

    #[test]
    fn rc_native_ref_non_null() {
        let mut font_mgr = FontMgr::new();
        let f: *mut SkFontMgr = font_mgr.native_mut();
        let r = FontMgr::from_unshared_ptr(f);
        assert!(r.is_some())
    }
}

impl<N: NativeRefCounted> NativeAccess for RCHandle<N> {
    type Native = N;

    /// Returns a reference to the native representation.
    fn native(&self) -> &N {
        unsafe { self.0.as_ref() }
    }

    /// Returns the handle's own pointer. A pointer derived from [`Self::native`] would only
    /// permit reads.
    unsafe fn native_mut_force(&self) -> *mut N {
        self.0.as_ptr()
    }
}

impl<N: NativeRefCounted> NativeAccessMut for RCHandle<N> {
    /// Returns a mutable reference to the native representation.
    fn native_mut(&mut self) -> &mut N {
        unsafe { self.0.as_mut() }
    }
}

impl<N: NativeRefCounted> Clone for RCHandle<N> {
    fn clone(&self) -> Self {
        // Support shared mutability when a ref-counted handle is cloned.
        let ptr = self.0;
        unsafe { N::_ref(ptr.as_ptr()) };
        Self(ptr)
    }
}

impl<N: NativeRefCounted> Drop for RCHandle<N> {
    #[inline]
    fn drop(&mut self) {
        unsafe { N::_unref(self.0.as_ptr()) };
    }
}

impl<N: NativeRefCounted + NativePartialEq> PartialEq for RCHandle<N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.native().eq(rhs.native())
    }
}

/// A trait that consumes self and converts it to a ptr to the native type.
pub(crate) trait IntoPtr<N> {
    fn into_ptr(self) -> *mut N;
}

impl<N: NativeRefCounted> IntoPtr<N> for RCHandle<N> {
    fn into_ptr(self) -> *mut N {
        let ptr = self.0.as_ptr();
        mem::forget(self);
        ptr
    }
}

/// A trait that consumes self and converts it to a ptr to the native type or null.
pub(crate) trait IntoPtrOrNull {
    type Native;
    fn into_ptr_or_null(self) -> *mut Self::Native;
}

impl<N: NativeRefCounted> IntoPtrOrNull for Option<RCHandle<N>> {
    type Native = N;
    fn into_ptr_or_null(self) -> *mut N {
        self.map(|rc| rc.into_ptr()).unwrap_or(ptr::null_mut())
    }
}

/// Tag the type to automatically implement get() functions for
/// all Index implementations.
pub trait IndexGet {}

/// Tag the type to automatically implement get() and set() functions
/// for all Index & IndexMut implementation for that type.
pub trait IndexSet {}

pub trait IndexGetter<I, O: Copy> {
    // TODO: Not sure why clippy 1.78-beta.1 complains about this one.
    #[allow(unused)]
    fn get(&self, index: I) -> O;
}

impl<T, I, O: Copy> IndexGetter<I, O> for T
where
    T: Index<I, Output = O> + IndexGet,
{
    fn get(&self, index: I) -> O {
        self[index]
    }
}

pub trait IndexSetter<I, O: Copy> {
    fn set(&mut self, index: I, value: O) -> &mut Self;
}

impl<T, I, O: Copy> IndexSetter<I, O> for T
where
    T: IndexMut<I, Output = O> + IndexSet,
{
    fn set(&mut self, index: I, value: O) -> &mut Self {
        self[index] = value;
        self
    }
}

/// Trait to mark a native type that can be treated a Rust type _inplace_ with the same size and
/// field layout.
pub trait NativeTransmutable<NT: Sized>: Sized
where
    Self: Sized,
{
    /// Provides access to the native value through a
    /// transmuted reference to the Rust value.
    fn native(&self) -> &NT {
        unsafe { transmute_ref(self) }
    }

    /// Provides mutable access to the native value through a transmuted reference to the Rust
    /// value.
    fn native_mut(&mut self) -> &mut NT {
        unsafe { transmute_ref_mut(self) }
    }

    /// Copies the native value to an equivalent Rust value.
    ///
    /// The `_c` suffix is to remind callers that values that requires C++ ABI features can't be
    /// used here.
    fn from_native_c(nt: NT) -> Self {
        let r = unsafe { mem::transmute_copy::<NT, Self>(&nt) };
        // don't drop, the Rust type takes over.
        mem::forget(nt);
        r
    }

    /// Copies the rust type to an equivalent instance of the native type.
    fn into_native(self) -> NT {
        let r = unsafe { mem::transmute_copy::<Self, NT>(&self) };
        // don't drop, the native type takes over.
        mem::forget(self);
        r
    }

    /// Returns a reference to the Rust value by transmuting a reference to the native value.
    fn from_native_ref(nt: &NT) -> &Self {
        unsafe { transmute_ref(nt) }
    }

    /// Returns a reference to the Rust array reference by transmuting a reference to the native
    /// array.
    fn from_native_array_ref<const N: usize>(nt: &[NT; N]) -> &[Self; N] {
        unsafe { transmute_ref(nt) }
    }

    /// Returns a reference to the Rust value through a transmuted reference to the native mutable
    /// value.
    fn from_native_ref_mut(nt: &mut NT) -> &mut Self {
        unsafe { transmute_ref_mut(nt) }
    }

    /// Converts a pointer to a native value into a pointer to the Rust value.
    fn from_native_ptr(np: *const NT) -> *const Self {
        np as _
    }

    /// Converts a pointer to a mutable native value into a pointer to the mutable Rust value.
    fn from_native_ptr_mut(np: *mut NT) -> *mut Self {
        np as _
    }

    fn construct(construct: impl FnOnce(*mut NT)) -> Self {
        Self::try_construct(|i| {
            construct(i);
            true
        })
        .unwrap()
    }

    fn try_construct(construct: impl FnOnce(*mut NT) -> bool) -> Option<Self> {
        self::try_construct(construct).map(|n| Self::from_native_c(n.into_inner()))
    }
}

/// Tests if T1 and T2 do have the same size and alignment.
pub(crate) const fn assert_layout_compatible<T1, T2>() {
    assert!(
        mem::size_of::<T1>() == mem::size_of::<T2>(),
        "Type layout verification failed: Size mismatch"
    );
    assert!(
        mem::align_of::<T1>() == mem::align_of::<T2>(),
        "Type layout verification failed: Alignment mismatch"
    );
}

pub(crate) trait NativeTransmutableSliceAccess<NT: Sized> {
    fn native(&self) -> &[NT];
    fn native_mut(&mut self) -> &mut [NT];
}

impl<NT, ElementT> NativeTransmutableSliceAccess<NT> for [ElementT]
where
    ElementT: NativeTransmutable<NT>,
{
    fn native(&self) -> &[NT] {
        unsafe { &*(self as *const [ElementT] as *const [NT]) }
    }

    fn native_mut(&mut self) -> &mut [NT] {
        unsafe { &mut *(self as *mut [ElementT] as *mut [NT]) }
    }
}

impl<NT, RustT> NativeTransmutable<Option<NT>> for Option<RustT> where RustT: NativeTransmutable<NT> {}

impl<NT, RustT> NativeTransmutable<Option<&[NT]>> for Option<&[RustT]> where
    RustT: NativeTransmutable<NT>
{
}

pub(crate) trait NativeTransmutableOptionSliceAccessMut<NT: Sized> {
    fn native_mut(&mut self) -> &mut Option<&mut [NT]>;
}

impl<NT, RustT> NativeTransmutableOptionSliceAccessMut<NT> for Option<&mut [RustT]>
where
    RustT: NativeTransmutable<NT>,
{
    fn native_mut(&mut self) -> &mut Option<&mut [NT]> {
        unsafe { transmute_ref_mut(self) }
    }
}

//
// Convenience functions to access Option<&[]> as optional ptr (opt_ptr)
// that may be null.
//

pub(crate) trait AsPointerOrNull<PointerT> {
    fn as_ptr_or_null(&self) -> *const PointerT;
}

pub(crate) trait AsPointerOrNullMut<PointerT>: AsPointerOrNull<PointerT> {
    fn as_ptr_or_null_mut(&mut self) -> *mut PointerT;
}

impl<E> AsPointerOrNull<E> for Option<E> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(e) => e,
            None => ptr::null(),
        }
    }
}

impl<E> AsPointerOrNullMut<E> for Option<E> {
    fn as_ptr_or_null_mut(&mut self) -> *mut E {
        match self {
            Some(e) => e,
            None => ptr::null_mut(),
        }
    }
}

impl<E> AsPointerOrNull<E> for Option<&[E]> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(slice) => slice.as_ptr(),
            None => ptr::null(),
        }
    }
}

impl<E> AsPointerOrNull<E> for Option<&mut [E]> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(slice) => slice.as_ptr(),
            None => ptr::null(),
        }
    }
}

impl<E> AsPointerOrNullMut<E> for Option<&mut [E]> {
    fn as_ptr_or_null_mut(&mut self) -> *mut E {
        match self {
            Some(slice) => slice.as_mut_ptr(),
            None => ptr::null_mut(),
        }
    }
}

impl<E> AsPointerOrNull<E> for Option<&Vec<E>> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(v) => v.as_ptr(),
            None => ptr::null(),
        }
    }
}

impl<E> AsPointerOrNull<E> for Option<Vec<E>> {
    fn as_ptr_or_null(&self) -> *const E {
        match self {
            Some(v) => v.as_ptr(),
            None => ptr::null(),
        }
    }
}

impl<E> AsPointerOrNullMut<E> for Option<Vec<E>> {
    fn as_ptr_or_null_mut(&mut self) -> *mut E {
        match self {
            Some(v) => v.as_mut_ptr(),
            None => ptr::null_mut(),
        }
    }
}

// Wraps a handle so that the Rust's borrow checker assumes it represents
// something that borrows something else.
#[repr(transparent)]
pub struct Borrows<'a, H>(H, PhantomData<&'a ()>);

impl<H> Deref for Borrows<'_, H> {
    type Target = H;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: this is most likely unsafe because someone could replace the
// value the reference is pointing to.
impl<H> DerefMut for Borrows<'_, H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<H> Borrows<'_, H> {
    /// Notify that the borrowed dependency is not referred to anymore and return the handle.
    /// # Safety
    /// The borrowed dependency must be removed before calling `release()`.
    pub unsafe fn release(self) -> H {
        self.0
    }
}

pub(crate) trait BorrowsFrom: Sized {
    fn borrows<D: ?Sized>(self, _dep: &D) -> Borrows<Self>;
}

impl<T: Sized> BorrowsFrom for T {
    fn borrows<D: ?Sized>(self, _dep: &D) -> Borrows<Self> {
        Borrows(self, PhantomData)
    }
}

impl<H> Borrows<'_, H> {
    pub(crate) unsafe fn unchecked_new(h: H) -> Self {
        Self(h, PhantomData)
    }
}

/// Declares a base class for a native type.
pub trait NativeBase<Base> {
    fn base(&self) -> &Base {
        unsafe { &*(self as *const Self as *const Base) }
    }

    fn base_mut(&mut self) -> &mut Base {
        unsafe { &mut *(self as *mut Self as *mut Base) }
    }
}

pub struct Sendable<H: ConditionallySend>(H);
unsafe impl<H: ConditionallySend> Send for Sendable<H> {}

impl<H: ConditionallySend> Sendable<H> {
    #[deprecated(note = "Use Sendable::into_inner() instead")]
    pub fn unwrap(self) -> H {
        self.0
    }

    pub fn into_inner(self) -> H {
        self.0
    }
}

impl<H> Debug for Sendable<H>
where
    H: Debug + ConditionallySend,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Sendable").field(&self.0).finish()
    }
}

pub trait ConditionallySend: Sized {
    /// Returns `true` if the handle can be sent to another thread.
    fn can_send(&self) -> bool;
    /// Wrap the handle in a type that can be sent to another thread and unwrapped there.
    ///
    /// Guaranteed to succeed of can_send() returns `true`.
    fn wrap_send(self) -> Result<Sendable<Self>, Self>;
}

/// `RCHandle<H>` is conditionally Send and can be sent to
/// another thread when its reference count is 1.
impl<H: NativeRefCountedBase> ConditionallySend for RCHandle<H> {
    fn can_send(&self) -> bool {
        unsafe { H::unique(self.0.as_ptr()) }
    }

    fn wrap_send(self) -> Result<Sendable<Self>, Self> {
        if self.can_send() {
            Ok(Sendable(self))
        } else {
            Err(self)
        }
    }
}

/// Functions that are (supposedly) _safer_ variants of the ones Rust provides.
pub(crate) mod safer {
    use core::slice;
    use std::ptr;

    /// Invokes [`slice::from_raw_parts`] with the `ptr` only when `len` != 0, otherwise passes
    /// `ptr::NonNull::dangling()` as recommended.
    ///
    /// Panics if `len` != 0 and `ptr` is `null`.
    pub unsafe fn from_raw_parts<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
        unsafe {
            let ptr = if len == 0 {
                ptr::NonNull::dangling().as_ptr()
            } else {
                assert!(!ptr.is_null());
                ptr
            };
            slice::from_raw_parts(ptr, len)
        }
    }

    /// Invokes [`slice::from_raw_parts_mut`] with the `ptr` only if `len` != 0, otherwise passes
    /// `ptr::NonNull::dangling()` as recommended.
    ///
    /// Panics if `len` != 0 and `ptr` is `null`.
    pub unsafe fn from_raw_parts_mut<'a, T>(ptr: *mut T, len: usize) -> &'a mut [T] {
        unsafe {
            let ptr = if len == 0 {
                ptr::NonNull::dangling().as_ptr() as *mut _
            } else {
                assert!(!ptr.is_null());
                ptr
            };
            slice::from_raw_parts_mut(ptr, len)
        }
    }
}

#[cfg(test)]
mod tests {
    struct PanicOnDrop {}
    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            panic!("Shall not drop")
        }
    }
    #[test]

    fn try_construct_does_not_drop_when_construction_attempt_fails() {
        _ = super::try_construct::<PanicOnDrop>(|_| false);
    }
}
