---
status: proposed
---

# Skia runs on the `wgpu` crate through a shim behind Dawn's function table

Skia must run inside a `wgpu` application on every backend that `wgpu` supports, including
Direct3D 12 and OpenGL ES, and share the application's device, queue, and textures. Graphite has
native backends only for Metal and Vulkan (`src/gpu/graphite/mtl`, `src/gpu/graphite/vk`), so
only its Dawn backend can reach all of them. When Skia runs on `wgpu`, it therefore always uses
its Dawn backend, also where `wgpu` itself runs on Metal or Vulkan. Ganesh is not part of this
path, because upstream Skia removed Dawn from Ganesh ("Remove support for Dawn in Ganesh",
`0073862948`, August 2023).

## How Skia's calls reach `wgpu`

Skia's Dawn backend talks to WebGPU only through Dawn's header-only C++ wrapper
`webgpu/webgpu_cpp.h`. The wrapper calls the C functions of `webgpu.h`, which all start with
`wgpu` (`include/gpu/graphite/dawn/DawnBackendContext.h`). No file in `src/gpu/graphite/dawn`
calls Dawn's implementation directly. rust-skia compiles the backend against Dawn's generated
headers and links none of Dawn's implementation.

Skia's GN build compiles the Dawn backend only together with all of Dawn, which it builds with
CMake (`skia_use_dawn` adds `//third_party/dawn`). `skia-bindings` therefore leaves
`skia_use_dawn` off and compiles the backend's sources itself, the `.cpp` files of
`skia_graphite_dawn_sources` in `gn/graphite.gni`, in the same `cc` build as its bindings and with
the same defines. Nothing in Skia's library outside the backend tests `SK_DAWN`, and the core
Graphite code handles `BackendApi::kDawn` without conditions. `libskia.a` is therefore the same
with and without the `wgpu` feature.

The `wgpu*` functions reach Rust through Dawn's `dawn_proc`. `dawn_proc` is a generated C++ file (`src/dawn/dawn_proc.cpp`) in
which every `wgpu*` function forwards through a `DawnProcTable`, a struct with one function pointer
per WebGPU function. Upstream Skia links `dawn_proc` too and installs real Dawn's table with
`dawnProcSetProcs` (`tools/graphite/dawn/GraphiteDawnTestContext.cpp`). rust-skia compiles
`dawn_proc` next to the backend into `libskia-bindings.a` and installs a table of Rust functions
instead. Each of these
functions converts its C arguments and calls the `wgpu` crate. That set of functions is called the
shim. A `WGPUDevice` handle wraps a clone of the application's `wgpu::Device`, so Skia and the
application share one device.

The `wgpu*` symbols resolve inside `libskia-bindings.a`, the library that calls them, so the link
order of the Rust crates does not
matter, and the shim can live in a crate that depends on `skia-bindings`. `skia-bindings`
generates `DawnProcTable` and the WebGPU types in its existing bindgen run. Each table entry is a
typed function pointer, so rustc checks every shim function's signature against the header. The
shim builds the table as a struct literal that names every field, so a function added to the
header fails to compile until the shim handles it.

## Which headers

Skia is compiled against Dawn's `webgpu.h` and not against the standard one from
`webgpu-headers`. Skia's native code path references Dawn extensions such as
`wgpu::YCbCrVkDescriptor`, `wgpu::TextureFormat::R8BG8Biplanar420Unorm`,
`wgpu::FeatureName::DawnLoadResolveTexture`, and `wgpu::Device::Tick`. Only Skia's 74
`#if defined(__EMSCRIPTEN__)` branches avoid these extensions. Dawn's header therefore compiles
Skia's sources without a fork patch. The same generated header is the input of the backend's
compilation and of bindgen, so both sides agree on struct layouts and enum values. The shim reports every Dawn
extension feature as unsupported from `wgpuDeviceHasFeature`, and Skia takes its fallback paths
(`src/gpu/graphite/dawn/DawnCaps.cpp`).

Source builds with the `wgpu` feature generate the headers and the `dawn_proc` files. The fork
removed Dawn from `DEPS` ("rust-skia: Trim dependencies"), so `skia-bindings` records the Dawn
revision as `[package.metadata].dawn`, next to the Skia fork tag. The build downloads GitHub's
archive of that revision (about 26 MB), unpacks only the generator, `src/dawn/dawn.json`, a few
static headers, and the license, and runs Dawn's generator with the Jinja2 copy from Skia's
dependency sync. `SKIA_DAWN_SOURCE_DIR` points offline builds to a Dawn checkout instead. Builds
without the `wgpu` feature run none of this, and prebuilt binaries already contain everything
compiled from the headers. Skia moved its Dawn revision at every milestone from m140 to m157,
and `dawn.json` changed in each of those intervals (1 to 21 commits, per the commit history of
the `google/dawn` GitHub mirror). The milestone update therefore copies the revision from Skia's
upstream `DEPS`, and it reviews the difference between the old and the new generated headers,
because no checked-in file shows it.

