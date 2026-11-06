# Which WebGPU features the shim reports

Skia asks `wgpuDeviceHasFeature` which optional WebGPU features the device has, and it takes a
fallback path for each feature that is missing. The shim answers from the features of the
application's `wgpu::Device`, so the application decides which of them Skia can use
([ADR 0003](https://github.com/rust-skia/rust-skia/blob/master/docs/adr/0003-skia-runs-on-wgpu-through-a-webgpu-shim.md)).
This file records the decision for each feature, and the reason behind it.

The table describes the shim apart from any one `wgpu` version. Where a `wgpu` version deviates
from WebGPU in a way that makes the shim report less, the notes of that version say so, such as
[`src/v30/README.md`](src/v30/README.md) for `wgpu` 30.

The table lists every feature that Skia's Dawn backend queries at m157, in
`src/gpu/graphite/dawn/DawnCaps.cpp` and in the format table of `DawnGraphiteUtils.cpp`. A
standard feature is part of the WebGPU specification. A Dawn feature is an extension that only
Dawn defines. A reported feature is one for which the shim answers `true` when the device has the
`wgpu` counterpart in the third column.

| Feature | Kind | `wgpu` counterpart | Status | What Skia does without it |
|---|---|---|---|---|
| `CoreFeaturesAndLimits` | standard | none needed | always reported | Skia limits itself to the formats of WebGPU's compatibility mode. `wgpu` devices are never in that mode. |
| `TimestampQuery` | standard | `TIMESTAMP_QUERY`, and `TIMESTAMP_QUERY_INSIDE_ENCODERS` unless on Metal | reported | Skia reports no GPU timings. |
| `ShaderF16` | standard | `SHADER_F16` | reported | Shaders use 32-bit floats where Skia would use half precision. |
| `DualSourceBlending` | standard | `DUAL_SOURCE_BLENDING` | reported | Some blend modes that combine coverage with the destination read the destination through a texture copy. |
| `TextureCompressionBC`, `TextureCompressionETC2` | standard | `TEXTURE_COMPRESSION_BC`, `TEXTURE_COMPRESSION_ETC2` | reported | Skia cannot sample these compressed formats. |
| `RG11B10UfloatRenderable` | standard | `RG11B10UFLOAT_RENDERABLE` | reported | Skia cannot render into `RG11B10Ufloat`. |
| `BGRA8UnormStorage` | standard | `BGRA8UNORM_STORAGE` | reported | Skia cannot use `BGRA8Unorm` as a storage texture. |
| `Float32Filterable` | standard | `FLOAT32_FILTERABLE` | reported | Skia cannot filter 32-bit float textures. |
| `Float32Blendable` | standard | `FLOAT32_BLENDABLE` | reported | Skia cannot blend into 32-bit float render targets. |
| `TextureFormatsTier1`, `TextureFormatsTier2` | standard | none | not reported | Skia lacks the render, multisample, and storage capabilities that the tiers add to some formats. |
| `Unorm16TextureFormats` | Dawn | `TEXTURE_FORMAT_16BIT_NORM` | not reported | Skia cannot use the 16-bit normalized formats. |
| `TransientAttachments` | Dawn | `TextureUsages::TRANSIENT_ATTACHMENT` | not reported | MSAA attachments have backing memory, which costs memory and bandwidth on Apple and other tile-based GPUs. |
| `MSAARenderToSingleSampled` | Dawn | none | not reported | Skia renders into a separate MSAA texture and resolves it into the target. |
| `DawnLoadResolveTexture`, `DawnPartialLoadResolveTexture` | Dawn | none | not reported | Skia loads the target into the MSAA attachment and resolves it in separate render passes. |
| `RenderPassRenderArea` | Dawn | none | not reported | A render pass cannot be limited to the area that Skia draws into. |
| `DawnAllowUndefinedLoadStoreOp` | Dawn | `LoadOp::DontCare`, behind an unsafe token | not reported | Skia clears an attachment whose previous contents it discards, instead of leaving them undefined. |
| `BufferMapExtendedUsages` | Dawn | `MAPPABLE_PRIMARY_BUFFERS`, which `wgpu` warns is slow on GPUs with their own memory | reported | Skia cannot map vertex, index, and uniform buffers, and uploads their data through transfer buffers. |
| `DawnTexelCopyBufferRowAlignment` | Dawn | none | not reported | Skia pads each row of a copy between a buffer and a texture to 256 bytes. |
| `FramebufferFetch` | Dawn | none | not reported | Advanced blend modes read the destination through a texture copy. |
| `YCbCrVulkanSamplers` | Dawn | none | not reported | Skia cannot sample Vulkan YCbCr formats, such as video frames. |

`BufferMapExtendedUsages` lets Skia map the buffers that the GPU reads directly. That helps only on
GPUs that share memory with the CPU, and `wgpu` warns that it is slow on GPUs with their own
memory. The shim reports it when the application requested `MAPPABLE_PRIMARY_BUFFERS` for its
device, so the application makes that trade-off, as it does for every reported feature.

Three features that the shim does not report have a `wgpu` counterpart, but reporting them would
let Skia make calls that `wgpu` rejects or that break `wgpu`'s safety contract:

- `TransientAttachments`: Skia makes its MSAA attachments transient when the device reports this
  feature, and it then expects to load the target into them with `DawnLoadResolveTexture`. Without
  that feature, Skia emulates the load and stores into the MSAA attachment between passes
  (`emulateLoadMSAAFromResolveAndBeginRenderPassEncoder` in `DawnCommandBuffer.cpp`). `wgpu`
  allows a transient attachment only with a clearing load and a discarding store, so the shim can
  report `TransientAttachments` only together with a counterpart of `DawnLoadResolveTexture`, which
  `wgpu` lacks.
- `Unorm16TextureFormats`: Skia then renders into, blends, multisamples, and resolves the 16-bit
  normalized formats (`DawnGraphiteUtils.cpp`). `TEXTURE_FORMAT_16BIT_NORM` guarantees only
  sampling, multisampling, and storage for them. The other capabilities depend on the adapter and
  need `TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`, which the shim cannot check, because it holds
  the device and not the adapter.
- `DawnAllowUndefinedLoadStoreOp`: Skia then uses undefined load and store operations wherever it
  discards an attachment's contents. The counterpart, `LoadOp::DontCare`, is unsafe in `wgpu`: the
  contents are undefined behavior unless every pixel is written before it is read, blended, or
  stored. The shim cannot verify that Skia's discarding passes write every pixel, and `wgpu` has
  no device feature through which the application could accept the risk.

`Unorm16TextureFormats` and `DawnAllowUndefinedLoadStoreOp` stay unreported, because each would
cost more than it gains:

- `Unorm16TextureFormats` is rarely needed. Skia uses the 16-bit normalized formats only for the
  color types `A16_unorm`, `R16G16_unorm`, and `R16G16B16A16_unorm`, which an application chooses
  explicitly (`TextureFormat.cpp`). Skia's own paths prefer `F16`: the PNG codec usually decodes
  16-bit images to it, and `SkYUVAPixmaps` picks it for 16-bit video planes, because it has better
  GPU support. Without the feature, an application converts images of these color types to `F16`
  before Skia uploads them. Reporting the feature needs the application's `wgpu::Adapter` in the
  API of `skia-wgpu`, so that the shim can check the adapter's capabilities for the three formats.
  The recommendation is to add that only when an application needs these color types on the GPU.
- `DawnAllowUndefinedLoadStoreOp` gains little. It replaces a clear with an undefined load in
  opaque layers, in explicit discards, and in the MSAA attachment of a target that is cleared or
  discarded. When the target is loaded, Skia's load emulation loads the MSAA attachment anyway.
  Tile-based GPUs clear for free when a tile starts, and most desktop GPUs clear through metadata
  only. Skia also does not uphold `wgpu`'s contract. An opaque layer starts out undefined, and Skia
  composites the whole layer although the application may have drawn into only part of it.
  Reporting the feature needs an unsafe opt-in in the API of `skia-wgpu`, through which the
  application accepts the contract, and a conversion of the load and store operations that tells an
  unused depth or stencil aspect apart from a discarded one by the attachment's format. The
  recommendation is to leave it unreported.

Skia writes its timestamps in one of two ways, chosen by the backend (`DawnCaps.cpp`,
`fSupportsCommandBufferTimestamps`). Unless `wgpu` runs on Metal, Skia writes them on the command
encoder, outside any pass, which `wgpu` allows only with `TIMESTAMP_QUERY_INSIDE_ENCODERS` in
addition to `TIMESTAMP_QUERY`. On Metal, Skia instead asks render passes to write timestamps at
their beginning and end (`timestampWrites`), which `TIMESTAMP_QUERY` alone covers. The shim
therefore reports `TimestampQuery` on Metal for a device with `TIMESTAMP_QUERY`, and on every other
backend only for a device with both features. The test
`timestamp_query_alone_is_reported_only_on_metal` in `skia-wgpu` pins this rule.
