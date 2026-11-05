---
status: proposed
---

# The shim exports the WebGPU C functions from Rust

ADR 0003 runs Skia's Graphite Dawn backend on the `wgpu` crate. The backend calls the C functions
of `webgpu.h`, which all start with `wgpu`, and the crate `skia-wgpu` implements them on `wgpu`.
That set of functions is called the shim. This ADR decides how the backend's calls reach the
shim.

## Decision

The shim defines the `wgpu*` functions as Rust functions with `#[no_mangle]` and the C calling
convention. The backend's calls reach the shim directly. No function table and no install step
sit between them.

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

`webgpu.h` declares a function pointer type for every function, such as
`WGPUProcDeviceCreateBuffer`, and `skia-bindings` generates these types with the other WebGPU
types. Next to each shim function, an assignment of the function to that type
(`const _: WGPUProcDeviceCreateBuffer = Some(wgpuDeviceCreateBuffer);`) makes rustc check its
signature against the header. A function that the backend calls and the shim lacks usually fails
the link with an undefined symbol that names the function. The linker reports it only if it keeps
the code of the backend that calls the function, so the list of undefined `wgpu*` symbols of
`libskia-bindings.a` is the authoritative check.

## Skia's calls stay inside the binary that contains them

On ELF targets, such as Linux, a shared library exports every function with default visibility,
and the dynamic linker may resolve the library's own calls of such a function to a definition in
another library that was loaded earlier. A shared library that contains Skia would export the
shim's `wgpu*` functions, and another WebGPU implementation in the process, or a second copy of
the shim, could take over Skia's calls with handles that it does not understand.

On ELF targets, `skia-bindings` therefore compiles its C++ sources, Skia's Dawn backend among
them, with `WGPU_FUNCTION_ATTRIBUTE`, the macro that `webgpu.h` appends to every function
declaration, set to `visibility("hidden")`. The linker gives a symbol the most restrictive
visibility of all its references, so the shim's definitions become hidden in the output, and
Skia's calls bind inside it. An experiment with a model of a Rust `cdylib` that contains a C
library calling an exported Rust function confirmed this on aarch64 Linux with GNU ld, in debug
and release builds, and with LLD 19 in a release build. Without the attribute, the library
exported the function, and with it, the function was local.

Windows and macOS do not need the attribute. A Rust `cdylib` exports the shim's functions there,
too, but the linker binds the library's own calls to its own definitions: a DLL calls its
functions directly, and macOS records the defining library with each import (two-level
namespace).

## One set of exports per build

ADR 0003 lets a build enable exactly one `wgpu-NN` feature of `skia-wgpu`. Each enabled version
would export the same `wgpu*` symbols. Once `skia-wgpu` supports a second `wgpu` major version, it
stops the build with a `compile_error!` when more than one is enabled. With a single version, no
build can enable two. For the same reason, `skia-wgpu` declares `links = "skia-wgpu"`, with
the empty build script that Cargo requires for the key. Cargo then allows only one version of
`skia-wgpu` in a dependency graph, and two semver-incompatible versions cannot both export the
symbols. Cargo rejects such a graph when it resolves the dependencies, before anything is built.
Without the check, the linker might load only one version's exports and silently send the other
version's handles to them.

`skia-bindings` declares `links = "skia"`, and `skia-wgpu` pins its exact version. As long as all
crates share one version per milestone, these two already keep a second `skia-wgpu` out. The key
on `skia-wgpu` keeps this guarantee on the crate that exports the symbols, so it also holds if
`skia-wgpu` ever gets a version line of its own.

## Consequences

- The undefined `wgpu*` symbols of `libskia-bindings.a` are the functions that the shim must
  implement, 106 at m157. The link of a test binary usually names a function that a new milestone
  adds, and `nm -u` lists all of them. `cargo check` does not report a missing function.
- If two crates in one dependency graph enable different `wgpu-NN` features of `skia-safe`, Cargo
  enables both, and the `compile_error!` that comes with the second supported version fails the
  build.
- The `wgpu*` symbols keep their `webgpu.h` names. An application that statically links another
  WebGPU implementation that defines them, such as Dawn or wgpu-native, fails to link with
  duplicate symbols. Shared libraries do not export them on ELF targets. If such an application
  needs rust-skia, a prefix on the symbol names of both sides, through a generated header of
  `#define`s for Skia's C++ sources and renamed exports in the shim, removes the clash without a
  change to the public API.
- The link experiment did not cover Windows with MSVC's `link.exe`. Windows QA on
  `x86_64-pc-windows-msvc` and `aarch64-pc-windows-msvc` linked and ran the test binaries of the
  `wgpu` feature with `link.exe`, so rustc's `symbols.o` loads the shim's functions there, too.

## Considered Options

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
- **Rejected: export the `wgpu*` functions from a Rust crate below `skia-bindings`.** That crate
  could not use the WebGPU types of `skia-bindings`, so a second bindgen run or a shared `-sys`
  crate would be needed. Link order gives no reason to place the shim below, because rustc's
  `symbols.o` loads the shim's functions from a crate above `skia-bindings` too.
