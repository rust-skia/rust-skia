# rust-skia Docs Porting Tracker

Persistent tracker of documentation ported from Skia C++ headers into `skia-safe` Rust rustdoc and **verified for completeness** against the C++ header (rule: C++ doc ported COMPLETELY, no shortening; only allowed drops = fiddle example links, out-param/nullptr→Option, ownership sentences, name translation + intra-doc links).

**Milestone:** m154 (skia-bindings `skia = "m154.5"`; Skia submodule tag `m154.5`; crates `0.154.0`). The current state is described in the `# m154 update (2026-09-20)` section below; everything above it is the m153 doc corpus (verified against the m153 headers on 2026-09-08) and still applies to every header that did not change in m154.

## Verification method
- Independent subagent audit comparing EVERY public item doc in each Rust file vs its C++ header.
- Header read directly during review (per user requirement: "actually read the C++ header").
- `cargo fmt -p skia-safe` after edits; `cargo doc` run ONCE at end of batch (user rule).

## Status: ALL items in the 34-core-file audit FIXED (2026-09-08)

Committed on branch `docs` (in order):
- `c3865966` skia-safe: complete M44 documentation
- `0c610c9f` skia-safe: document remaining Pixmap methods
- `d21a7e25` skia-safe: document StrokeRec methods
- `80afd23f` skia-safe: complete ImageInfo and ColorInfo documentation
- `6401ed60` skia-safe: document TextBlob::from_str
- `8502a0fb` skia-safe: document Color4f, PMColor, and HSV methods
- `bff0c211` skia-safe: document RRect accessors and transforms
- `571b0532` skia-safe: document Region iterators
- `a64e56f0` skia-safe: document Matrix methods (largest, 824 lines)

## Units ported + completeness-verified (chronological)

