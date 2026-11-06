# Deviations of `wgpu` 30

This file lists where `wgpu` 30 deviates from WebGPU in ways that Skia runs into, and how the
module `compat` next to it makes up for each. Remove a section, together with its `compat` item,
once the minimum `wgpu` 30 release that the shim requires contains the upstream fix. The whole
folder goes when the shim drops `wgpu` 30.

## Immediates are lost before the first pipeline and on pipeline changes

**WebGPU.** A render pass accepts immediates before its first pipeline and keeps them across
pipeline changes. Skia's Dawn backend relies on both. It passes its intrinsic uniforms, 32 bytes
with the viewport and the bounds of a destination read, as immediates when the device's
`maxImmediateSize` has room for them (`DawnCaps.cpp`). A device has a non-zero
`max_immediate_size` only with `IMMEDIATES`. Skia sets the immediates once per render pass, right
after the pass begins and before its first pipeline (`DawnCommandBuffer::onAddRenderPass`).

**`wgpu` 30.** A render pass rejects immediates before the first pipeline ("Render pipeline must
be set"). It also zeroes them whenever the pipeline layout changes (`change_pipeline_layout` in
`wgpu-core/src/command/pass.rs`). Every Skia pipeline has its own layout, so the immediates are
zeroed on every pipeline switch.

**Effect on Skia.** Without a workaround, the first call fails validation. Every draw after the
first pipeline switch in a pass would use a zeroed viewport and vanish without an error.

**Workaround.** `compat::RenderPass` wraps `wgpu`'s render pass and keeps the bytes that Skia set.
It passes them on once the pass has a pipeline, and it sets them again after each pipeline. The
section "Known issues" describes the case that this workaround does not cover.

**Upstream fix.** gfx-rs/wgpu#9597 makes `wgpu` keep the immediates itself and set them before each
draw, cut to the size of the pipeline's layout. It is merged, and no release contains it yet.

**Test.** `skia_renders_on_a_device_with_every_adapter_feature` in `skia-safe/tests/wgpu.rs` draws
two shapes that need different pipelines in one pass.

## `f16` fragment outputs are rejected for `f32` color targets

**WebGPU.** A fragment output must match its color target's scalar kind, float or integer, but not
its width. With `ShaderF16`, Skia writes `f16` fragment outputs.

**`wgpu` 30.** Pipeline creation rejects an `f16` output for a color target whose format has 32-bit
scalars, such as `BGRA8Unorm` (`check_texture_format` in `wgpu-core/src/validation.rs` compares the
scalar width).

**Effect on Skia.** Creating a render pipeline fails on a device with `SHADER_F16`.

**Workaround.** `compat::ACCEPTS_F16_FRAGMENT_OUTPUTS` is `false`, so the shim does not report
`ShaderF16`, and Skia uses 32-bit floats.

**Upstream fix.** gfx-rs/wgpu#9999 compares only the scalar kind. It is merged, and no release
contains it yet.

**Test.** `skia_renders_on_a_device_with_every_adapter_feature` in `skia-safe/tests/wgpu.rs`
requests every feature of the adapter.

## Known issues

### The immediates do not fit a smaller pipeline layout

`compat::RenderPass` sets all kept immediates again after each pipeline, starting at offset 0.
`wgpu` 30 checks them against the immediate size of the new pipeline's layout
(`validate_immediates_ranges` in `wgpu-core/src/binding_model.rs`). A pipeline whose layout has
less room than the kept bytes therefore fails validation, although WebGPU would accept the same
calls, because it checks only against the device's `maxImmediateSize`.

Skia does not run into this today. Every Skia pipeline that runs after the immediates reserves the
32 bytes (`DawnGraphicsPipeline.cpp`). Skia's blit pipelines, whose shaders use no immediates, run
only before the immediates, when they load the target into the MSAA attachment, or in a pass of
their own, when they resolve it (`DawnCommandBuffer.cpp`). A Skia change that sets such a pipeline
after the immediates in the same pass would fail validation on `wgpu` 30.

A fix would cut the resent bytes to the new layout's immediate size, as gfx-rs/wgpu#9597 does.
`wgpu::RenderPipeline` does not expose its layout, so the shim would have to record the size when
it creates a pipeline, and it cannot know the size of a layout that `wgpu` derives from the
shaders. With gfx-rs/wgpu#9597, `wgpu` cuts the bytes itself, and the issue goes away.