One generated set serves every platform. Dawn's generator selects the items of `dawn.json` by
API tags (`dawn`, `native`, `emscripten`, `art`), not by operating system, and it generates the
native headers with the tags `dawn`, `native`, and `deprecated` (`generator/dawn_json_generator.py`
at Skia's pinned Dawn revision). The only platform differences in the output are preprocessor
conditions, such as the `_WIN32` export attributes, which the compiler resolves on each target.
The Rust types that bindgen generates from the headers, however, are specific to each target.
C enum types are unsigned with Clang and GCC but signed with MSVC, and pointer sizes and the
alignment of 64-bit fields differ on 32-bit targets. `skia-bindings` already runs bindgen per
target, so the shim uses the generated types and constants and never assumes an integer
representation.

## Crates and `wgpu` versions

The shim lives in its own crate, `skia-wgpu`, which depends on `skia-bindings` and `wgpu`.
`skia-safe` depends on `skia-wgpu`, so a `wgpu` backend is chosen as a `skia-safe` feature, like
`metal` or `d3d`. `skia-safe`'s `d3d` feature already sets the precedent of a public API that uses
a GPU API crate's types (`skia-safe/src/gpu/d3d.rs` re-exports types of the `windows` crate).

An application can only hand its device to Skia if both use the same `wgpu` major version, and
frameworks pin different majors. `skia-wgpu` therefore supports several `wgpu` major versions at
once. Each version has a feature, `wgpu-NN`, that enables an optional dependency renamed to
`wgpuNN` (`wgpu30 = { package = "wgpu", version = "30" }`). Cargo allows several majors in one
dependency graph, because none of the `wgpu` crates declares a `links` key. One shim source is
compiled once per enabled version, as the module `vNN`. Inside `vNN`, the name `wgpu` refers to
that version's crate. The few places where two `wgpu` majors differ go into a per-version `compat`
module, so the shared source never names a version.

The two sides of the shim change at different rates. Dawn's headers change once per Skia
milestone, and a rust-skia release has exactly one milestone, so every `vNN` module of a release
converts from the same generated C types. A Dawn update is therefore made once, in the shared
source. Only the `wgpu` side differs between `vNN` modules.

`skia-wgpu` sits below `skia-safe`, so its API cannot return `skia-safe` types. Each `vNN` module
turns `wgpu` objects into version-independent handle types of `skia-wgpu`, for example
`skia_wgpu::Device`, which owns one reference to a `WGPUDevice`. `skia-safe` takes only these
handle types, so its Graphite API exists once, whatever `wgpu` versions are enabled. `skia-safe`
forwards each `wgpu-NN` feature to `skia-wgpu`, so an application adds only `skia-safe`.
`skia-bindings` has one `wgpu` feature, which builds the Dawn backend and `dawn_proc` and does not
depend on the `wgpu` version.

## Consequences

- The shim implements the `wgpu*` functions that Skia's Dawn backend calls, about 55 methods plus
  reference counting for about 19 object types. The functions change with each Skia milestone.
- Skia's SkSL compiler emits WGSL that Dawn's Tint compiler accepts. `wgpu` compiles WGSL with naga
  instead, and it is not yet verified that naga accepts Skia's output.
- Everything the shim does passes through `wgpu`, so `wgpu` tracks resource state and
  synchronization between Skia's work and the application's work. `wgpu` validation adds CPU
  overhead compared with Graphite's native backends.
- `dawnProcSetProcs` installs one table for the whole process. Only one `wgpu` version can
  therefore drive Skia in a process at a time. Creating a context with a second version while
  contexts of the first are alive fails with an error, because a handle created by one version's
  shim must never reach another version's shim.
- Adding a `wgpu` major version adds an optional feature, which is compatible under semver. It
  ships as a patch release of `skia-wgpu` and `skia-safe` within the current milestone. Removing a
  version breaks users, so it only happens with a milestone version bump. The first release
  supports only the newest `wgpu` major version.

## Considered Options

- **Rejected: link Dawn, as upstream Skia does.** A `wgpu` application would run two WebGPU
  implementations that cannot share a device or a texture.
- **Rejected: keep Dawn in `DEPS` and build it, only to use Skia's `skia_use_dawn` unchanged.**
  `tools/git-sync-deps` syncs every `DEPS` entry, so every source build would fetch Dawn (about
  530 MB on disk), with or without the `wgpu` feature. Skia builds Dawn and Tint with CMake, which
  rust-skia's build cannot require.
- **Rejected: a fork patch to Skia's GN build that compiles only `dawn_proc`.** It works, but it
  is one more patch to rebase and review at every milestone, for what `skia-bindings` can compile
  itself.
- **Rejected: share `wgpu`'s native handles with Graphite's Metal and Vulkan backends.** `wgpu`
  exposes its `MTLDevice`, `MTLCommandQueue`, `VkDevice`, and textures through `as_hal`. This needs
  far less code than the shim and runs at native speed, but it cannot reach Direct3D 12 or
  OpenGL ES. On Vulkan, Skia and `wgpu` would also have to hand image layouts and queue access to
  each other explicitly.
- **Rejected: use wgpu-native as the `webgpu.h` implementation.** wgpu-native implements the
  standard header on `wgpu-core` with its own instance. It does not create its handles from `wgpu`
  crate objects, so an application's `wgpu::Device` cannot be handed to Skia.
- **Rejected: export the `wgpu*` functions from a Rust crate below `skia-bindings`.** A missing
  function would fail at link time, and no runtime install step would be needed. GNU ld scans each
  library once in order, and only a dependency of `skia-bindings` is linked after its libraries, so
  `wgpu` would enter the lowest crate. That crate could not use the WebGPU types of
  `skia-bindings`, so a second bindgen run or a shared `-sys` crate would be needed.
- **Rejected: compile Skia's Emscripten code path natively against the standard header.** The
  shim would not need Dawn-only functions, but the fork would carry a patch across the 74
  `__EMSCRIPTEN__` sites.
- **Rejected: one `wgpu` feature that adapts to the application's `wgpu` version.** A dependency
  range across majors (`wgpu = ">=29, <31"`) lets Cargo pick a version that differs from the
  application's, and the shim would have to compile against whichever version Cargo picks. A
  `wgpu-NN` feature states the version explicitly.
- **Rejected: `skia-wgpu` depending on `skia-safe`.** `skia-safe` would stay free of `wgpu`, but
  applications would have to add a second crate, and `wgpu` would be the only GPU backend that is
  not a `skia-safe` feature.
