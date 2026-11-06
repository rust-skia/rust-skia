---
status: proposed
---

# Skia runs on the `wgpu` crate through a WebGPU shim

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

The crate `skia-wgpu` implements the `wgpu*` functions that the backend calls, in Rust. Each of
these functions converts its C arguments and calls the `wgpu` crate. That set of functions is
called the shim. A `WGPUDevice` handle wraps a clone of the application's `wgpu::Device`, so Skia
and the application share one device. ADR 0004 decides how the backend's calls reach the shim.

`skia-bindings` generates the WebGPU types in its existing bindgen run, and the shim uses them.
The shim implements only the functions that the backend calls, 106 at m157, of the 276 functions
in `webgpu.h`. It also implements the `AddRef` functions of six object types that the backend only
releases, so every object type has both.

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
only the generator, `src/dawn/dawn.json`, a few static headers, and the license. The fork's
`DEPS` does not sync Jinja2 and MarkupSafe either, the Python packages that the generator imports,
so the build downloads them from Chromium's Gitiles at the revisions in
`[package.metadata].jinja2` and `[package.metadata].markupsafe`. Offline builds therefore do not
support the `wgpu` feature. Builds without the `wgpu` feature run none of this, and
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

`skia-wgpu/FEATURES.md` records the decision for each feature and its reason. The list of features
that Skia queries changes with every Skia milestone, so it is kept apart from this ADR.

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

A `wgpu` version can also deviate from WebGPU in ways that Skia runs into, such as rejecting a call
that WebGPU allows. Its `compat` module then makes up for the deviation, or it makes the shim report
a feature less. The notes of the version, `src/vNN/README.md` in `skia-wgpu`, record each deviation
with its workaround and its upstream fix, so the workaround can go once no supported version needs
it.

A build enables exactly one `wgpu-NN` feature, because Skia's backend calls one set of `wgpu*`
functions in the whole process. ADR 0004 describes how the build enforces this. Because only one
is enabled, the crate root re-exports the API of its `vNN` module, which stays private.
Application code then names the `wgpu` version only in its `Cargo.toml`.

The two sides of the shim change at different rates. Dawn's headers change once per Skia
milestone, and a rust-skia release has exactly one milestone, so every `vNN` module of a release
converts from the same generated C types. A Dawn update is therefore made once, in the shared
source. Only the `wgpu` side differs between `vNN` modules.

`skia-wgpu` sits below `skia-safe`, so its API cannot return `skia-safe` types. It turns `wgpu`
objects into WebGPU handles, and `skia-safe` builds Graphite contexts and surfaces from these
handles. `skia-safe`'s module `gpu::graphite::wgpu` takes the application's `wgpu::Device`,
`wgpu::Queue`, and `wgpu::Texture`. These are types of the enabled version, which `skia-wgpu`
re-exports as `skia_wgpu::wgpu`. A build enables exactly one version, so the module exists once,
and its code never names a version. The module depends only on `skia-safe`'s hidden feature
`_wgpu`, which every `wgpu-NN` feature enables, so a new `wgpu` version changes no code in
`skia-safe`. `skia-safe` forwards each `wgpu-NN` feature to `skia-wgpu`, so an application adds
only `skia-safe`.
`skia-safe`'s feature `wgpu` enables one of them, the version that a release recommends, so an
application that takes its `wgpu` crate from `skia_safe::wgpu` does not name a version.
`skia-bindings` has one `wgpu` feature, which builds the Dawn backend and does not depend on the
`wgpu` version. One prebuilt binary per feature set therefore serves every `wgpu` version.

## Consequences

- The shim implements 74 methods and the `AddRef` and `Release` functions of 19 object types, 112
  functions in total at m157, of which the backend calls 106. The functions change with each Skia milestone.
- Skia's SkSL compiler emits WGSL that Dawn's Tint compiler accepts. `wgpu` compiles WGSL with naga
  instead, and it is not yet verified that naga accepts Skia's output.
- Everything the shim does passes through `wgpu`, so `wgpu` tracks resource state and
  synchronization between Skia's work and the application's work. `wgpu` validation adds CPU
  overhead compared with Graphite's native backends.
- One `wgpu` version drives Skia per build. If two crates in one dependency graph enable different
  `wgpu-NN` features of `skia-safe`, Cargo enables both, and the build fails (ADR 0004).
- Adding a `wgpu` major version adds an optional feature, which is compatible under semver. It
  ships as a patch release of `skia-wgpu` and `skia-safe` within the current milestone. Removing a
  version breaks users, so it only happens with a milestone version bump. The first release
  supports only the newest `wgpu` major version.
- Moving `skia-safe`'s feature `wgpu` to another `wgpu-NN` breaks applications that use their own
  `wgpu` dependency of the previous version, so it also only happens with a milestone version bump.
