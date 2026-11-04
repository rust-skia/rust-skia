---
status: proposed
---

# Skia runs on the `wgpu` crate through WebGPU C functions that Rust exports

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

The crate `skia-wgpu` defines the `wgpu*` functions that the backend calls, as Rust functions
with `#[no_mangle]` and the C calling convention. Each of these functions converts its C
arguments and calls the `wgpu` crate. That set of functions is called the shim. The backend's
calls reach the shim directly. No function table and no install step sit between them. A
`WGPUDevice` handle wraps a clone of the application's `wgpu::Device`, so Skia and the
application share one device.

`skia-wgpu` depends on `skia-bindings`, so the linker sees the shim's library before
`libskia-bindings.a`, the library that calls the shim. GNU ld scans each library once, in order,
and loads from it only the members that define a symbol still undefined at that point. Library
order nevertheless does not matter here. rustc passes a generated object file, `symbols.o`, to
every link ahead of the libraries, and that file references each `#[no_mangle]` function of the
linked crates. The linker therefore always loads the shim's functions. An experiment with a
model of the three crates confirmed this. It linked and ran with GNU ld 2.44 on x86_64 Linux and
with ld64 on macOS, in debug and release builds, with thin and with fat LTO, and with the C++
library both bundled into the rlib (`static=`, as on Unix) and passed to the linker separately
(as on Windows).

`skia-bindings` generates the WebGPU types in its existing bindgen run, and the shim uses them.
`webgpu.h` declares a function pointer type for every function, such as
`WGPUProcDeviceCreateBuffer`. Next to each shim function, an assignment of the function to that
type (`const _: WGPUProcDeviceCreateBuffer = Some(wgpuDeviceCreateBuffer);`) makes rustc check
its signature against the header. A function that the backend calls and the shim lacks fails the
link with an undefined symbol that names the function. The shim implements only the functions
that the backend calls, 106 at m157, of the 276 functions in `webgpu.h`.

## Which headers

Skia is compiled against Dawn's `webgpu.h` and not against the standard one from
`webgpu-headers`. Skia's native code path references Dawn extensions such as
`wgpu::YCbCrVkDescriptor`, `wgpu::TextureFormat::R8BG8Biplanar420Unorm`,
`wgpu::FeatureName::DawnLoadResolveTexture`, and `wgpu::Device::Tick`. Only Skia's 74
`#if defined(__EMSCRIPTEN__)` branches avoid these extensions. Dawn's header therefore compiles
Skia's sources without a fork patch. The same generated header is the input of the backend's
compilation and of bindgen, so both sides agree on struct layouts and enum values. Which Dawn
extensions Skia may use is decided at runtime, as the section "Which features the shim reports"
describes.

