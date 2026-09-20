# A source build never moves an existing `skia/` submodule checkout

The revision of the `skia/` submodule is recorded in the superproject's gitlink, but developers move
the checkout on purpose: a milestone refresh builds the rebased and tagged tip before the gitlink is
staged, a bisect moves it backwards, and testing a wrapper against a newer Skia needs it ahead of the
recorded revision. Since `git submodule update` resets the submodule to the recorded revision, the
build initializes a missing submodule but never moves an existing one, and reports a checkout that
differs from the recorded revision with a warning instead. `just check-skia-submodule-tag` stays the
check that the recorded tag and the gitlink agree.

## Considered Options

- **Rejected: update unconditionally.** Repairs a stale checkout and keeps `git pull` transparent,
  but silently discards deliberate checkouts, which forced the milestone workflow to stage the
  gitlink before every build to protect itself.
- **Rejected: never initialize and fail with instructions instead.** Breaks `git clone && cargo build`
  and contradicts the README, which promises that `cargo build` builds Skia.
- **Deferred: build Skia from a build owned source directory** (`OUT_DIR`/`target`) so that the build
  writes nothing into the repository. The hermetic answer, but Skia's `tools/git-sync-deps`,
  `bin/fetch-gn`, the `gn` invocation, and the bindgen include paths all assume an in-repository
  checkout today.
