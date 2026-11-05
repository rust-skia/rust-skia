This template should be used for new Skia Milestone update PRs, which can be added
as soon new `chrome/m**` branches appear in https://github.com/google/skia.

Title: `Milestone XX` (e.g. `Milestone 155`).

---

This PR aligns rust-skia with Skia's `chrome/mXX` branch.

- [x] Update `README.md` ([rendered](https://github.com/pragmatrix/rust-skia/blob/mXX/README.md))  
  > Most important here is to change the Skia branch name and the current Skia submodule tag at the top of the `README.md` file.
- [ ] Read the Skia [release notes](https://github.com/google/skia/blob/chrome/mXX/RELEASE_NOTES.md) and record a decision for every note under "Release notes" below.
- [ ] Diff the build organization files to see if they changed significantly: `/.gn`,
  `/BUILD.gn`, `/gn/**`, `/modules/*/BUILD.gn`, `/modules/*/*.gni`,
  `/third_party/*/BUILD.gn`, and `/third_party/*/*.gni`. List each changed file under
  "Build organization" below.
- [ ] Review every third-party pin in `DEPS`. Each is the newer of Chromium's commit for the
  milestone and Skia's own pin (`just chromium-deps-pins XX`), or a stable release that fixes a
  known vulnerability in it. `just audit-skia-deps` and `just check-third-party-sources` pass,
  and a local source build compiled the pins its platform uses.
  List each changed pin under "Third-party dependencies" below.
- [ ] Diff the generated Dawn headers and the undefined `wgpu*` functions of `libskia-bindings.a`,
  and record the changes under "Dawn headers" below.
- [ ] Re-check every callback that Skia registers against the release poll, and list them under
  "Release poll" below.
- [ ] Compare the newest `wgpu` version with the newest `wgpu-NN` feature of `skia-wgpu`, and
  record the outcome under "`wgpu` versions" below.
- [ ] Check the upstream commits for raised toolchain minimums, and list them under "Toolchain
  requirements" below.
- [ ] Skia builds.
- [ ] `/skia-bindings` builds.
- [ ] `/skia-safe`: Update & add new wrappers by diffing include files.  
  > Add TODOs for everything that can not be updated right now, and attempt to stay compatible with previous versions of skia-safe without trying too hard before version 1.0. Use `deprecated` attributes if needed.
- [ ] Look for `todo!()` macros.
- [ ] Review `Send` & `Sync` implementations for new wrappers.
- [ ] Review `Debug` implementation for new wrapper types and functions.
- [ ] Release date of the matching Chrome version in less than 7 days?  
    > Blocks every item below until checked. Check it once `stable_date` from
    > `https://chromiumdash.appspot.com/fetch_milestone_schedule?mstone=XX` is less than
    > 7 days away (release the crates with the PR's merge).
- [ ] Any pending changes in the Skia `chrome/mXX` branch that aren't synchronized yet?
- [ ] Rebase on or merge with master.
- [ ] Do the fork patches in the `skia-bindings/skia` subdirectory match with `master` (`just diff-skia`).
- [ ] Update versions of `skia-bindings/Cargo.toml` and `skia-safe/Cargo.toml` and also add the version to the new `deprecated` attributes.
- [ ] Review API changes: `just diff-api`.  
    > The diff compares with the latest crates.io release, so it also lists what master
    > changed since that release. An item belongs to this PR only if
    > `git diff master...HEAD -- skia-safe/src` touches it.
- [ ] Do one final review of all the changes.
- [ ] Run `just doc` and fix all warnings.

> Every section below holds at least one entry. Write `no change` where a check found nothing,
> so a skipped check stands out from a clean one.

## Release notes

> Record a decision for every note of Skia's release notes between the old and the new tag:
> what it changes for rust-skia, or `no change`.

## Build organization

> List the build organization files that changed and record each as `no change` or
> `updated build glue`. Most source-list changes flow through the existing Skia
> targets; call out `declare_args()` changes in `gn/skia.gni` explicitly, because
> rust-skia pins several of those flags.

## Third-party dependencies

> List each changed `DEPS` pin with its old and new version, and whether it is Chromium's
> commit for the milestone, Skia's newer pin, or a stable release taken for a vulnerability
> fix. List the entries added to or removed from `.github/scripts/audit-skia-deps.ignore`,
> and any `third_party/*/BUILD.gn` change the new pins needed.

## Dawn headers

> Record the Dawn, Jinja2, and MarkupSafe revisions, every change of the generated `webgpu.h`
> that Skia's Dawn backend uses (functions, structs, enum values, feature names), the `wgpu*`
> functions added to or removed from the undefined symbols of `libskia-bindings.a`, the decision
> on every new `wgpuDeviceHasFeature` check, and any change mirrored from `skia_use_dawn` into
> `skia-bindings/build_support/dawn.rs`.

## Release poll

> List every callback that Skia registers (the `CallbackMode` search of the milestone update),
> with the reason it is safe while the device and Skia's shared context are torn down: it uses
> only its own buffer or state that outlives them, Skia waits for it, or its body is empty.

## `wgpu` versions

> Record the newest `wgpu` version and the newest `wgpu-NN` feature of `skia-wgpu`, and the
> decision on a newer major version.

## Toolchain requirements

> List each upstream commit that the toolchain search finds, with the minimum it raises and the
> decision for it (raise the minimum, or carry a fork patch), or `no requirement`.

## Wrapper updates

> Summarize the public API changes and how they were wrapped: new types and their
> wrapper kind (`native_transmutable!`, `Send`/`Sync`, `Debug`), changed method
> signatures including any temporary conversion that keeps existing call sites
> compiling, new `deprecated` attributes with their `since` version, and which of
> the changed public headers needed a wrapper change at all.

---

## AI disclosure

Parts of this PR were prepared with AI assistance: submodule
rebase/retag bookkeeping, header accounting, and wrapper updates. All changes
were reviewed and verified by the listed authors before being committed.