### Core — 34-file audit (all confirmed complete or C++ had no docs)
- **region.rs** (SkRegion.h): impl Region + Iterator/Cliperator/Spanerator (type + all methods: new/new_empty/rewind/reset/is_done/next/rect/rgn).
- **text_blob.rs** (SkTextBlob.h): from_str + builder alloc_* (RunBuffer pos/utf8text/clusters "should be ignored" sentences dropped as Rust returns only relevant slices — legit adaptation).
- **pixmap.rs** (SkPixmap.h): scale_pixels, erase, erase_4f, read_pixels_to_pixmap + all accessors/converters.
- **image_generator.rs** (SkImageGenerator.h): all methods (unique_id/is_texture_generator have no C++ doc → OK).
- **drawable.rs** (SkDrawable.h): all methods + GpuDrawHandler class doc.
- **vertices.rs** (SkVertices.h): all (new_copy omits "texs/colors may be nullptr" — Rust API can't pass None, adaptation).
- **image_info.rs** (SkImageInfo.h): ColorInfo + ImageInfo type overviews, all factories/accessors, reset, is_opaque, is_gamma_close_to_srgb.
- **color_space.rs** (SkColorSpace.h): new_srgb/linear/new_icc/new_cicp/to_xyzd50_hash/with_*_gamma/with_color_spin/is_srgb/serialize (deserialize/transfer_fn/etc. have no C++ doc → OK).
- **graphics.rs** (SkGraphics.h): all functions.
- **m44.rs** (SkM44.h): new, rect_to_rect, to_m33 (asM33 diagram) + previously ported methods.
- **matrix.rs** (SkMatrix.h): ~60 methods — factories (scale/translate/rotate_deg/skew/new_all), getters/setters (rc/scale_x/y/skew_x/y/translate_x/y/persp_x/y + set_*), set_all/get_9/set_9/reset/set_identity/set_translate/set_scale/set_rotate/set_sin_cos/set_rsxform/set_skew/set_concat, all pre_*/post_* matrix-multiply diagrams, rect_2_rect/rect_to_rect_or_identity/poly_to_poly/invert/affine family/normalize_perspective, all map_* methods, min_scale/max_scale/min_max_scales/decompose_scale/i/invalid_matrix/concat/dirty_matrix_type_cache/set_scale_translate/is_finite/dump.
- **path_effect.rs** (SkPathEffect.h): all documented.
- **data_table.rs** (SkDataTable.h): documented (at_t/iter are rust helpers, excluded).
- **path_measure.rs** (SkPathMeasure.h): all documented (current_measure rust-specific; deprecated skipped).
- **picture_recorder.rs** (SkPictureRecorder.h): all documented (new() has no C++ ctor doc → OK).
- **pixel_ref.rs** (SkPixelRef.h): module doc + all documented methods (accessors have no C++ docs → OK).
- **point3.rs** (SkPoint3.h): all methods.
- **cubic_map.rs** (SkCubicMap.h): class doc only in C++ (methods have no C++ docs → OK).
- **rsxform.rs** (SkRSXform.h): from_radians + others (no C++ docs → OK).
- **size.rs** (SkSize.h): is_zero/is_empty/set_empty (others no C++ docs → OK).
- **color.rs** (SkColor.h): Color + Color4f (is_opaque/fits_in_bytes/to_color/to_opaque/pin_alpha/as_array/as_array_mut) + pre_multiply_argb/pre_multiply_color + RGB::to_hsv + HSV::to_color + constants.
- **font_metrics.rs** (SkFontMetrics.h): Flags/fields/methods all documented.
- **font_parameters.rs** (SkFontParameters.h): Axis fields + is_hidden/set_hidden (new has no C++ doc → OK).
- **font_types.rs** (SkFontTypes.h): TextEncoding variants (all 4 match C++ //!< verbatim).
- **blend_mode.rs** (SkBlendMode.h): module doc matches overview verbatim.
- **stroke_rec.rs** (SkStrokeRec.h): apply_to_paint, inflation_radius(+from_paint_and_style), has_equal_effect; set_stroke_style/need_to_apply/apply_to_path documented; getters/setters/new/from_paint no C++ docs → OK; inflation_radius_from_params no C++ prose → OK.
- **surface_props.rs** (SkSurfaceProps.h): new/new_with_text_properties (accessors no C++ docs → OK; misleading new() default-ctor doc removed).
- **annotation.rs** (SkAnnotation.h): rect_with_url/named_destination/link_to_destination + Canvas annotate_*.
- **swizzle.rs** (SkSwizzle.h): swap_rb/swap_rb_inplace.
- **coverage_mode.rs / alpha_type.rs / tile_mode.rs**: module docs match C++ enum/class overviews.
- **rrect.rs** (SkRRect.h): rect/radii/bounds/inset/outset/offset/with_* + contains_point/contains/is_valid/write_to_memory/read_from_memory/transform/dump; get_type/is_empty/is_rect/etc. no C++ docs → OK.
- **point.rs** (SkPoint.h + SkIPoint): all IPoint + Point public methods fully documented (Ops trait impls not counted).

### Earlier verified units (from doc-porting-plan.md)
- **paint.rs + font.rs** (SkPaint.h/SkFont.h): commit 313b826a (+434 lines), 1c795118 skill alignment. rustdoc clean for both (13 links via crate:: paths). SKIPPED (not in milestone header): SkPaint setFill/getFillStyle/getStrokeStyle, SkFont countStreamFonts.
- **gpu/ganesh** (commit a7a8fb9a, +460/−2, 7 files): GrDirectContext.h (~25 methods), GrRecordingContext.h (7), GrTypes.h (BackendApi/GrSurfaceOrigin/GrFlushInfo/GrSubmitInfo/SemaphoresSubmitted), GrYUVABackendTextures accessors, ContextOptions struct, BackendSurface (sparse C++ // one-liners), `//!` on ganesh.rs. SKIPPED: driver_bug_workarounds.rs (C++ zero docs), gl/vk/d3d/mtl subdirs, TODO-commented gr_context wrappers.
- **effects/** (commit 52a639c8, +257, 13 files): runtime_effect.rs (39 items), image_filters.rs (10 leftovers), 5 path-effect files (corner/dash/discrete/trim/_1d), color-filter batch (high_contrast/overdraw/luma), perlin noise, blenders::Blender::arithmetic. SKIPPED (C++ zero docs): _2d_path_effect.rs, color_matrix.rs; skipped deprecated: color_matrix_filter/shader_mask_filter/table_color_filter/table_mask_filter/gradient_shader; gradient.rs leftovers (3 accessors, no C++ docs).
- **Doc-convention normalization** (commit b2af801d, +188/−309, 16 files): 122 fiddle links removed, 167 colon-bullets fixed, nullptr→None. DEFERRED (judgment call): Class 3b SkRefCnt wording (18 lines paint/font).

## Backlog progress (survey 2026-09-08 → continuing, milestone m153)
Working the survey top-to-bottom (priority list removed per user request). Committed on `docs`:
- `110103b1` skia-safe: document SkCodec (`codec/_codec.rs` — full SkCodec.h: Result/SelectionPolicy/ZeroInitialized/ScanlineOrder/IsAnimated enum docs, Options + FrameInfo field docs, all methods incl. from_stream/from_data/get_pixels*/start_*_decode/scanlines/frames/is_animated; codecs::Decoder + deferred_image).
- `698fc77c` skia-safe: document codec animation, decoders, and encoded formats (`codec_animation.rs` Blend/DisposalMethod variant docs; `decoders.rs` all 8 decoder modules; `encoded_origin.rs` to_matrix_inverse; `encoded_image_format.rs` enum overview — C++ has NO per-variant comments, so no Variants list).
- `0ddc3258` skia-safe: document FontMgr match and fallback APIs (`core/font_mgr.rs` — module doc, Request.bcp_47 field, CMapEntry.variation, match_family/match_family_style/match_family_style_character/match_request/fallback/new_from_data/new_from_bytes/empty).
- `a2119ba5` skia-safe: document image encoders (`encode_.rs` module + Pixmap/Bitmap/Image::encode + Comment; `png_encoder.rs` Options fields + encode/encode_pixmap/encode_image; `jpeg_encoder.rs` AlphaOption/Downsample + Options fields + encode*/encode_image; `webp_encoder.rs` Compression + Options fields; `png_rust_encoder.rs` CompressionLevel + Options.comments).
- `f21a3f3a` skia-safe: document path operations (`pathops.rs` — module doc, PathOp variant docs, op/simplify/tight_bounds/as_winding, OpBuilder class + add/resolve).
- `f353638d` skia-safe: document gpu type enums (`gpu/types.rs` — BackendApi/Budgeted/Mipmapped/Protected/Origin; all have C++ doc comments). GpuStats/GpuStatsFlags have NO C++ docs → left undocumented.
- `8e4edaf2` → REVERTED as `b905bb54`: had invented rustdoc for `paragraph.rs` accessors/state getters that have NO C++ doc comments in `Paragraph.h`. Correction: **never invent docs when C++ has none** — port only existing C++ docs (rule reinforced by user).
- `8e8a8da0` skia-safe: normalize png_rust_encoder import order (pre-existing pending cleanup).
- `037a64f3` skia-safe: document shadow utils (`utils/shadow_utils.rs` / `SkShadowUtils.h` — module `//!`, ShadowFlags variant docs, draw_shadow/local_bounds/compute_tonal_colors; `None`/`Some` for out-param bounds).
- `60222c0f` skia-safe: document tiled image utils (`core/tiled_image_utils.rs` / `SkTiledImageUtils.h` — namespace `//!` + get_image_key_values; draw_image_rect/draw_image/NUM_IMAGE_KEY_VALUES have no C++ doc → left undoc).
- `84bc1dc3` skia-safe: document shaper run handler and drop invented current_font doc (`modules/shaper.rs` / `SkShaper.h` — ported `SkTextBlobBuilderRunHandler` class doc; removed invented `current_font` doc since C++ `currentFont()` has none; all other documented items already ported).

> **CORRECTION (2026-09-08):** The graphite comments I removed in `391d20f7`, `58e6e82a`, and `536a020f` were **added by a contributor**, not invented. They are legitimate and were reverted (`def6f267`, `39722653`, `4a945dcd`) to restore the contributor's comments intact. `gpu/graphite/context_options.rs`, `context.rs`, and `recorder.rs` are back to their contributor-documented state — do NOT remove those comments.
- `1f1e64d2` skia-safe: document SVG DOM root and set_container_size (`modules/svg/dom.rs` / `SkSVGDOM.h` — ported `getRoot` "Returns the root (outermost) SVG element." and `setContainerSize` full prose; read/from_str/from_bytes/render C++-undoc → left undoc; fe.rs SkSVGFe.h doc'd methods are virtual/unwrapped → nothing to port).
- `03a8aa1e` skia-safe: document Vk DrawableInfo (`gpu/ganesh/vk/vk_types.rs` / `GrVkTypes.h` — ported GrVkDrawableInfo struct overview).
- `ec3169ca` skia-safe: document GL Standard, Format, and TextureInfo (`gpu/ganesh/gl/types.rs` / `GrGLTypes.h` — GrGLStandard + GrGLFormat enum overviews + GrGLTextureInfo struct doc).
- `e740935b` skia-safe: document MutableTextureState (`gpu/mutable_texture_state.rs` / `MutableTextureState.h` — class overview; methods C++-undoc → left undoc).
- `09f64c1f` skia-safe: fix intra-doc links in ported gpu/utils docs and complete set_font_arguments (added missed links in vk_types DrawableInfo, tiled_image_utils module/key-values docs, gl Format doc; completed `TextStyle::set_font_arguments` missing sentence).
- `356b4749` skia-safe: port remaining gpu and utils documentation (graphite `TextureInfo` module overview; ganesh GL `Interface` class docs + `new_native()`; GL `Extensions` class + `has`/`remove`/`add`; GL `BackendState` enum overview + TEXTURE_BINDING/VIEW flag docs from `GrTypes.h`; Vulkan `ImageInfo` wrapping constraints from `GrVkTypes.h`; `VulkanAlloc` struct doc; null canvas fn docs; `OrderedFontMgr` module overview).
- `cb542106` skia-safe: port backend direct-context and typeface documentation (gl/vk/mtl/d3d `make_ganesh` one-liners from `SkContexts`; gl `make_gl` + vk `make_vulkan` docs from `GrDirectContexts`; `Typeface::unique_id`).
- `84d6d6a1` skia-safe: close remaining backlog — d3d `TextureResourceInfo` note + `FenceInfo.value` field comment (`GrD3DTypes.h`); graphite `Context::delete_backend_texture` + `is_device_lost` full C++ docs (`Context.h`); mtl `TextureInfo` struct doc (`GrMtlTypes.h`); gl `new_load_with`/`new_load_with_cstr` docs (`GrGLAssembleInterface.h`); svg `use.rs` module doc (`SkSVGUse.h` class overview).

## Final verification sweep (2026-09-08, closes the survey)
Confirmed NOTHING to port (C++ has no docs on wrapped items, or API not wrapped):
- `core/recorder.rs` / `core/cpu_recorder.rs`: SkRecorder.h documents only a PRIVATE `makeCaptureCanvas`; SkCPURecorder.h documents `TODO()` and `makeBitmapSurface` — neither wrapped in Rust.
- `modules/resources.rs`: all wrapped C++-documented items already ported (ImageAsset, ImageDecodeStrategy, ResourceProvider + load/loadImageAsset/loadTypeface); `font_mgr()` is rust-specific (no C++ counterpart); FileResourceProvider/CachingResourceProvider/DataURIResourceProviderProxy/ExternalTrackAsset not wrapped.
- `modules/svg/dom.rs`: complete — Builder setters/containerSize/findNodeById/renderNode are NOT wrapped in Rust at all; read/from_str/from_bytes/render are C++-undocumented.
- `gpu/graphite/image.rs`, `surface.rs`: fully documented (0 undocumented pub items).
- `gpu/graphite/backend_texture.rs`, `recording.rs`: C++ headers have 0 doc blocks.
- `gpu/graphite/graphite_types.rs`: contributor-authored docs — do not touch.
- `gpu/ganesh/surface_ganesh.rs` (all 6 fns), `image_ganesh.rs`, `yuva_backend_textures.rs` (incl. `YUVABackendTextures` class doc), `types.rs` (FlushInfo/SubmitInfo documented): complete.
- `gpu/ganesh/gl/interface.rs`+`extensions.rs`+`types.rs`, `vk/vk_types.rs`, `d3d/*`: closed by 356b4749; remaining d3d/mtl/vk backend headers (GrD3DBackendContext.h, GrMtlBackendContext.h, GrVkBackendSurface.h, GrGLBackendSurface.h, Gr*BackendSemaphore.h) have 0 doc blocks.
- `core/contour_measure.rs`: complete (MatrixFlags docs are contributor-added; C++ enum has no docs; pos_tan/get_matrix/get_segment/length/is_closed all ported; ContourMeasure/ContourMeasureIter have no C++ class overviews).
- `core/font_arguments.rs`: complete (FontArguments class doc + Palette doc + all setters ported).
- `core.rs` facade: only 2 undocumented trait methods (contains/quick_reject) — rust-specific, no C++ counterpart.
- `core/typeface.rs`: closed by cb542106 (unique_id was the only genuine miss; equal/serialize/table_tags flags were false positives).
- `core/font_mgr.rs`: complete (count_families/family_name/family_names/new_style_set/legacy_make_typeface have NO C++ docs; empty/match_* ported in 0ddc3258).
- `839633a6` skia-safe: fix unresolved doc links from gpu/utils porting (final `cargo doc` sweep closed the last porting-introduced warnings: `Interface::new_native` make_gl links → `crate::gpu::direct_contexts::make_gl()`; GL `BackendState` doc comment moved inside `bitflags!` (was `unused doc comment`); `_codec.rs` subset doc `EncodedImageFormat::Webp` → `WEBP`; `coverage_mode.rs` module doc `crate::RegionOp` → `crate::region::RegionOp`).
- `e0a0021b` skia-safe: fix unresolved from_backend_texture doc links — the last 2 PRE-EXISTING doc warnings in `core/surface.rs` are now resolved. The C++ `SkSurface.h` `replaceBackendTexture` doc refers to `MakeFromBackendTexture`; the Rust ganesh equivalent is `surface_ganesh::wrap_backend_texture()`, so `[`Self::from_backend_texture`]` → `[`crate::gpu::ganesh::surface_ganesh::wrap_backend_texture()`]` (both occurrences). `cargo doc --no-deps --features gl,vulkan,metal,textlayout,svg,skottie,ureq,webp` now completes with **ZERO warnings**.

## Rule reinforced (2026-09-08)
Do NOT add rustdoc for items that have no C++ doc comment. Only port docs that exist in the C++ header. Items without C++ docs are left undocumented (matches the established "C++ NO DOCS → low value, leave undocumented" rule). `modules/paragraph/paragraph.rs` was already complete (all C++-documented methods ported); it needs no further work.

## PR finalized (2026-09-08)
- Branch `docs` (98 commits ahead of `master`, 0 behind — no rebase needed) opened as **PR #1328** → https://github.com/rust-skia/rust-skia/pull/1328 (`master` ← `pragmatrix:docs`).
- Review (quick depth): no Critical/High/Medium findings. [Low] pre-existing `Self::from_backend_texture` doc links in `core/surface.rs` — user chose "fix issues first"; fixed in `e0a0021b` (→ `crate::gpu::ganesh::surface_ganesh::wrap_backend_texture()`), making `cargo doc` fully warning-free.
- Gates: `cargo fmt -- --check` clean; `make test-macos` (lib + integration + examples) all pass; `cargo clippy -p skia-safe --features "all-macos,ureq" --all-targets -- -D warnings` clean; `cargo doc --no-deps --features gl,vulkan,metal,textlayout,svg,skottie,ureq,webp` zero warnings.
- Note: `make test-macos` requires network (Skia `git-sync-deps` fetches GN); the sandbox's SSL interception blocks it — run unsandboxed.
- Only behavioral changes in the branch: `BackendAPI` → `BackendApi` in Ganesh `backend()` methods (deprecated alias retained) + `variant_name!(LineBreakMode::Strict)`.

## Known remaining doc warnings (PRE-EXISTING, not from porting)
- NONE. All doc warnings resolved as of `e0a0021b` (2026-09-08): `core/surface.rs:344,359` (`Self::from_backend_texture` → `crate::gpu::ganesh::surface_ganesh::wrap_backend_texture()`) fixed; `modules/svg/dom.rs:48` was a stale tracker note (no current warning); UReqResourceProvider no longer warns. `cargo doc --no-deps --features gl,vulkan,metal,textlayout,svg,skottie,ureq,webp` is fully clean.

## Gotchas / rules (from porting)
- Intra-doc links: prefer fully-qualified [`crate::X`] when type not in immediate scope; from inside a module use [`self::item`]; NEVER downgrade a link to plain backticked text (user rule, commit 506b6a68) — find the resolving path.
- BackendApi trap: ganesh::BackendApi (GrBackendApi) ≠ gpu::BackendApi (skgpu/Graphite). Link [`crate::gpu::ganesh::BackendApi`] explicitly.
- Don't run cargo doc after each edit — run once at end of batch.

---

# m154 update (2026-09-20)

Milestone m154: crates `0.154.0`, skia-bindings metadata `skia = "m154.5"`, Skia submodule tag `m154.5`, `core/milestone.rs` `MILESTONE = 154`.

Branch `m154` commits (vs `master`): `52e5110a` Update Skia to milestone 154 · `2c950404` + `fff83457` milestone-update skill / PR template · `a6d1b9fa` Port FlushResult API (refresh from `chrome/m154`) · `cf9c3b2b` skill branch naming · `2017e567` assert Send + Sync for `gpu::FlushResult` · `c619dcad` re-insert the missing `is_clang.py` compiler-command patch (unpushed at the time of writing).

## Header accounting (m153-0.101.2 → m154-0.153.3)

Method: `git -C skia-bindings/skia diff m153-0.101.2 m154-0.153.3` — 115 changed `.h` files in total. Only the 13 listed below live under `include/` or `modules/` (11 Skia headers, 2 third-party skcms internals). The remaining 102 are `src/`, `bench/`, `tests/` and `tools/` internals (Skia rewrote `SkPicture`'s internals — `SkBigPicture` deleted, `SkCachedData`, `SkPicturePriv` —, reworked `GrRenderTask`/`GrResourceAllocator` in Ganesh and the graphite `DrawList*`/`Renderer`/`Geometry` code, and added `sparse_strips/` and `src/gpu/graphite/geom/`). None of those are reachable through `skia-bindings`/`skia-safe`, so the m153 doc corpus stays valid for every header that did not change.

| Changed header | Rust counterpart | Doc status |
|---|---|---|
| `include/core/SkMilestone.h` (`153`→`154`) | `core/milestone.rs` | ✅ `MILESTONE = 154` (no prose) |
| `include/core/SkPicture.h` | `core/picture.rs` | ⭐ **GAP — 1 doc sentence** (see below) |
| `include/gpu/ganesh/GrDirectContext.h` | `gpu/ganesh/direct_context.rs`, `gpu/ganesh/types.rs` | ✅ prose ported; ⭐ thin docs on 2 helper fns |
| `include/gpu/graphite/ContextOptions.h` | `gpu/graphite/context_options.rs` | ⭐ **GAP — 1 field doc** |
| `modules/skshaper/include/SkShaper.h` | `modules/shaper.rs` | ✅ Rust wrapper documented (new `Options` API) |
| `include/gpu/ganesh/SkSurfaceGanesh.h` | `gpu/ganesh/surface_ganesh.rs` | Flush/FlushAndSubmit not wrapped (pre-existing) |
| `include/gpu/graphite/GraphiteTypes.h` | — | not wrapped (`DrawTypeFlags`, `Precompile`) |
| `include/private/chromium/GrVkSecondaryCBDrawContext.h` | — | not wrapped (private/chromium) |
| `include/core/SkSerialProcs.h` | — | not wrapped (TODO in `picture.rs`, `flattenable.rs`) |
| `modules/skottie/include/SkottieProperty.h`, `TextShaper.h` | — | not wrapped (`TextPropertyValue`, `ShapingProps`) |
| `modules/skcms/src/*.h` | — | third-party internals; upstream added `modules/skcms/LICENSE` |

Upstream added **no** public header and removed none (the only `--diff-filter=ADR` hit under `include/`+`modules/` is the skcms `LICENSE`).

## Gaps / follow-ups found in the m154 update

Documentation regressions (C++ doc exists, Rust doc missing/now incomplete — same rule as the m153 corpus: port the existing C++ text, never invent):
1. ⭐ `core/picture.rs` — `SkPicture.h::approximateOpCount()` gained a sentence: *"If 0 is returned, we say the SkPicture is "empty" meaning its cullRect is the result of an SkRect::MakeEmpty()."* Neither `approximate_op_count()` nor `approximate_op_count_nested()` has it.
2. ⭐ `gpu/graphite/context_options.rs` — `ContextOptions.h::fUseDrawListLayer` is new and documented: *"Enabling switches Graphite from the existing sort-based draw ordering to the new layer-based system."* `use_draw_list_layer()` / `set_use_draw_list_layer()` have no docs (the file is otherwise documented).

API follow-ups from the m154 `FlushResult` migration (not doc-specific, recorded here because they came out of the same header accounting):
- `GrDirectContext::flushAndSubmit(GrSyncCpu)` and `flushAndSubmit(SkSurface*, GrSyncCpu)` now return `FlushResult`, but `C_GrDirectContext_flushAndSubmit` still calls `self->flushAndSubmit()` and discards the result, so `DirectContext::flush_and_submit()` keeps returning `&mut Self` and the new return value is unreachable through the Rust API. (`DirectContext::flush_submit_and_sync_cpu()` predates m154 — introduced in `13f13c14` m130 — and is semantically equivalent to the upstream `flushAndSubmit(sync)`, so it needs no change; only its thin doc is worth completing.)
- The `GrDirectContext.h` prose for `flushAndSubmit(SkSurface*, GrSyncCpu)` — *"Call to ensure all reads/writes of the surface have been issued to the underlying 3D API. Skia will correctly order its own draws and pixel operations. This must be used to ensure correct ordering when the surface backing store is accessed outside Skia (e.g. direct use of the 3D API or a windowing system). This is equivalent to calling ::flush with a default GrFlushInfo followed by ::submit(syncCpu). Has no effect on a CPU-backed surface."* — is not ported to `flush_and_submit_surface()` (which carries only a shorter paraphrase).
- `SkSurfaces::Flush` / `SkSurfaces::FlushAndSubmit` (`gpu/ganesh/SkSurfaceGanesh.h`, now returning `GrDirectContext::FlushResult`) have no C wrapper and no Rust API at all; their C++ prose (*"Clients should strive to call GrDirectContext::flush directly. However, there exist some places where the GrDirectContext is hard to find, these helpers allow for the flushing of the provided surface. This is a no-op if the surface is nullptr or not GPU backed."*) has no Rust counterpart.
- `GrDirectContext::flush(SkSurface*)` (default-info overload, documented *"Flushes the given surface with the default GrFlushInfo. Has no effect on a CPU-backed surface."*) is not wrapped; Rust exposes only `flush_surface()` via `flushSurfaceWithAccess(NoAccess, default)`.
- Doc-convention note: upstream folded the *"If the return is GrSemaphoresSubmitted::kYes …"* blocks into *"If FlushResult.fSubmitted is …"*. The Rust `flush()` doc in `gpu/ganesh/direct_context.rs` was updated to match verbatim, but it is line-wrapped irregularly (e.g. "call (it is / possible Skia failed to create a subset of the semaphores)"). Cosmetic, not a correctness gap — worth normalizing if the file is touched again.

## m154 header docs already correctly ported (verified against the m154 headers)

- `gpu/ganesh/types.rs`: `FlushResult` struct overview *"Result of a Ganesh flush call. A flush can be successful with or without any semaphores being flushed. In some circumstances an unsuccessful flush can still have flushed the semaphores, but the rendering results should be discarded."* ✅ plus both field docs (`success`, `submitted`) ✅. `From<FlushResult> for SemaphoresSubmitted` carries a deliberate divergence comment: it keys on `success` only, as upstream's implicit `operator GrSemaphoresSubmitted()` does, rather than on `submitted`; and it is kept because the Rust helper path (`flush_image`, `flush_and_submit_surface`) would otherwise lose the `kNo` result upstream reports (`RustSkiaContexts::flush` returns `fSubmitted`). No TODO in the code marks that reason — the tracker records it here.
- `gpu/graphite/context_options.rs`: contributor docs plus the new `use_draw_list_layer` accessor pair ported in `a6d1b9fa` (only the field prose is missing, see gap 2).
- `modules/shaper.rs`: new `SkShaper::Options` (upstream comment: width = *"Width available for horizontal layout, before wrapping kicks in."*; tracking = *"Extra advance added after each glyph, expressed as a fraction of the font size (i.e. em units, thus it scales with the text). It applies on top of font kerning and participates in line breaking."*) is covered by `shape_with_iterators_and_features_and_options(…, width, tracking, …)`; the doc explains tracking as "glyph tracking (letter spacing) in addition to the width available for horizontal layout" without quoting the em-unit comment. The underlying `SkShaper_Options` struct is bindgen-generated but never re-exported: the wrapper passes `width`/`tracking` as separate scalars, so there are no Rust fields to document.
- `core/picture.rs`: unchanged m153 docs are still correct against m154 for `playback`, `cull_rect`, `from_data`/`from_bytes`, `serialize`, `new_placeholder`, `approximate_bytes_used`, `to_shader` (upstream only reflowed parameters, made `playback`/`cullRect`/`approximateOpCount`/`approximateBytesUsed` non-virtual, and replaced the `SkBigPicture` subclass hooks with an `SkRecord`-based private constructor — none of that changes the documented prose).

## m154: nothing to port (verified)

- `SkPicture` class overview (`\class SkPicture …`) and `SkPicture::AbortCallback` have no Rust counterpart at all (the AbortCallback API is a standing TODO in `picture.rs`), so their C++ prose remains unported by design, not by omission.
- `SkSerialProcs.h` / `SkDeserialProcs`: still unwrapped (`MakeFromData`/`serialize` take no procs); the m154 change only dropped the `SK_LEGACY_DESERIAL_IMAGE_PROC` guard and added a `std::optional<SkAlphaType>` parameter. Nothing to port until the procs API is wrapped.
- `GraphiteTypes.h`: `DrawTypeFlags` is not wrapped (`Precompile` has no binding either), so `kDrawMesh`'s new pipeline-label comments and the `Tris*`→`Pos*` `VerticesRenderStep` renames are out of scope.
- `GrVkSecondaryCBDrawContext.h`: private/chromium, not wrapped — its `flush()` → `GrDirectContext::FlushResult` change needs no Rust work.
- `SkottieProperty::TextPropertyValue::fTextTracking` and `TextShaper::ShapingProps::fTextTracking`: none of `TextPropertyValue`/`ShapingProps`/`SkottieProperty` is wrapped by `skia-safe`, so the new field comments have no Rust surface.

---

# MISSING / UNCHECKED YET (survey 2026-09-08, milestone m153)

> **Stale relative to m154:** the surveys and tables in this section were taken against the **m153** headers and were not re-verified for m154. Re-verification found no *new* gaps in these areas (the m154 header diff touched none of them), but the per-file counts below are the m153 numbers with the m153 commit history.

File-by-file audit of the remaining `skia-safe` crate. "Rust doc" = doc-comment coverage (none/partial/mostly/complete). "C++ richness" = whether the matching C++ header carries substantive doc comments (rich/sparse/no docs). Items where C++ has NO docs are LOW porting value and are candidates to leave undocumented per the established rule. This section is the unchecked backlog, NOT yet verified against headers.

## codec/ — ✅ DONE (committed above)
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `codec/_codec.rs` | `SkCodec.h` | **complete** ✅ | **rich** ⭐ |
| `codec/decoders.rs` | `SkPngDecoder.h`, `SkJpegDecoder.h`, `SkWebpDecoder.h`, `SkGifDecoder.h`, `SkBmpDecoder.h`, `SkWbmpDecoder.h`, `SkIcoDecoder.h`, `SkAvifDecoder.h`, `SkRawDecoder.h`, `SkJpegxlDecoder.h` | **complete** ✅ | rich |
| `codec/codec_animation.rs` | `SkCodecAnimation.h` | complete ✅ | rich |
| `codec/encoded_origin.rs` | `SkEncodedOrigin.h` | complete ✅ | rich |
| `codec/encoded_image_format.rs` | `SkEncodedImageFormat.h` | complete ✅ (C++ no per-variant comments) | some |
| `codec/pixmap_utils.rs` | `SkPixmapUtils.h` | complete | rich |

Note: `SkAndroidCodec.h` (rich) not wrapped at all (TODO in codec.rs).

## encode_/ — ✅ DONE (committed above; all C++ rich)
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `encode_.rs` | `SkEncoder.h` | complete ✅ | rich |
| `encode_/png_encoder.rs` | `SkPngEncoder.h` | complete ✅ | rich |
| `encode_/jpeg_encoder.rs` | `SkJpegEncoder.h` | complete ✅ | rich |
| `encode_/webp_encoder.rs` | `SkWebpEncoder.h` | complete ✅ | rich |
| `encode_/png_rust_encoder.rs` | `SkPngRustEncoder.h` | complete ✅ | rich |

## core/font_mgr.rs — ✅ DONE (committed `0ddc3258`; SkFontMgr.h rich)
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `core/font_mgr.rs` | `SkFontMgr.h` | **complete** ✅ | **rich** ⭐ |

## pathops/ — ✅ DONE (committed `f21a3f3a`; SkPathOps.h rich)
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `pathops.rs` | `SkPathOps.h` | **complete** ✅ | **rich** |

## utils/ — mostly LOW (C++ undocumented), one high-value item
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `utils/shadow_utils.rs` | `SkShadowUtils.h` | **none — 5 undoc** | **rich** ⭐ |
| `utils/null_canvas.rs` | `SkNullCanvas.h` | none — 2 undoc | some |
| `utils/ordered_font_mgr.rs` | `SkOrderedFontMgr.h` | none — 3 undoc | some |
| `utils/camera.rs` | `SkCamera.h` | none — 19 undoc | **NO DOCS** (low) |
| `utils/custom_typeface.rs` | `SkCustomTypeface.h` | none — 9 undoc | **NO DOCS** (low) |
| `utils/parse_path.rs` | `SkParsePath.h` | none — 6 undoc | **NO DOCS** (low) |
| `utils/text_utils.rs` | `SkTextUtils.h` | none — 6 undoc | **NO DOCS** (low) |

## interop/ — LOW (private module, internal glue, skip)
| File | C++ counterpart | Rust doc | C++ richness |
|---|---|---|---|
| `interop.rs` | internal bindings glue | none | — |
| `interop/cpp.rs` | bindings.h glue | sparse — 4 undoc | internal |
| `interop/stream.rs` | `SkStream.h` + glue | sparse — 19 undoc | SkStream.h rich, glue internal |
| `interop/string.rs` | SkString glue | none — 6 undoc | internal |
| `interop/strings.rs` | SkStrings glue | sparse — 4 undoc | internal |

## gpu/ — graphite good shape; ganesh subdirs mostly low; a few rich items
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `gpu/types.rs` | `GpuTypes.h` | **none — 5 undoc** | **rich** ⭐ |
| `gpu/mutable_texture_state.rs` | `MutableTextureState.h` | none — 6 undoc | some |
| `gpu/vk.rs` + `gpu/vk/vulkan_*.rs` | `VulkanTypes.h`, `VulkanBackendContext.h`, `VulkanMutableTextureState.h`, etc. | none/sparse — ~34 undoc | mostly NO DOCS (low) |
| `gpu/graphite/context_options.rs` | `ContextOptions.h` | **sparse — 1 undoc** | **rich** ⭐ |
| `gpu/graphite/context.rs` | `Context.h` | **complete** ✅ (delete_backend_texture + is_device_lost full C++ docs) | rich |
| `gpu/graphite/recorder.rs` | `Recorder.h` | mostly (1 undoc) | rich |
| `gpu/graphite/texture_info.rs` | `TextureInfo.h` | partial (1 undoc) | sparse |
| `gpu/graphite/backend_texture.rs` | `BackendTexture.h` | partial (1 undoc) | NO DOCS |
| `gpu/graphite/recording.rs` | `Recording.h` | none — 1 undoc | NO DOCS |
| graphite surface/image/graphite_types/mtl/vk | various | mostly/complete | mostly rich or NO DOCS |
| `gpu/ganesh/gl/types.rs`, `gl/interface.rs`, `gl/extensions.rs` | `GrGLTypes.h`, `GrGLInterface.h`, `GrGLExtensions.h` | **complete** ✅ (incl. new_load_with/new_load_with_cstr from `GrGLAssembleInterface.h`) | some/rich |
| `gpu/ganesh/vk/vk_types.rs` | `GrVkTypes.h` | sparse — 5 undoc | rich |
| `gpu/ganesh/d3d/types.rs` | `GrD3DTypes.h` | **complete** ✅ (TextureResourceInfo note + FenceInfo.value) | rich ⭐ |
| `gpu/ganesh/mtl/types.rs` | `GrMtlTypes.h` | **complete** ✅ (TextureInfo struct doc) | some |
| ganesh gl/vk/mtl/d3d other files | GrGL*/GrVk*/GrMtl*/GrD3D* headers | none/sparse | mostly NO DOCS (low) |

## modules/shaper/ — MODERATE
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `modules/shaper.rs` | `SkShaper.h` | **sparse — ~45 undoc** | sparse-but-meaningful |
| `modules/shaper/core_text.rs` | `SkShaper_coretext.h` | none — 1 undoc | NO DOCS |
| `modules/shaper/harfbuzz.rs` | `SkShaper_harfbuzz.h` | none — 4 undoc | NO DOCS |
| `modules/shaper/unicode.rs` | `SkShaper_skunicode.h` | none — 1 undoc | NO DOCS |

## modules/paragraph/ — ✅ DONE already (C++ rich; all documented methods ported; do NOT invent docs for undocumented accessors)
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `modules/paragraph/paragraph.rs` | `Paragraph.h` | complete (all C++-documented methods) | **rich** ⭐ |
| `modules/paragraph/text_style.rs` | `TextStyle.h` | sparse — ~92 undoc | sparse |
| `modules/paragraph/paragraph_style.rs` | `ParagraphStyle.h` | none — 53 undoc | NO DOCS |
| `modules/paragraph/font_collection.rs` | `FontCollection.h` | none — 21 undoc | NO DOCS |
| paragraph_builder/cache/dart_types/font_arguments/typeface_font_provider/text_shadow/metrics | `ParagraphBuilder.h` etc. | none | NO DOCS (all) |

## modules/svg/ — LOW value (C++ almost all NO DOCS)
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `modules/svg/dom.rs` | `SkSVGDOM.h` | partial — 26 doc | **rich** (has pre-existing warning line 48) |
| `modules/svg/fe.rs` | `SkSVGFe.h` | none | some |
| `modules/svg/use.rs` | `SkSVGUse.h` | **complete** ✅ (module doc from class overview) | some |
| all other svg/* (node, shape, types, gradient, container, etc.) | `SkSVG*` headers | none — ~180 total undoc | **almost all NO DOCS** (low) |

## modules/ — 
| File | C++ header | Rust doc | C++ richness |
|---|---|---|---|
| `modules/resources.rs` | `SkResources.h` | **partial — 9 undoc** | **rich** ⭐ |
| `modules/skottie.rs` | `Skottie.h`, `ExternalLayer.h`, `SkottieProperty.h`, `SlotManager.h` | **complete** ✅ | rich |
| `modules.rs` / `skottie.rs` / `svg.rs` | re-export facades | none | — |

## Core — verify done vs gap (corrected from earlier belief)
| File | C++ header | Status |
|---|---|---|
| `core/strike_ref.rs` | `SkStrikeRef.h` (rich) | **complete** ✅ |
| `core/typeface.rs` | `SkTypeface.h` (rich 206) | **mostly** ✅ (only ~6–10 aliases/unique_id undoc) |
| `core/font_mgr.rs` | `SkFontMgr.h` (rich 64) | **GAP — ~27 undoc** ⭐ (only new_from_data/new_from_bytes documented) |
| `core/tiled_image_utils.rs` | `SkTiledImageUtils.h` (rich 21) | **GAP — 4 undoc** |
| `core/recorder.rs` | `SkRecorder.h` (some 7) | **GAP — 4 undoc** |
| `core/cpu_recorder.rs` | `SkCPURecorder.h` (rich 20) | **GAP — 1 undoc** |
| `core/mesh.rs` | `SkMesh.h` (rich 138) | **EMPTY stub** (nothing to port) |
| `core/promise_image_texture.rs` | `GrPromiseImageTexture.h` (private) | **EMPTY file** (nothing to port) |
| `core/font_scanner.rs` | `SkFontScanner.h` (NO DOCS) | **stub** (nothing to port) |
