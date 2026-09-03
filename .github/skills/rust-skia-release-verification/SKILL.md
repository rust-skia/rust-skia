---
name: rust-skia-release-verification
description: Verify a prepared rust-skia release after the release branch has built its binaries. Use to check release commits, CI, binary assets, generated images, crate packaging, and release notes before publication. This skill never publishes crates or creates GitHub releases.
---

# rust-skia Release Verification

Read the [release checklist](../../release-checklist.md), the
[release notes rules](../../release-notes-guidelines.md), and the
[image comparison gate](references/image-comparison.md). Coordinate executable
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
   reported difference.
3. Run `just release-verify-vector <previous-version> <commit>` and inspect the
   reported SVG/PDF differences (advisory).
4. Validate the release notes against the repository rules.
5. Report the verified version, previous version, exact commit, notes file,
   canonical remote, and image-review result.

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
