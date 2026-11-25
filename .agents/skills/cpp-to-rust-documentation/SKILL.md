---
name: cpp-to-rust-documentation
description: "Use when porting documentation from Skia C++ headers to Rust APIs, including Rustdoc comments, parameter descriptions, type links, or C++-to-Rust names."
---

# C++ to Rust Documentation

When porting documentation from C++ headers:

- Keep wording as close to the original as possible, including grammatical errors, except where Rust terminology or syntax requires a change.
- Port the documentation completely: do not shorten, summarize, or omit any
  information from the C++ comment. Every sentence, clause, and detail must be
  carried over (for example, the per-color-type breakdown in
  `SkPixmap::computeIsOpaque` must be included in full, not reduced to a
  one-line summary). Only drop content that is explicitly excluded by another
  rule.
- Before porting a file, read the full corresponding C++ header and the full
  Rust file side by side, and port every public method that has a C++ doc
  comment. Do not rely on memory or on a partial survey: open the header and
  walk through each method in order, checking that each documented C++ method
  has a matching Rust doc. When reviewing for omissions, re-read the C++ header
  again rather than trusting a prior pass.
- Document parameters using a list entry per parameter, backticking the Rust
  parameter name and following it directly with the description (for example,
  `` - `color` unpremultiplied RGBA ``). Do not add a colon between name and
  description, and do not reword the description into a sentence.
- Convert `@return` clauses into a prose sentence starting with `Returns ...`
  instead of a list entry (for example, `Returns unpremultiplied RGBA.`).
- Link types using brackets (for example, `[`Type`]`).
- Link functions and methods using the Rust name with call parentheses (for example, `SkPath::arcTo` becomes `[`Path::arc_to()`]`).
- Use Rust equivalents for C++ types and constants (for example, `SkPoint` becomes `[`Point`]` and `kMove_Verb` becomes `[`PathVerb::Move`]`).
- Do not rename functions when generating documentation.
- If the Rust function name differs from the C++ function name, use the Rust name in the documentation text.
- Ensure documentation parameter names match the Rust function parameter names.
- Refer to parameter values in prose as backticked code (`` `None` ``), and use
  `None` / `Some` instead of `nullptr` / optional wording when describing Rust
  `Option` parameters.
- Keep `example:` fiddle links, but add a concise hint that the example is in
  C++ (for example, `Example (C++):` or a short note that the fiddle is C++),
  since rustdoc does not render Skia fiddles and the linked code is C++. Wrap
  the URL in angle brackets (for example,
  `Example (C++): <https://fiddle.skia.org/c/@Canvas_drawRect>`) so rustdoc
  treats it as an explicit link; the `@` in fiddle URLs can otherwise break
  bare-URL auto-linking.
- Every reference to a Rust item in documentation must be a working intra-doc
  link, never plain backticked text. When a link fails to resolve, find the
  path that resolves instead of downgrading the reference to plain code text:

  - Enum variants link like any other item once the enum's path is correct
    (for example, `GrBackendApi::kOpenGL` becomes
    `[`crate::gpu::ganesh::BackendApi::OpenGL`]`). A failure usually means the
    path points at a different type with the same name — qualify further
    rather than dropping the link.
  - From inside the module that defines an item, link it with `[`self::item`]`
    — the module's own name is not in scope there.
  - Link the type that actually declares the member: references in prose to
    `dst.colorType()` style C++ chains become links to the Rust counterpart
    methods, e.g. `[`Pixmap::color_type()`]`; sibling methods of the documented
    type are `[`Self::method()`]`, or `[`Type::method()`]` when `Self` refers
    to a different type (for example inside a trait impl like `Iterator`).
- Do not add documentation above `variant_name!` macro invocations. These are
  compile-time API checks, not public Rust items requiring rustdoc.

## Documenting re-exported enums

Many enums are re-exported from `skia_bindings` rather than defined in
`skia-safe`, for example:

```rust
pub use skia_bindings::SkRRect_Type as Type;
variant_name!(Type::Complex);
```

Rustdoc only reads variant documentation from the item's definition site, and a
`pub use` is a single item with no syntax to attach per-variant `///` comments.
The generated `skia_bindings` enum carries no docs (bindgen runs with
`generate_comments(false)`), so `#[doc(inline)]` has nothing to inline. There is
no way to add per-variant rustdoc blocks to a re-exported enum without
re-defining the variants.

To surface the C++ variant documentation anyway, document the variants in the
enum-level doc comment on the `pub use` itself, using intra-doc links to the
variants:

```rust
/// Describes possible specializations of [`RRect`]. Each type is exclusive; an
/// [`RRect`] may only have one type.
///
/// Type members become progressively less restrictive; larger values of type
/// have more degrees of freedom than smaller values.
///
/// Variants:
/// - [`Type::Empty`]: zero width or height.
/// - [`Type::Rect`]: non-zero width and height, and zeroed radii.
/// - [`Type::Oval`]: non-zero width and height filled with radii.
/// - [`Type::Simple`]: non-zero width and height with equal radii.
/// - [`Type::NinePatch`]: non-zero width and height with axis-aligned radii.
/// - [`Type::Complex`]: non-zero width and height with arbitrary radii.
pub use skia_bindings::SkRRect_Type as Type;
```

- Port the C++ `\enum` overview into the doc comment, then list each variant
  with its `//!<` comment, backticking the variant name and following it
  directly with the description (no colon between name and description).
- Link each variant with `[`Type::Variant`]` so the references resolve.
- Do not re-define the enum in `skia-safe` just to get per-variant rustdoc
  blocks; keeping the discriminants in sync with the C++ values is not worth
  the duplication.

## Module level documentation

- When a Rust module wraps a single C++ header or class, port the header's class
  overview documentation into the module's `//!` documentation at the top of the
  module file.
- If the C++ header provides no class or module level documentation, write a
  very concise module description instead (one or two lines, no prose padding).

## Porting tracker

- Track every ported unit in the persistent tracker at
  `skia-safe/docs-porting-tracker.md` (mirrored in repo memory at
  `/memories/repo/docs-porting-tracker.md`). The tracker records, per unit, the
  Rust file, the C++ header it was ported from, and the commit that completed
  it, and is verified for completeness against the C++ header.
- Before starting a new file, check the tracker to see whether the unit was
  already ported or was previously determined to have no C++ documentation
  (items without C++ doc comments are left undocumented by rule).
- After completing a unit, update the tracker with the commit and any
  corrections or rule refinements learned during the port.
