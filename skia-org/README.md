# skia-org

Renders the examples of Skia's documentation at [skia.org](https://skia.org/) with rust-skia, once
per driver (CPU, PDF, SVG, OpenGL, Vulkan, Metal, Direct3D, wgpu). Each example is a port of the
C++ fiddle that a skia.org page shows. `cargo run -p skia-org -- <OUTPUT_DIR> --driver <DRIVER>`
writes the images to `<OUTPUT_DIR>/<driver>/<folder>/`. The folders below become part of the
published URLs, such as `https://rust-skia.github.io/skia-org/cpu/SkPaint-Overview/…`.

## Pages

The pages live in the Skia submodule under `skia-bindings/skia/site/`. Every page there that
embeds fiddles is listed in one of the two tables, and `just org-unlisted-pages` prints the pages
that are in neither, for example after a milestone update adds one.

Ported pages:

| Page | Module | Output folder |
|---|---|---|
| `site/docs/user/api/skcanvas_overview.md` | `skcanvas_overview.rs` | `SkCanvas-Overview` |
| `site/docs/user/api/SkPath_Overview.md` | `skpath_overview.rs` | `SkPath-Overview` |
| `site/docs/user/api/skpaint_overview.md` | `skpaint_overview.rs` | `SkPaint-Overview` |
| `site/docs/user/api/SkBlendMode_Overview.md` | `skblendmode_overview.rs` | `SkBlendMode-Overview` |
| `site/docs/user/coordinates.md` | `coordinates.rs` | `Coordinate-Spaces` |
| `site/docs/user/sksl.md` | `sksl.rs` | `SkSL` |

Pages not ported, and why:

| Page | Reason |
|---|---|
| `site/docs/_index.md` | Its 8 examples repeat APIs the ported pages cover. Three of them are the SkPaint path effects in blue. |
| `site/docs/user/sample/pdf.md` | Not ported yet. Its example needs the PDF document API. |
| `site/docs/user/tips.md` | Not ported yet. One example. |

`skshaper_example.rs` and `skparagraph_example.rs` (feature `textlayout`) are not taken from a
skia.org page.

## Porting a fiddle

- **Source code.** A fiddle that a page names (`@skcanvas_star`) has its source in
  `skia-bindings/skia/docs/examples/<name>.cpp`. A fiddle that a page embeds by hash has none in the
  repository. `https://fiddle.skia.org/e/<hash>` returns its code and options (canvas size, source
  image) as JSON. Hash-only fiddles are often written against Skia's pre-2020 API.
- **Reference image.** `https://fiddle.skia.org/i/<hash>_raster.png`, or
  `https://fiddle.skia.org/i/@<name>_raster.png` for a named fiddle.
- **Source images.** A fiddle's `image` is the built-in source its options or `REG_FIDDLE` name:
  source 3 is the mandrill (`resources::mandrill()`), source 5 the color wheel
  (`resources::color_wheel()`).
- **Text.** A fiddle that draws with `SkFont(nullptr, …)` gets an empty typeface in current Skia and
  draws no text. The ports use `helper::default_typeface()`.

`just compare-org` renders the examples with two drivers and lists the images that differ.
