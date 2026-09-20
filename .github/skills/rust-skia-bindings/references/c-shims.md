# C Shims (`skia-bindings/src/*.cpp`)

The low-level half of the boundary. Bindgen parses these `.cpp` files **as headers**
(`-x c++`), and `cc` compiles the same files into `libskia-bindings.a`, so a shim is
both an API declaration and an implementation. There is no generated umbrella header;
every file includes the hand-written `skia-bindings/src/bindings.h`.

## File wiring

The authoritative list of shim files is the feature-gated `binding_sources` in
`skia_bindgen::Configuration::new` (`skia-bindings/build_support/skia_bindgen.rs`).
Each entry is consumed twice in one loop — `cc_build.file(source)` and
`builder.header(source)` — plus `cargo::rerun_if_file_changed(source)`. A shim in a
file that is not in this list is invisible to both bindgen and the linker, and adding
a file means adding it there under the matching feature gate.

One file per upstream header area, so put a new shim in the file that already wraps
that header, and add a new file only when an area has none. Read the list in
`skia_bindgen.rs` for the current mapping; do not rely on a copy of it.

`bindings.cpp` is the catch-all for `codec/`, `core/`, `docs/`, `effects/`, `encode/`,
`pathops/`, `svg/`, and `utils/`, and groups shims with `// path/file.h` banner
comments.

## Naming

`C_<Scope>_<name>`. Bindgen's `allowlist_function("C_.*")` is the only reason the
`C_` prefix exists: it is the namespace. Name parts are separated by `_`; static
functions and factories begin with an uppercase letter, member methods with a
lowercase one. Preserve the original C++ casing of the method name
(`getScaledDimensions`, `isValid`, `makeImageSnapshot`).

| Suffix | Meaning | Example |
|---|---|---|
| `_Construct(out*, …)` | placement `new` into an out-parameter | `C_SkPath_Construct(SkPath*, SkPathFillType)` |
| `_CopyConstruct(out*, src*)` | copy constructor into an out-parameter | `C_SkCodecs_Decoder_CopyConstruct` |
| `_destruct(self*)` | explicit destructor call, pairs with `NativeDrop` | `C_SkPaint_destruct(SkPaint*)` |
| `_new(…)` / `_delete(self*)` | heap object Rust owns by pointer | `C_SkPathBuilder_new()` / `_delete()` |
| `_Make*` | static factory returning a raw pointer | `C_SkCodec_MakeFromStream` |
| `_get*` / `_set*` | property accessor when direct field access is not possible | `C_SkPaint_setShader` |
| `_ref` / `_unref` / `_unique` | refcounting, consumed by `NativeRefCounted` | `C_SkData_ref` |
| `_Equals` | equality, consumed by `NativePartialEq` | `C_SkPaint_Equals` |
| `_Types` / `_UnreferencedTypes` | anchor function, see below | `C_Core_Types` |

Numbers are appended to disambiguate overloads (`C_SkCodec_MakeFromData2`).

The case of the first letter of the last name part follows the C++ function kind:
a member method keeps its lowercase name (`_makeImageSnapshot`, `_isValid`), a static
or free function is capitalized (`_MakeFromStream`, `_Equals`, `_NumChannelsInPlane`).
An `operator==` wrapper is a free function, so it is always `_Equals` — never
`_equals`, even when the shim takes a `self` pointer.

Returning by value through an out-parameter is the norm for non-trivial results:

```cpp
extern "C" void C_SkCodec_getInfo(const SkCodec* self, SkImageInfo* info) {
    *info = self->getInfo();
}
```

## Anchor functions for unreferenced types

Bindgen only emits types reachable from an allow-listed `C_*` function. A no-op
function with dummy pointer parameters forces generation of otherwise unreferenced
types:

```cpp
extern "C" void C_Core_Types(SkArc *, SkGraphics *, SkCoverageMode *,
                             SkColorChannelFlag *,
                             SkSurfaces::BackendSurfaceAccess) {}
extern "C" void C_GraphiteUnreferencedTypes(skgpu::Budgeted *, skgpu::Mipmapped *,
                                            skgpu::Budgeted *) {}
```

When the type is not a real Skia type, use a local proxy — see
`ganesh.cpp`: `typedef std::optional<uint64_t> OptionalU64;`.

## Smart pointers

`sk_sp<T>` is generated as a plain struct with an `fPtr` and no destructor. The key
fact: **`sk_sp(T*)` adopts the pointer without incrementing the reference count**.
The helpers in `bindings.h` are `sp(T*)` and `spFromConst(const T*)`.

### Returning ownership to Rust

Release the smart pointer and let Rust take the pointer:

```cpp
extern "C" SkImage* C_SkImage_MakeFromBitmap(const SkBitmap* bitmap) {
    return SkImage::MakeFromBitmap(*bitmap).release();
}
```

Rust wraps it with `from_ptr` (takes ownership, does not increase the count) or
`from_unshared_ptr` (shares, increases the count). Rule of thumb from the wiki: when
a refcounted value crosses the boundary, the **sender** is responsible for the
reference that the receiver's sharing requires.

### Taking ownership from Rust

A shim parameter of type `T*` that is wrapped with `sp(ptr)` or `sk_sp<T>(ptr)`
adopts the reference, so the Rust call site must hand over an owned one:

