# Skia on `wgpu`: architecture

This document shows how Skia runs inside a `wgpu` application. The decisions behind it are recorded
in ADR 0003 (Skia runs on the `wgpu` crate through a shim behind Dawn's function table).

Terms used below:

- **Dawn backend**: Skia's Graphite backend for WebGPU. Skia names it after Dawn, Google's C++
  WebGPU implementation, but rust-skia links none of Dawn's implementation.
- **Dawn headers**: the C header `webgpu.h`, the C++ wrapper `webgpu_cpp.h`, and the `dawn_proc`
  files. Dawn's generator produces them during source builds with the `wgpu` feature, from the
  Dawn revision in `[package.metadata].dawn` of `skia-bindings`.
- **Function table**: Dawn's `DawnProcTable`, a C struct with one function pointer per WebGPU
  function. `dawnProcSetProcs` installs it for the whole process.
- **Shim**: the Rust code in `skia-wgpu` that fills the function table with functions that call
  `wgpu`.

## Crate dependencies

An arrow points from a crate to a crate it depends on. `skia-safe` depends on `skia-wgpu`, which
depends on `skia-bindings`.

```mermaid
flowchart TD
    app["Application<br/>(uses wgpu 30)"]
    safe["skia-safe<br/>feature wgpu-30"]
    shim["skia-wgpu<br/>feature wgpu-30<br/>(shim)"]
    bindings["skia-bindings<br/>feature wgpu"]
    libskia["libskia-bindings.a<br/>Dawn backend + dawn_proc<br/>libskia.a: Skia"]
    wgpu["wgpu 30"]

    app --> safe
    app --> wgpu
    safe --> shim
    safe --> bindings
    shim --> bindings
    shim --> wgpu
    bindings --> libskia
```

The application and `skia-wgpu` must resolve to the same `wgpu` major version, so that a
`wgpu::Device` of the application is the type the shim accepts.

## Build inputs

The generated Dawn headers feed both the compilation of Skia's Dawn backend and bindgen. Struct layouts, enum values,
and function signatures therefore come from one source on both sides of the function table.

```mermaid
flowchart LR
    dawn["Dawn archive at<br/>[package.metadata].dawn"]
    generator["Dawn generator"]
    headers["Generated Dawn headers<br/>webgpu.h, webgpu_cpp.h,<br/>dawn_proc.cpp, dawn_proc_table.h"]
    cc["cc build of skia-bindings<br/>Dawn backend + dawn_proc.cpp"]
    bindgen["bindgen run of<br/>skia-bindings"]
    lib["libskia-bindings.a"]
    rs["bindings.rs<br/>WGPU* types, DawnProcTable,<br/>dawnProcSetProcs"]
    shim["skia-wgpu shim"]

    dawn --> generator --> headers
    headers --> cc --> lib
    headers --> bindgen --> rs --> shim
```

## Path of one WebGPU call

When Skia creates a buffer, the call leaves Skia's C++ code through Dawn's C++ wrapper, passes the
function table inside `libskia-bindings.a`, and arrives in the shim, which calls `wgpu`.

```mermaid
sequenceDiagram
    participant Skia as Skia Dawn backend (C++)
    participant Cpp as webgpu_cpp.h (inline)
    participant Proc as dawn_proc.cpp
    participant Shim as skia-wgpu v30
    participant Wgpu as wgpu 30

    Skia->>Cpp: device.CreateBuffer(&desc)
    Cpp->>Proc: wgpuDeviceCreateBuffer(device, &desc)
    Proc->>Shim: procs.deviceCreateBuffer(device, &desc)
    Shim->>Shim: convert WGPUBufferDescriptor
    Shim->>Wgpu: Device::create_buffer(&descriptor)
    Wgpu-->>Shim: wgpu::Buffer
    Shim-->>Skia: WGPUBuffer (boxed wgpu::Buffer)
```

`wgpu` then runs the work on whichever native API it uses: Metal, Vulkan, Direct3D 12, or
OpenGL ES.

## Creating a Graphite context

The application passes its own `wgpu` objects to `skia-safe`. The shim installs its function table
once, before Skia makes its first WebGPU call.

```mermaid
sequenceDiagram
    participant App as Application
    participant Safe as skia-safe
    participant Shim as skia-wgpu v30
    participant Skia as Skia (libskia-bindings.a)

    App->>Safe: make Graphite context from wgpu::Device, wgpu::Queue
    Safe->>Shim: v30 handles from wgpu objects
    Shim->>Shim: refuse if another wgpu version's table is active
    Shim->>Skia: dawnProcSetProcs(&table) (first use only)
    Shim-->>Safe: skia_wgpu::Instance, Device, Queue (version independent)
    Safe->>Skia: ContextFactory::MakeDawn(DawnBackendContext)
    Skia-->>App: graphite::Context
```

## Several `wgpu` major versions

`skia-wgpu` compiles one shared shim source once per enabled `wgpu` major version. Each `vNN`
module binds the name `wgpu` to its version's crate, and its `compat` module holds the few places
where that version's API differs. All modules convert from the same generated C types, and all
produce the same version-independent handle types that `skia-safe` accepts.

```mermaid
flowchart TD
    src["Shared shim source<br/>shim/mod.rs"]
    types["Generated C types<br/>(skia-bindings)"]
    subgraph v29 ["module v29 (feature wgpu-29)"]
        c29["compat"]
        w29["wgpu = wgpu29"]
    end
    subgraph v30 ["module v30 (feature wgpu-30)"]
        c30["compat"]
        w30["wgpu = wgpu30"]
    end
    handles["Version-independent handles<br/>skia_wgpu::Instance, Device, Queue, Texture"]
    table["One process-wide function table"]

    types --> src
    src --> v29
    src --> v30
    v29 --> handles
    v30 --> handles
    v29 -. "installs (one version at a time)" .-> table
    v30 -. "installs (one version at a time)" .-> table
```

A new Skia milestone changes the generated C types, and the shared source is updated once. A new
`wgpu` major version adds a feature, a renamed dependency, and a `compat` module.
