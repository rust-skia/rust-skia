This template should be used for new Skia Milestone update PRs, which can be added
as soon new `chrome/m**` branches appear in https://github.com/google/skia.

---

This PR aligns rust-skia with Skia's `chrome/mXX` branch.

- [x] Update `README.md` ([rendered](https://github.com/pragmatrix/rust-skia/blob/mXX/README.md))  
  > Most important here is to change the Skia branch name and the current Skia submodule tag at the top of the `README.md` file.
- [ ] Diff the following files to see if the build organization has changed significantly:
  - [ ] `/BUILD.gn`
  - [ ] `/gn/*` (recursively)
  - [ ] `/modules/skshaper/BUILD.gn`
  - [ ] `/modules/skshaper/skshaper.gni`
  - [ ] `/modules/paragraph/BUILD.gn`
  - [ ] `/modules/paragraph/skparagraph.gni`
  - [ ] `/modules/skottie/BUILD.gn`
  - [ ] `/modules/skottie/skottie.gni`
  - [ ] `/modules/svg/BUILD.gn`
  - [ ] `/modules/svg/svg.gni`
- [ ] Skia builds ([release notes](https://github.com/google/skia/blob/chrome/mXX/RELEASE_NOTES.md)).
- [ ] `/skia-bindings` builds.
- [ ] `/skia-safe`: Update & add new wrappers by diffing include files.  
  > Add TODOs for everything that can not be updated right now, and attempt to stay compatible with previous versions of skia-safe without trying too hard before version 1.0. Use `deprecated` attributes if needed.
- [ ] Look for `todo!()` macros.
- [ ] Review `Send` & `Sync` implementations for new wrappers.
- [ ] Review `Debug` implementation for new wrapper types and functions.
- [ ] Release date of the matching Chrome version in less than 7 days?
- [ ] Any pending changes in the Skia `chrome/mXX` branch that aren't synchronized yet?
- [ ] Rebase on or merge with master.
- [ ] Do the `rust-skia:` commits in the `skia-bindings/skia` subdirectory match with `master` (`make diff-skia`).
- [ ] Update versions of `skia-bindings/Cargo.toml` and `skia-safe/Cargo.toml` and also add the version to the new `deprecated` attributes.
- [ ] Review API changes: `make diff-api`.
- [ ] Do one final review of all the changes.
- [ ] Run `make doc` and fix all warnings.

## Wrapper updates

> Summarize the public API changes and how they were wrapped: new types and their
> wrapper kind (`native_transmutable!`, `Send`/`Sync`, `Debug`), changed method
> signatures including any temporary conversion that keeps existing call sites
> compiling, new `deprecated` attributes with their `since` version, and which of
> the changed public headers needed a wrapper change at all.

## Build organization

> List the build organization files that changed and record each as `no change` or
> `updated build glue`. Most source-list changes flow through the existing Skia
> targets; call out `declare_args()` changes in `gn/skia.gni` explicitly, because
> rust-skia pins several of those flags.

---

## AI disclosure

Parts of this PR were prepared with AI assistance: submodule
rebase/retag bookkeeping, header accounting, and wrapper updates. All changes
were reviewed and verified by the listed authors before being committed.
