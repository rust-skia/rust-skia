# BorrowSanitizer findings

[BorrowSanitizer](https://github.com/BorrowSanitizer/bsan) is a dynamic checker for Rust's aliasing
rules. It instruments Rust code and reports undefined behavior when memory is accessed in a way the
[Tree Borrows](https://www.ralfj.de/blog/2023/06/02/tree-borrows.html) model forbids. Unlike Miri, it runs the
real program, so it also sees accesses that happen while C++ code runs. This document lists the
problems it still reports in rust-skia and describes how to reproduce them.

The findings were collected on Skia `m156.5` with the BorrowSanitizer image
`ghcr.io/borrowsanitizer/bsan@sha256:805350ca503eac7d14ee445f8a2bf08a9798f8d1723df3de70addc31affbf276`.
The prebuilt Skia libraries were used, so the C++ code is not instrumented. BorrowSanitizer sees C++
accesses only where they go through functions it intercepts, such as `memcpy` and atomic
operations. Ordinary C++ loads and stores are invisible to it.

## Remaining problems

Each problem names the unit tests that report it. One test reports only its first problem, because
BorrowSanitizer stops at the first error.

### C++ writes through `const` pointers to `mutable` or atomic members

Rust forbids writes to memory behind a shared reference unless that memory is inside an
`UnsafeCell`. The types that bindgen generates contain no `UnsafeCell`. C++ code, however, writes to
`mutable` members and atomic counters through `const` pointers. When skia-safe passes the result of
`native()` (a `&N`) to such C++ code, the write is undefined behavior in Rust.

Reported instances:

- `FontMgr::legacy_make_typeface()` passes `&SkFontMgr`. The fontconfig font manager fills its
  `mutable` typeface cache in a `const` method. Reported by `core::font::tests::test_flags`,
  `core::font_mgr::tests::new_typeface_from_data_using_default`,
  `core::strike_ref::tests::strike_ref_widths_match_font_widths`, and four tests in
  `core::typeface::tests` (`family_name_iterator_returns_at_least_one_name_for_the_default_typeface`,
  `get_existing_font_data_of_default`, `get_font_bytes_of_default`,
  `serialize_and_deserialize_default_typeface`).
- `Matrix::get_type()` passes `&SkMatrix`. `SkMatrix::getType()` computes the type mask and stores
  it in the `mutable` member `fTypeMask`. Reported by
  `core::matrix::tests::setting_a_matrix_component_recomputes_typemask`.
- `Canvas::draw_image()` passes `&SkImage`. Drawing a lazily decoded image stores to the atomic
  member `fAddedToRasterCache` of `SkImage_Base`. Reported by the `skia-org` example, not by a unit
  test.

Known instances that no test reports:

- Every `RCHandle` native has an atomic reference count (`fRefCnt` in `SkRefCnt` and `SkNVRefCnt`).
  C++ increments it through a `const` pointer whenever it creates an `sk_sp` from a `const` object.
  This happens in many calls that receive `native()`.

A complete fix changes `native()` to return a raw pointer for every handle kind and rewrites about
490 call sites that call bindgen methods or read fields through `native()`. `Matrix` needs a separate
fix, because it is a `Copy` value type: C++ would receive a pointer to a mutable copy. This work is
deferred. The compiler does not currently exploit these writes, because skia-safe passes the pointer
to opaque C++ functions and does not read the affected memory around the call.

### Self-referential `RCHandle` types reached through `&mut`

Some C++ types store pointers into their own memory, for example in inline small-buffer arrays. A
`&mut` to such an object promises that nothing else writes to it, but C++ writes through the
internal pointers. skia-safe therefore calls these types through raw pointers. `RefHandle` offers no
`native_mut()`, so the compiler rules out the problem for heap-owned types that are not reference
counted. `RCHandle` still offers `native_mut()`. The reference-counted types known to be
self-referential (`SkCanvas` via `SkDocument`, the SVG containers, and `TypefaceFontProvider`) are
called through `RCHandle::native_ptr()`. Other self-referential `RCHandle` types are only found when
a test reaches them. No test reports such a type at the moment.

### Probable false positives in uninstrumented C++

These reports come from C++ code and system libraries without any Rust code involved. They look
like BorrowSanitizer losing track of pointers that uninstrumented libraries created.

- An out-of-bounds access in `SkDataTable::MakeCopyArrays()`, which copies family names returned by
  the fontconfig library. The reported offset (`0x1fbf9068a230` into an allocation of 56 bytes) is
  not plausible. Reported by `core::font_mgr::tests::create_all_typefaces`.
- A use-after-free in `SkSL::stod()`, at `buffer.imbue(std::locale::classic())`, which accesses
  libstdc++'s global locale. Reported by five tests in `core::mesh::tests`
  (`children_outlive_their_mesh`, `cloned_mesh_outlives_its_original`,
  `missing_uniforms_are_an_error`, `specification_errors_are_returned`,
  `specification_reflects_attributes_varyings_and_uniforms`).

## Reproducing the findings

BorrowSanitizer supports only Linux. On macOS, run it in its Docker image. The commands below mount
the repository into the container and keep the container's build output in a Docker volume, so the
host's `target/` directory is not touched.

```sh
docker run -d --name bsan-skia \
  -v "$PWD":/src -v bsan-skia-target:/target -v bsan-skia-cargo:/root/.cargo/registry \
  -e CARGO_TARGET_DIR=/target -w /src \
  ghcr.io/borrowsanitizer/bsan@sha256:805350ca503eac7d14ee445f8a2bf08a9798f8d1723df3de70addc31affbf276 \
  sleep infinity
docker exec bsan-skia bash -c 'apt-get update && apt-get install -y libfontconfig1-dev libfreetype-dev'
docker exec bsan-skia cargo bsan setup
```

The `skia-org` example stops at the first problem it reaches:

```sh
docker exec bsan-skia cargo bsan run -j 4 -p skia-org -- /target/org-out --driver cpu --driver pdf --driver svg
```

The unit tests give a more complete picture when each test runs as its own process, because a
problem then stops only its own test. Build the tests with a single job. Compiling the instrumented
`vulkano` dev-dependency in parallel exceeds 8 GB of memory.

```sh
docker exec bsan-skia cargo bsan test -j 1 -p skia-safe --lib --no-run
```

Then run each test from the built test binary, whose path `--no-run` prints. Run the loop inside the
container, for example in a shell from `docker exec -it bsan-skia bash`. Set `BSAN_SYMBOLIZER` so
that the reports contain source locations, and raise `stacktrace_max_len` to see the skia-safe
frames.

```sh
BIN=/target/bsan/aarch64-unknown-linux-gnu/debug/build/skia-safe/<hash>/out/skia_safe-<hash>
mkdir -p /target/bsan-tests
cd /src/skia-safe
for t in $($BIN --list 2>/dev/null | sed -n 's/: test$//p'); do
  BSAN_SYMBOLIZER=/root/.rustup/toolchains/bsan/bin/llvm-symbolizer \
  BSAN_OPTIONS=stacktrace_max_len=16 \
    $BIN --exact "$t" --test-threads 1 > "/target/bsan-tests/$t.log" 2>&1
done
```

BorrowSanitizer has no option to continue after an error or to suppress a known report.
`BSAN_OPTIONS=help=1` lists the options it supports.
