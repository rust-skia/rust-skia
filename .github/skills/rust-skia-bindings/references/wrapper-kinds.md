# Wrapper Kinds (`skia-safe/src/**`)

The safe half of the boundary. `skia-safe` mirrors Skia's `include/` layout:
`src/core/` ← `include/core/`, `src/gpu/ganesh/` ← `include/gpu/ganesh/`,
`src/modules/paragraph/` ← `modules/skparagraph/include/`, and so on.

## Which wrapper kind?

| The C++ type is… | Rust wrapper | Also implement |
|---|---|---|
| an `enum class` re-exported as-is | `pub type Foo = skia_bindings::SkFoo;` | `variant_name!` |
| an enum needing custom discriminants | Rust enum + `native_transmutable!` | `Copy, Clone, PartialEq, Eq, Hash, Debug` |
| a bit mask (`*Mask`/`*Flags`/`*Bits`) | `bitflags!` block | `Default` if one exists |
| a POD struct with identical layout | `#[repr(C)] struct` + `native_transmutable!` | derives |
| value semantics with ctor/dtor | `Handle<SkFoo>` | `NativeDrop` (+ `NativeClone`, `NativePartialEq`, `NativeHash`) |
| derives `SkRefCnt` | `RCHandle<SkFoo>` | `NativeRefCountedBase` |
| custom refcounting | `RCHandle<SkFoo>` | full `NativeRefCounted` |
| heap-only, or has interior pointers | `RefHandle<SkFoo>` | `NativeDrop` (calls `delete`) |
| trivially thread-safe | any of the above | `unsafe_send_sync!(Foo)` |

### Plain re-exports are for primitives and enums

Re-exporting the bindgen type (`pub type Foo = skia_bindings::SkFoo;` or
`pub use skia_bindings::SkFoo as Foo;`) is the right choice for primitives, scalar
aliases, and enums — that is the overwhelming majority of the ~150 re-exports in the
tree. It is rare for a struct, because a re-export exposes the generated field names
verbatim.

bindgen keeps the C++ member name, so a Skia struct member arrives as
`fUseDrawListLayer`, not `use_draw_list_layer`:

```rust
pub struct skgpu_graphite_ContextOptions {
    pub fUseDrawListLayer: bool,
    pub fGpuBudgetInBytes: usize,
    pub fRequireOrderedRecordings: bool,
    // …
}
```

Re-exporting that would make callers write `options.fUseDrawListLayer`. Wrap it
instead (`Handle<SkFoo>` with accessor methods), which is what `ContextOptions` does
and what makes direct field access inside the wrapper acceptable.

The exception is a struct whose C++ members already have Rust-idiomatic names —
`GrDriverBugWorkarounds` is the one instance, because Skia's `GPU_OP` macro emits the
snake_case name as the second argument:

```cpp
GPU_OP(ADD_AND_TRUE_TO_LOOP_CONDITION, add_and_true_to_loop_condition)
```

Its wrapper is a plain re-export plus a naming test that pins the generated field:

```rust
pub use skia_bindings::GrDriverBugWorkarounds as DriverBugWorkarounds;

#[test]
fn test_driver_bug_workarounds_naming() {
    fn _n(workarounds: &DriverBugWorkarounds) {
        let _ = workarounds.max_fragment_uniform_vectors_32;
    }
}
```

If a struct must be re-exported with non-idiomatic members, the rename belongs in
`skia_bindgen.rs`, next to the existing renamers. `ParseCallbacks::field_name` exists in
bindgen 0.73.2 but is not implemented, so only `enum_variant_name` and `item_name`
renaming happens today. The change has the shape of `ITEM_RENAMES`:

