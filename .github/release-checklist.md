# Release Checklist

Start here only after the `release` branch is ready, its binary workflows have
finished, and the release notes are prepared. All executable steps go through
`just`.

Set `<version>`, `<previous>`, `<commit>`, and `<notes>` to concrete values.

## Verify

- [ ] Run `just release-verify <version> <previous> <commit>`. This verifies the
      release/master commit, every binary workflow, the binary release assets,
      binary downloads, source builds, and the optional `skia-svg-macros`
      package dry run.
- [ ] Run `just release-verify-images <previous> <commit>`, then inspect
      `/tmp/rust-skia-release-images/comparison/summary.tsv` and every image in
      its `review` directory.
- [ ] Run `just release-verify-vector <previous> <commit>`, then inspect
      `/tmp/rust-skia-release-vector/comparison/summary.tsv` and its `review`
      directory (advisory SVG/PDF check).
- [ ] Review `<notes>` against the
      [release notes guidelines](release-notes-guidelines.md).

Image comparison exits 0 for an exact match, 1 when differences need human
review, and 2 when the check could not run. Stop for a failed workflow, missing
artifact, source-build fallback, or unexplained image change. The vector gate
is advisory: treat its differences as a signal to review, not a hard block.

## Publish

Crate and GitHub publication are irreversible and require explicit approval.

- [ ] Run `just release-publish-crates <version> <previous>`. It publishes only
      the required crates in dependency order, waits for each registry entry,
      and runs the post-release smoke test. If `skia-svg-macros` changed, it is
      published first.
- [ ] Run `just release-publish-github <version> <commit> <notes> true`. All
      rust-skia GitHub releases are prereleases. This creates or resumes the
      GitHub release and verifies its tag.

Never republish an existing crate version. If only `skia-svg-macros` has been
published, reuse it and resume. If the release commit changes after
`skia-bindings` or `skia-safe` is published, use a new rust-skia patch version.