- When Skia releases its last device handle, the shim polls the device until all submitted work
  is done. A buffer map that is still pending would otherwise keep itself alive: `wgpu` holds the
  map's closure until a poll completes the map, the closure holds Skia's callback, and the callback
  holds Skia's buffer, which holds the `wgpu` buffer. The poll fires Skia's map callbacks while
  Skia destroys its shared context. This is safe, because maps are Skia's only callbacks with
  `AllowSpontaneous`, and a map callback uses only its buffer, whose resource cache the buffer
  keeps alive (`DawnBuffer.cpp`, `Resource.cpp`). The release of the instance also fires every
  callback that has not fired yet with `CallbackCancelled`, whatever its mode, during the same
  teardown. This is safe as well: Skia waits for its error-scope, shader-compilation, and pipeline
  callbacks before their state goes away (`DawnErrorChecker.cpp`, `DawnGraphiteUtils.cpp`, and
  `DawnGraphicsPipeline::freeGpuData`), and its work-done callback is empty
  (`DawnQueueManager.cpp`). Nothing in the build checks this, so the milestone update re-reads
  every callback that Skia registers at each tag cut. The poll also waits for the application's
  work on the device. The test
  `releasing_the_device_frees_a_buffer_with_a_pending_map` in `skia-wgpu` pins this. On WebGL, the poll does not wait at all, so a map whose work is still running stays
  pending, and its buffers leak.
- Every blocking poll of the shim, in `wgpuInstanceWaitAny` and when Skia releases its last device
  handle, waits until all submitted work is done. `wgpu`'s GL backend gives up a wait after about
  2 seconds, so the shim starts the wait again. On WebGL, a wait gives up at once, and
  `wgpuInstanceWaitAny` returns `WGPUWaitStatus_Error` for futures that cannot complete.
- A lost device aborts the process. `wgpu` treats the loss as fatal: its poll panics instead of
  returning an error, and the panic cannot unwind out of the WebGPU function that polls. An
  application that uses `wgpu` directly fails the same way, unless it catches the panic around
  its own calls. Catching it in the shim would also catch every other panic that the poll's
  callbacks raise, because `wgpu` fires them inside the poll. A device lost after a driver reset,
  a driver update, or, on Linux with NVIDIA's driver, a system suspend, therefore ends the
  application.
- The `wgpu` backend is experimental. Until it stabilizes, no prebuilt binaries contain it, so the
  `wgpu` feature builds Skia from source.

## Considered Options

- **Rejected: link Dawn, as upstream Skia does.** A `wgpu` application would run two WebGPU
  implementations that cannot share a device or a texture.
- **Rejected: keep Dawn in `DEPS` and build it, only to use Skia's `skia_use_dawn` unchanged.**
  `tools/git-sync-deps` syncs every `DEPS` entry, so every source build would fetch Dawn (about
  530 MB on disk), with or without the `wgpu` feature. Skia builds Dawn and Tint with CMake, which
  rust-skia's build cannot require.
- **Rejected: share `wgpu`'s native handles with Graphite's Metal and Vulkan backends.** `wgpu`
  exposes its `MTLDevice`, `MTLCommandQueue`, `VkDevice`, and textures through `as_hal`. This needs
  far less code than the shim and runs at native speed, but it cannot reach Direct3D 12 or
  OpenGL ES. On Vulkan, Skia and `wgpu` would also have to hand image layouts and queue access to
  each other explicitly.
- **Rejected: use wgpu-native as the `webgpu.h` implementation.** wgpu-native implements the
  standard header on `wgpu-core` with its own instance. It does not create its handles from `wgpu`
  crate objects, so an application's `wgpu::Device` cannot be handed to Skia.
- **Rejected: dispatch each `wgpu*` function through its handle, so that several `wgpu` majors
  can drive Skia in one build.** Every function that Skia calls takes a handle as its first
  argument. Each handle could start with a pointer to its version's operations, and one set of
  `wgpu*` functions would dispatch through it. This needs an operations struct per version and an
  indirection in every function. One version per build is simpler, and the first release supports
  only one version.
- **Rejected: compile Skia's Emscripten code path natively against the standard header.** The
  shim would not need Dawn-only functions, but the fork would carry a patch across the 74
  `__EMSCRIPTEN__` sites.
- **Rejected: one `wgpu` feature that adapts to the application's `wgpu` version.** A dependency
  range across majors (`wgpu = ">=29, <31"`) lets Cargo pick a version that differs from the
  application's, and the shim would have to compile against whichever version Cargo picks. A
  `wgpu-NN` feature states the version explicitly.
- **Rejected: a `skia-safe` API that takes version-independent handle types of `skia-wgpu`.**
  Applications would convert their `wgpu` objects into handles first, for example into a
  `skia_wgpu::Device`. This pays off only if several `wgpu` versions drive Skia in one build,
  which this ADR rejects.
- **Rejected: `skia-wgpu` depending on `skia-safe`.** `skia-safe` would stay free of `wgpu`, but
  applications would have to add a second crate, and `wgpu` would be the only GPU backend that is
  not a `skia-safe` feature.