```cpp
extern "C" void C_SkPaint_setShader(SkPaint* self, SkShader* shader) {
    self->setShader(sp(shader));
}
```

```rust
// ownership is transferred, so use into_ptr_or_null, not a borrowed pointer
unsafe { sb::C_SkPaint_setShader(self.native_mut(), shader.into().into_ptr_or_null()) }
```

Getting this wrong is a refcount underflow and a use-after-free, so the call site
carries a comment when the shim adopts:

```rust
// `C_SkImages_WrapTextureGraphite` adopts the color space (the shim wraps the
// raw pointer in an `sk_sp` *without* adding a ref), so transfer an owned
// reference via `into_ptr_or_null`. A borrowed pointer would let Skia release
// a ref it never retained — a refcount underflow / use-after-free.
```

When the caller keeps using the object, hand over a *fresh* reference instead:
`surface.clone().into_ptr()`.

### `const T*` parameters

Use `spFromConst` when the C++ API takes a `const T*` and stores it in an `sk_sp`.

### Reading an `sk_sp` member

Expose the raw `fPtr` and let Rust decide:

```rust
pub fn shader(&self) -> Option<Shader> {
    Shader::from_unshared_ptr(self.native().fShader.fPtr)
}
```

### `std::unique_ptr<T>`

Ownership is transferred with the adopting constructor
(`std::unique_ptr<SkStream>(stream)`) or with `.release()`. `std::unique_ptr` is
opaque to bindgen, so Rust never models it. A `unique_ptr`-owned result must be
`delete`d on the Rust side, not unref'd:

```cpp
// skgpu::graphite::Context is owned via std::unique_ptr ... It is NOT
// ref-counted, so the Rust wrapper must `delete` it rather than unref a
// (non-existent) SkRefCntBase.
extern "C" void C_Context_delete(skgpu::graphite::Context* self) { delete self; }
```

### `sk_sp<T>` slices

The one place `sk_sp` appears in a Rust-visible signature is an accessor slice, which
Rust transmutes into `&[RCHandle<T>]` with `from_non_null_sp_slice`; the shim exposes
`const sk_sp<T>*` plus a count.

## `std::optional<T>`

Three surrogates, in order of preference:

1. `opt(pt)` from `bindings.h` for a nullable input pointer:
   `opt(alphaType)`.
2. A ternary for an inline value:
   `arguments ? std::optional(*arguments) : std::nullopt`.
3. An `int` sentinel when there is no other channel — `-1` means `nullopt`, `0`/`1`
   mean `false`/`true`:

```cpp
extern "C" int C_SkFontArguments_getSyntheticBold(const SkFontArguments* self) {
    auto syntheticBold = self->getSyntheticBold();
    if (!syntheticBold.has_value()) { return -1; }
    return syntheticBold.value() ? 1 : 0;
}
```

Rust models optionality as `Option<T>` and usually never sees `std::optional`.

## Strings and spans

`std::string`, `std::string_view`, and `SkString` cross as `(ptr, len)` through
length-carrying accessors:

```cpp
extern "C" const char* C_string_view_ptr_size(const std::string_view* self, size_t* size) {
    *size = self->size();
    return *size ? self->data() : nullptr;
}
```

`SkSpan<T>` never appears in the Rust API — a shim always constructs it from a
`(ptr, len)` pair coming from Rust:

```cpp
self->drawPoints(mode, SkSpan(points, pointCount), *paint);
```

`SkTArray`/`SkTDArray`/`skia_private::TArray` are never modelled either. Either add a
dedicated wrapper type (`struct SkStrings { std::vector<SkString> strings; };`) or
accept that the containing type must be heap-allocated (see
`references/exceptions.md`).

## Callbacks

`std::function` is opaque. Two mechanisms replace it.

**`Sink`/`VecSink`** for a Rust closure that C++ calls synchronously. `bindings.h`
declares a `TraitObject { void* data; void* vtable; }` plus a `set_fn` pointer; Rust
transmutes a `&mut dyn FnMut(..)` into that pair in `skia-safe/src/interop/cpp.rs`.
The `PhantomData<&'a mut …>` ties the sink to the borrow, so the callback cannot
escape the call.

**A C++ adapter class** overriding a virtual interface, driven by an
`extern "C"` function-pointer table — `rust_resource_provider.h`,
`shaper.cpp` (`RustRunHandler`), and the `RustStream`/`RustWStream` shims in
`bindings.cpp`. A Rust panic must not unwind into C++; catch it and abort:

```rust
match std::panic::catch_unwind(reader) {
    Ok(res) => res,
    Err(_) => {
        println!("Panic in FFI callback for `SkStream::read`");
        std::process::abort();
    }
}
```

## Compiler flags

`-std=c++20` on both bindgen and `cc`. `-fno-rtti` on non-MSVC; MSVC needs `/GR`
because `std::function` uses `typeid`. RTTI must match Skia's own build, otherwise
vtable layouts of inherited classes diverge. Layout-relevant `-D` definitions are
scraped from Skia's ninja files and applied to both bindgen and `cc` — that is what
keeps bindgen's layouts identical to the ones Skia was compiled with.