```rust
fn field_name(&self, info: bindgen::callbacks::FieldInfo<'_>) -> Option<String> {
    FIELD_RENAMES
        .iter()
        .find(|(type_name, field_name, _)| {
            *type_name == info.type_name && *field_name == info.field_name
        })
        .map(|(_, _, replacement)| replacement.to_string())
}

const FIELD_RENAMES: &[(&str, &str, &str)] = &[
    ("skgpu_graphite_ContextOptions", "fUseDrawListLayer", "use_draw_list_layer"),
];
```

`info.field_name` is the original C++ member name, and the callback also fires for
bitfields. `info.type_name` is bindgen's canonical name for the containing type rather
than its C++ spelling — log it once to confirm the value before writing the table.

Two consequences to plan for: the rename changes the generated bindings, so every
wrapper that reads the field must follow in the same commit; and it is a
`skia-bindings`-level change, so it needs a `cargo check -p skia-bindings` with the
affected feature enabled. Do not add a second, hand-maintained name mapping in
`skia-safe`.

### `native_transmutable!` — bit-for-bit compatible value types

Use when Rust can own exactly the same bytes as the C++ type. The macro implements
`NativeTransmutable` and asserts size **and** alignment compatibility at compile time,
so a wrong `#[repr]` or a field that does not exist in the C++ type fails the build
rather than producing a silently misaligned wrapper:

```rust
macro_rules! native_transmutable {
    ($nt:ty, $rt:ty) => {
        impl $crate::prelude::NativeTransmutable<$nt> for $rt {}
        const _: () = {
            $crate::prelude::assert_layout_compatible::<$nt, $rt>();
        };
    };
}
```

The wrapper gets `native()` / `native_mut()` / `from_native_c()` / `into_native()`
without copying, and slices transmute through `NativeTransmutableSliceAccess`, which is
what makes `&[Point]` usable as `&[SkPoint]` in FFI calls.

#### Newtype over a scalar or an existing bindgen type

Use `#[repr(transparent)]` and keep the single field private. `Color` wraps the
bindgen scalar alias `SkColor`; `FontStyle` wraps the bindgen struct `SkFontStyle`:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct Color(SkColor);

native_transmutable!(SkColor, Color);
```

```rust
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FontStyle(SkFontStyle);

native_transmutable!(SkFontStyle, FontStyle);
```

A newtype is the right shape when the C++ type is a single value — an integer, a
scalar, or one packed field — or when the Rust side wants to add constants and methods
without exposing the native field name. `SkFontStyle` is the second case: it is a
single private `int32_t fValue` packing weight, width, and slant, so the wrapper adds
`Weight::BLACK`, `Width::CONDENSED`, `Slant`, and a `Deref` to the weight rather than
exposing `fValue`.

#### Struct with matching members

Use `#[repr(C)]` and mirror the C++ members exactly — same names (snake_case, no `f`
prefix), same order, same types. Derive `Copy, Clone, PartialEq, Debug`, plus `Eq` and
`Default` where the value permits:

```rust
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

native_transmutable!(SkIRect, IRect);
```

`Point`/`IPoint`, `Point3`, `Rect`, `V4` follow the same shape. `Matrix` and `V2`/`V3`
show two variations: `Matrix` mirrors a nested array (`mat: [scalar; 9]`) plus a
private `type_mask`, and both omit `#[repr(C)]` — the layout assert is what guarantees
correctness, `#[repr(C)]` only makes the field order explicit. Prefer `#[repr(C)]` for
a new type; it documents the intent and removes reliance on Rust's default layout.

#### Transmutable iterator or handle with a lifetime

A native type that Rust only ever holds by reference can carry a `PhantomData` to tie
it to its owner. The `PhantomData` is zero-sized, so the layout still matches:

```rust
#[derive(Clone)]
#[repr(transparent)]
pub struct Iterator<'a>(SkRegion_Iterator, PhantomData<&'a Region>);

native_transmutable!(SkRegion_Iterator, Iterator<'_>);
```

#### Enum with explicit discriminants