Source builds with the `wgpu` feature generate the C header `webgpu.h` and the C++ wrapper
`webgpu_cpp.h`, together with the headers that they include. The fork removed Dawn from `DEPS` ("rust-skia: Trim
dependencies"), so `skia-bindings` records the Dawn revision as `[package.metadata].dawn`, next to
the Skia fork tag. The build downloads GitHub's archive of that revision (about 26 MB), unpacks
only the generator, `src/dawn/dawn.json`, a few static headers, and the license, and runs Dawn's
generator with the Jinja2 copy from Skia's dependency sync. `SKIA_DAWN_SOURCE_DIR` points offline
builds to a Dawn checkout instead. Builds without the `wgpu` feature run none of this, and
prebuilt binaries already contain everything compiled from the headers. Skia moved its Dawn
revision at every milestone from m140 to m157, and `dawn.json` changed in each of those intervals
(1 to 21 commits, per the commit history of the `google/dawn` GitHub mirror). The milestone update
therefore copies the revision from Skia's upstream `DEPS`, and it reviews the difference between
the old and the new generated headers, because no checked-in file shows it.

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

## Which features the shim reports

Skia asks `wgpuDeviceHasFeature` which optional WebGPU features the device has, and it takes a
fallback path for each feature that is missing. The shim answers from the features of the
application's `wgpu::Device`. A `wgpu` device has only the features that the application requested
when it created the device, so the application decides which of the reported features Skia can
use. The shim lists every feature of Dawn's header in its answer, so a feature that a new Dawn
revision adds fails to compile until the shim decides on it.

The table lists every feature that Skia's Dawn backend queries at m157, in
`src/gpu/graphite/dawn/DawnCaps.cpp` and in the format table of `DawnGraphiteUtils.cpp`. A
standard feature is part of the WebGPU specification. A Dawn feature is an extension that only
Dawn defines. A reported feature is one for which the shim answers `true` when the device has the
`wgpu` counterpart in the third column. The status is that of the shim on `wgpu` 30.

| Feature | Kind | `wgpu` 30 counterpart | Status | What Skia does without it |
|---|---|---|---|---|
| `CoreFeaturesAndLimits` | standard | none needed | always reported | Skia limits itself to the formats of WebGPU's compatibility mode. `wgpu` devices are never in that mode. |
| `TimestampQuery` | standard | `TIMESTAMP_QUERY` | reported | Skia reports no GPU timings. |
| `ShaderF16` | standard | `SHADER_F16` | reported | Shaders use 32-bit floats where Skia would use half precision. |
| `DualSourceBlending` | standard | `DUAL_SOURCE_BLENDING` | reported | Some blend modes that combine coverage with the destination read the destination through a texture copy. |
| `TextureCompressionBC`, `TextureCompressionETC2` | standard | `TEXTURE_COMPRESSION_BC`, `TEXTURE_COMPRESSION_ETC2` | reported | Skia cannot sample these compressed formats. |
| `RG11B10UfloatRenderable` | standard | `RG11B10UFLOAT_RENDERABLE` | reported | Skia cannot render into `RG11B10Ufloat`. |
| `BGRA8UnormStorage` | standard | `BGRA8UNORM_STORAGE` | reported | Skia cannot use `BGRA8Unorm` as a storage texture. |
| `Float32Filterable` | standard | `FLOAT32_FILTERABLE` | reported | Skia cannot filter 32-bit float textures. |
| `Float32Blendable` | standard | `FLOAT32_BLENDABLE` | not reported | Skia cannot blend into 32-bit float render targets. |
| `TextureFormatsTier1`, `TextureFormatsTier2` | standard | none | not reported | Skia lacks the render, multisample, and storage capabilities that the tiers add to some formats. |
| `Unorm16TextureFormats` | Dawn | `TEXTURE_FORMAT_16BIT_NORM` | not reported | Skia cannot use the 16-bit normalized formats. |
| `TransientAttachments` | Dawn | `TextureUsages::TRANSIENT_ATTACHMENT` | not reported | MSAA attachments have backing memory, which costs memory and bandwidth on Apple and other tile-based GPUs. |
| `MSAARenderToSingleSampled` | Dawn | none | not reported | Skia renders into a separate MSAA texture and resolves it into the target. |
| `DawnLoadResolveTexture`, `DawnPartialLoadResolveTexture` | Dawn | none | not reported | Skia loads the target into the MSAA attachment and resolves it in separate render passes. |
| `RenderPassRenderArea` | Dawn | none | not reported | A render pass cannot be limited to the area that Skia draws into. |
| `DawnAllowUndefinedLoadStoreOp` | Dawn | `LoadOp::DontCare`, behind an unsafe token | not reported | Skia clears an attachment whose previous contents it discards, instead of leaving them undefined. |
| `BufferMapExtendedUsages` | Dawn | `MAPPABLE_PRIMARY_BUFFERS`, which `wgpu` warns is slow on GPUs with their own memory | not reported | Skia cannot map vertex, index, and uniform buffers, and uploads their data through transfer buffers. |
| `DawnTexelCopyBufferRowAlignment` | Dawn | none | not reported | Skia pads each row of a copy between a buffer and a texture to 256 bytes. |
| `FramebufferFetch` | Dawn | none | not reported | Advanced blend modes read the destination through a texture copy. |
| `YCbCrVulkanSamplers` | Dawn | none | not reported | Skia cannot sample Vulkan YCbCr formats, such as video frames. |

Four features that the shim does not report have a `wgpu` counterpart: `Float32Blendable`,
`Unorm16TextureFormats`, `TransientAttachments`, and `DawnAllowUndefinedLoadStoreOp`. The shim
can report them once it converts the corresponding values, the formats, the texture usage, and the
load operation. `BufferMapExtendedUsages` has a counterpart only with a trade-off, because mapping
these buffers helps only on GPUs that share memory with the CPU.

`TimestampQuery` is reported, but one of Skia's uses is not covered yet. Unless `wgpu` runs on
Metal, Skia writes its timestamps on the command encoder, outside any pass
(`DawnCaps.cpp`, `fSupportsCommandBufferTimestamps`). `wgpu` allows such writes only with
`TIMESTAMP_QUERY_INSIDE_ENCODERS`. When Skia collects GPU timings on a device that has
`TIMESTAMP_QUERY` without that feature, the write fails `wgpu`'s validation.

## Crates and `wgpu` versions

The shim lives in its own crate, `skia-wgpu`, which depends on `skia-bindings` and `wgpu`.
`skia-safe` depends on `skia-wgpu`, so a `wgpu` backend is chosen as a `skia-safe` feature, like
`metal` or `d3d`. `skia-safe`'s `d3d` feature already sets the precedent of a public API that uses
a GPU API crate's types (`skia-safe/src/gpu/d3d.rs` re-exports types of the `windows` crate).

An application can only hand its device to Skia if both use the same `wgpu` major version, and
frameworks pin different majors. `skia-wgpu` therefore offers a feature `wgpu-NN` for each
supported `wgpu` major version. Each feature enables an optional dependency renamed to `wgpuNN`
(`wgpu30 = { package = "wgpu", version = "30" }`) and the module `vNN`. Inside `vNN`, the name
`wgpu` refers to that version's crate, and `vNN` compiles the one shim source. The few places
where two `wgpu` majors differ go into a per-version `compat` module, so the shared source never
names a version.

A build enables exactly one `wgpu-NN` feature. Each enabled version would export the same
`wgpu*` symbols, so `skia-wgpu` stops the build with a `compile_error!` when more than one is
enabled. For the same reason, `skia-wgpu` declares a `links` key. Cargo then allows only one
version of `skia-wgpu` in a dependency graph, and two semver-incompatible versions cannot both
export the symbols. Because only one is enabled, the crate root re-exports the API of its `vNN`
module, which stays private. Application code then names the `wgpu` version only in its
`Cargo.toml`.

The two sides of the shim change at different rates. Dawn's headers change once per Skia
milestone, and a rust-skia release has exactly one milestone, so every `vNN` module of a release
converts from the same generated C types. A Dawn update is therefore made once, in the shared
source. Only the `wgpu` side differs between `vNN` modules.

`skia-wgpu` sits below `skia-safe`, so its API cannot return `skia-safe` types. The `vNN` module
turns `wgpu` objects into version-independent handle types of `skia-wgpu`, for example
`skia_wgpu::Device`, which owns one reference to a `WGPUDevice`. `skia-safe` takes only these
handle types, so its Graphite API exists once, whatever `wgpu` version is enabled. `skia-safe`
forwards each `wgpu-NN` feature to `skia-wgpu`, so an application adds only `skia-safe`.
`skia-bindings` has one `wgpu` feature, which builds the Dawn backend and does not depend on the
`wgpu` version.

## Consequences

- The shim implements 74 methods and the `AddRef` and `Release` functions of 19 object types, 106
  functions in total at m157 (the undefined `wgpu*` symbols of `libskia-bindings.a`). The
  functions change with each Skia milestone, and the link of a test binary names every function
  that a new milestone adds. `cargo check` does not report a missing function.
- Skia's SkSL compiler emits WGSL that Dawn's Tint compiler accepts. `wgpu` compiles WGSL with naga
  instead, and it is not yet verified that naga accepts Skia's output.
- Everything the shim does passes through `wgpu`, so `wgpu` tracks resource state and
  synchronization between Skia's work and the application's work. `wgpu` validation adds CPU
  overhead compared with Graphite's native backends.
- One `wgpu` version drives Skia per build. If two crates in one dependency graph enable different
  `wgpu-NN` features of `skia-safe`, Cargo enables both, and the build fails.
- The `wgpu*` symbols are global. An application that also links another WebGPU implementation
  that exports them, such as Dawn or wgpu-native, fails to link with duplicate symbols.
- The link experiment did not cover Windows with MSVC's `link.exe`. Whether rustc's `symbols.o`
  also loads the shim's functions there is unverified.
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
- **Rejected: forward the `wgpu*` functions through Dawn's function table.** Dawn's generated
  `dawn_proc.cpp` defines every `wgpu*` function as a call through a `DawnProcTable`, a struct
  with one function pointer per WebGPU function, and `dawnProcSetProcs` installs the table for the
  whole process. Upstream Skia uses it to install real Dawn's table
  (`tools/graphite/dawn/GraphiteDawnTestContext.cpp`). A table of shim functions would need no
  symbols across crates. But the table has 276 entries, and a struct literal that names every
  entry, the only way to have rustc report a new function, needs a stub for each of the 170
  functions that Skia never calls. The table also needs `dawn_proc.cpp` and its replacement
  headers for Dawn internals in the build, an install step before Skia's first WebGPU call, and a
  check that only one `wgpu` version's table is installed.
- **Rejected: share `wgpu`'s native handles with Graphite's Metal and Vulkan backends.** `wgpu`
  exposes its `MTLDevice`, `MTLCommandQueue`, `VkDevice`, and textures through `as_hal`. This needs
  far less code than the shim and runs at native speed, but it cannot reach Direct3D 12 or
  OpenGL ES. On Vulkan, Skia and `wgpu` would also have to hand image layouts and queue access to
  each other explicitly.
- **Rejected: use wgpu-native as the `webgpu.h` implementation.** wgpu-native implements the
  standard header on `wgpu-core` with its own instance. It does not create its handles from `wgpu`
  crate objects, so an application's `wgpu::Device` cannot be handed to Skia.
- **Rejected: export the `wgpu*` functions from a Rust crate below `skia-bindings`.** That crate
  could not use the WebGPU types of `skia-bindings`, so a second bindgen run or a shared `-sys`
  crate would be needed. Link order gives no reason to place the shim below, because rustc's
  `symbols.o` loads the shim's functions from a crate above `skia-bindings` too.
- **Rejected: dispatch each `wgpu*` function through its handle, so that several `wgpu` majors
  can drive Skia in one build.** Every function that Skia calls takes a handle as its first
  argument. Each handle could start with a pointer to its version's operations, and one set of
  exports would dispatch through it. This needs an operations struct per version and an
  indirection in every export. One version per build is simpler, and the first release supports
  only one version.
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
