---
name: rust-skia-release-publishing
description: Publish a previously verified rust-skia release to crates.io and GitHub. Use only after rust-skia-release-verification has completed for the exact release commit and the user has explicitly authorized publication. Do not use for verification or release preparation.
---

# rust-skia Release Publishing

Read the [release checklist](../../release-checklist.md) and the
[release notes rules](../../release-notes-guidelines.md). Coordinate every
operation through the publishing recipes in the repository `justfile`.

Before publishing, require the completed verification result: release version,
previous version, exact release commit, notes file, canonical remote, and image
review result. Confirm that these values still describe local `HEAD`, remote
`master` and `release`, binary artifacts, and the intended final tag. If the
result is missing, stale, or mismatched, stop and invoke the
`rust-skia-release-verification` skill.

## Failure prevention

Before requesting approval for crate publication:

- Query publication state with
   `cargo info --registry crates-io <crate>@<version>`. An unqualified lookup
   can resolve the local workspace package and falsely report it as published.
- With no source-build or package check running, generate documentation
   bindings using `env -u FORCE_SKIA_BUILD -u FORCE_SKIA_BINARIES_DOWNLOAD make
   bindings-docs`. Require a successful exit and a non-empty
   `/tmp/bindings.rs` before publishing any crate, including
   `skia-svg-macros`. This catches a damaged nested dependency checkout before
   publication becomes partial.
- If documentation generation reports missing or incompatible files under a
   nested Skia dependency, stop. Repair or resynchronize that checkout, then
   rerun the documentation-binding preflight; never overlap it with another
   Skia source build.
- Post-publication smoke tests must install the exact release with
   `skia-safe@=<version>`, including the Emscripten build. An unpinned install
   can select an older release and produce an irrelevant failure.

Crate and GitHub publication are separate irreversible operations. Unless the
user has already explicitly authorized the specific operation, stop for
approval immediately before each command.

Publish in this order:

1. `just release-publish-crates <version> <previous-version>` publishes the
   required crates in dependency order, verifies registry availability, and
   runs smoke tests. It automatically includes `skia-svg-macros` first when the
   crate changed.
2. `just release-publish-github <version> <commit> <notes> true` creates or
   resumes the GitHub release as a prerelease and verifies the final tag.
   rust-skia GitHub releases are always prereleases; never omit `true`.

Do not replace a published crate version. If only `skia-svg-macros` has been
published, confirm that state against crates.io, reuse it, and resume at
`skia-bindings`; never attempt to republish it. If the release commit must
change after `skia-bindings` or `skia-safe` was published, use a new patch
version. Any source fix invalidates verification; return to release
preparation and rerun the `rust-skia-release-verification` skill.