When the C++ enum's values cannot be reproduced by bindgen's renaming, mirror the enum
in Rust with the C++ discriminants and transmute it. This is the fallback for
`pub use`d enums, and it is why the discriminants must be written as `as _`
expressions rather than literals — they stay in sync with Skia automatically:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(i32)]
pub enum ColorType {
    Unknown = SkColorType::kUnknown_SkColorType as _,
    Alpha8 = SkColorType::kAlpha_8_SkColorType as _,
    // …
}

native_transmutable!(SkColorType, ColorType);
```

Use `#[repr(i32)]` or `#[repr(u8)]` to match the C++ underlying type, and add a
`from_native_c`/`into_native` round-trip test. A newly added Skia variant shows up as a
changed public header, so the milestone update's header accounting is what catches a
mirror that fell behind — see the `skia-milestone-update` skill.

### `Handle<T>` — inline storage

Use when the C++ type is not refcounted, cannot be `memcpy`'d, has no interior
pointers, and is not pointed at persistently by another C++ object. All four matter,
because Rust moves values freely.

```rust
pub type Paint = Handle<SkPaint>;
unsafe_send_sync!(Paint);

impl NativeDrop for SkPaint {
    fn drop(&mut self) {
        unsafe { sb::C_SkPaint_destruct(self) }
    }
}
```

Construction goes through placement new, never `zeroed()`:

```rust
pub fn new() -> Self {
    Self::construct(|options| unsafe { sb::C_ContextOptions_Construct(options) })
}
```

`Handle`'s `PhantomData<*const ()>` deliberately makes it `!Send`/`!Sync`, so
thread-safety is always an explicit decision.

### `RCHandle<T>` — reference counted

```rust
pub type Typeface = RCHandle<SkTypeface>;
unsafe_send_sync!(Typeface);
require_base_type!(SkTypeface, sb::SkWeakRefCnt);

impl NativeRefCountedBase for SkTypeface {
    type Base = SkRefCntBase;
}
```

`NativeRefCountedBase` is enough when the counter lives in `SkRefCntBase`; implement
`NativeRefCounted` in full only for a custom counter (`SkData`). Assert the base-class
assumption with `require_base_type!`, and `require_type_equality!` when a
`*_INHERITED` alias must name the expected base.

`clone()` copies the pointer and increases the count; `drop()` decreases it. To move
a refcounted value across threads safely, use `Sendable`/`ConditionallySend`, which
checks that the count is 1.

### `RefHandle<T>` — heap object

Use for types C++ allocates with `new` and for types that cannot be `Handle<T>`
because of interior pointers. `NativeDrop` must call `delete`, not `unref`.

## Traits

| Trait | Purpose |
|---|---|
| `NativeDrop` | `drop()` for a bindgen type; required by `Handle`/`RefHandle` |
| `NativeClone` | `clone()` for a bindgen type |
| `NativePartialEq` | custom equality (`C_SkPaint_Equals`) |
| `NativeHash` | custom hashing |
| `NativeAccess` | `native()` / `native_mut()` / `native_mut_force()` |
| `NativeRefCounted` / `NativeRefCountedBase` | refcounting |
| `NativeTransmutable<NT>` | bit-for-bit layout compatibility |
| `NativeBase<Base>` | upcast to a C++ base class |

## Macros (`skia-safe/src/macros.rs`)

- `native_transmutable!(Native, Rust)` — implements `NativeTransmutable` and asserts
  size and alignment compatibility at compile time.
- `variant_name!(Foo::Variant)` — asserts a variant exists. Enum renaming in bindgen
  is silent, so this is the compile-time guard against a renamed variant.
- `require_type_equality!` / `require_base_type!` — assert a base-class assumption.
- `unsafe_send_sync!(Foo)` — implements `Send` and `Sync`. There is no
  `unsafe_send_sync_impl!`.

## Debug

Derive `Debug` when the wrapper is a plain Rust struct or enum. Otherwise write it by
hand, in the order of the C++ accessors:

