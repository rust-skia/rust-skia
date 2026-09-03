---
name: rust-skia-release-verification
description: Verify a prepared rust-skia release after the release branch has built its binaries. Use to check release commits, CI, binary assets, generated images, crate packaging, and release notes before publication. This skill never publishes crates or creates GitHub releases.
---

# rust-skia Release Verification

Read the [release checklist](../../release-checklist.md) and the
[release notes rules](../../release-notes-guidelines.md). Coordinate executable
operations through the verification recipes in the repository `justfile`.

Establish the release version, previous version, exact release commit, notes
file, and canonical remote. Never proceed across a mismatch: remote `master`
and `release`, binary artifacts, and the candidate commit must identify the
same release.

1. Run `just release-verify <version> <previous-version> <commit>`. It verifies
   the branches and binary workflows/assets, verifies binary-download and
   source-build packaging, and dry-runs `skia-svg-macros` packaging when that
   crate changed.
2. Run `just release-verify-images <previous-version> <commit>` and inspect every
   reported difference. This compares the CPU-rendered PNGs across every
   platform that produces QA artifacts (linux, macos, windows, windows-arm).
   The candidate comes from the `master` QA run at the exact release commit,
   because pushing `release` triggers binary workflows but not QA workflows.
   Read `/tmp/rust-skia-release-images/comparison/summary.tsv` and inspect its
   `review` directory. Exit status 0 is an exact match, 1 means differences
   require review, and 2 means the check could not run. Added, removed, resized,
   blank, corrupt, clipped, missing-text, wrong-color, and unexplained pixel
   changes block publication.
3. Run `just release-verify-vector <previous-version> <commit>` and inspect the
   reported SVG/PDF differences (advisory). This rasterizes the `svg/` and
   `pdf/` trees with third-party tools (`rsvg-convert`, Ghostscript) that are
   not Skia, so a difference only indicates the vector content changed as seen
   through a foreign renderer. Read
   `/tmp/rust-skia-release-vector/comparison/summary.tsv` and inspect its
   `review` directory.
4. Validate the release notes against the repository rules.
5. Report the verified version, previous version, exact commit, notes file,
   canonical remote, and image-review result.

## Options

- To compare only one vector format, pass it as the third argument:
  `just release-verify-vector <previous> <commit> svg` (or `pdf`).
- To check a subset of platforms, pass a space-separated list of
  `workflow:artifact` pairs as the `platforms` argument to either recipe.
- To pin the run used for a workflow instead of auto-selecting the most recent
  successful run at each commit, pass a space-separated list of `workflow:run`
  pairs as the `runs` argument to either recipe. This is useful when a commit
  has multiple successful runs and you want a specific one.

## Failure prevention

- Run source-build, package, and documentation-binding checks serially. They
   share `skia-bindings/skia/third_party/externals`; concurrent builds can leave
   nested dependency checkouts at incompatible revisions or with missing files.
- Use `just release-verify` for package checks instead of raw
   `cargo publish --dry-run`. A source build may legitimately modify its
   extracted Skia tree, so verification packages with `--no-verify`, extracts
   the crate, and builds that exact package explicitly.
- Treat direct `skia-bindings` features as independent. Checks that enable
   `gl` or `vulkan` and exercise Ganesh must also enable `ganesh` explicitly.
- Before handing off to publication, ensure no verification process is still
   using the shared Skia checkout.

This skill ends after reporting verification. Never invoke a publishing
recipe; publication requires a separate invocation of the
`rust-skia-release-publishing` skill.

Any source fix invalidates verification. Return to release preparation, then
rerun both verification gates.