```rust
impl fmt::Debug for Paint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Paint")
            .field("is_anti_alias", &self.is_anti_alias())
            .field("is_dither", &self.is_dither())
            // …
            .finish()
    }
}
```

Use `let mut d = f.debug_struct(...)` when some fields are `cfg`-gated. Use
`f.debug_struct("Foo").finish()` for a type with no informative accessors. A native
`dump()`/`toString()` is wrapped as a normal method (`dump_to_string`), not as
`Debug`.

## Send & Sync

Decide per type; never rely on the default.

- `RCHandle`: `Send`/`Sync` only if the type is immutable or documented as thread-safe.
- `Handle`/`RefHandle`: `Send`/`Sync` unless the type refers to other types, has
  unprotected interior mutability, or is not meant to be shared.
- `Send` without `Sync` is a real category — a Graphite `Recording` is moved between
  threads but must not be shared. Write the rationale in a comment.
- A thread-affine type stays `!Send`/`!Sync` with a comment saying so.
- Assert the result in `skia-safe/tests/send_sync.rs` with
  `static_assertions::{assert_impl_all!, assert_not_impl_any!}`.

## Enums

```rust
pub use skia_bindings::SkBlendMode as BlendMode;
variant_name!(BlendMode::ColorBurn);
```

The `k` prefix is stripped by bindgen's `enum_variant_name` callback, not in
`skia-safe`. `*Mask`, `*Flags`, and `*Bits` are constified and wrapped with
`bitflags!`; `from_bits_truncate` on the way in, `bits()` on the way out.

Add `impl From<…>` for the unambiguous conversions, including `impl From<T> for
Wrapper` where the wrapper is a value.

## Strings, slices, spans

- `interop::String` is `Handle<SkString>`; use `as_str()`, `String::from_str`, and
  `SetStr` for mutation. `AsStr` also covers `sb::std_string` / `sb::std_string_view`.
  Use `FromStrs` to build a `Vec<String>` from `&[impl AsRef<str>]`.
- Rust slices cross as `(ptr, len)`; use `points.native().as_ptr()` with
  `NativeTransmutableSliceAccess` to transmute `&[Point]` into `&[SkPoint]`.
- Returned slices must use `safer::from_raw_parts`, not `slice::from_raw_parts`,
  because Skia returns null for empty ranges.
- `SkSpan` is never part of the Rust API.
- Prefer `impl Into<T>` for simple types, `impl AsRef<T>` for `Handle`/`RCHandle`
  parameters, and `impl Into<Option<T>>` where a default is not wanted.

## Out-parameters, null, and `Option`

- `RCHandle::from_ptr` — takes ownership, `None` on null.
- `RCHandle::from_unshared_ptr` — shares, increases the count, `None` on null.
- `RCHandle::from_ptr_const` — for a `const T*`.
- `Handle::construct` / `try_construct` — placement construction; `try_construct`
  returns `Option` for a `bool`-returning constructor.
- `bool` + out-param → `Option`, e.g. `.then_some((p, v))`.
- `Option<Self>` is also the idiom for a platform-optional API that returns `nullptr`
  where Skia was built without the backend.
- `native_mut_force()` when a Skia free function takes a non-const pointer but the
  Rust API only has `&self`; `native_ptr_or_null_mut_force()` for the `Option<&H>`
  variant.

## Tests

Put a `#[cfg(test)] mod tests` block in the wrapper file. Assert the things that
would silently break:

- enum / conversion round-trips (`ColorType::from_native_c(...)`),
- layout, via `native_transmutable!`'s compile-time assert,
- refcount behaviour using the test-only `RefCount` trait,
- null handling (`assert!(Typeface::from_unshared_ptr(null).is_none())`),
- the specific behaviour the wrapper exists for.

Integration tests belong in `skia-safe/tests/` and are limited to feature matrices
(`codec.rs`), `Send`/`Sync` assertions (`send_sync.rs`), and end-to-end cases
(`shaper.rs`, `images/`).
