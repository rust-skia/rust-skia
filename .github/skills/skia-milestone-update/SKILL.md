---
name: skia-milestone-update
description: Perform a Skia milestone update for rust-skia or refresh the current milestone from its upstream chrome/mXX branch. Use when the user asks to "continue the milestone update", "update to milestone mXX", "update Skia from upstream", or references the milestone update process. Covers the Skia fork rebase and tag sequence, README update, build org diff, third-party DEPS pins, the Dawn revision and its generated headers, the newest `wgpu` version, header diff accounting, wrapper updates, version bump, API diff review, and final checks.
---

# Skia Milestone Update

This skill performs a Skia milestone update for rust-skia, aligning it with a new
Skia `chrome/mXX` branch. Use the bundled [`PR_TEMPLATE.md`](PR_TEMPLATE.md) as
the working PR checklist and follow every item in order. The item on the Chrome
release date blocks every item below it. While that release is 7 or more days away, stop
at that item and tell the user the date. It is based on the
[Template: Skia Milestone Update PR](https://github.com/rust-skia/rust-skia/wiki/Template:-Skia-Milestone-Update-PR)
wiki page and extends it with project-specific coverage such as Graphite. The
`just diff-skia` caveat lives in `AGENTS.md` — read it before starting.

CI builds and tests every platform on the PR. Leave each build or check that CI runs to
CI, unless the checklist or this skill names it, and run the named ones natively on the
local machine.

## Inputs

- `OLD_TAG`: the current Skia submodule tag (e.g. `m154.5`)
- `NEW_TAG`: the target Skia submodule tag (e.g. `m154.6` or `m155.0`)
- `OLD_MILESTONE` / `NEW_MILESTONE`: the numeric milestones (e.g. `154` / `155`)

A Skia fork tag is `m<milestone>.<ordinal>`, where the ordinal is the tag's position
within the milestone: `m155.0` is the first tag ever cut for milestone 155, `m155.1`
the second. The ordinal is independent of the crate version and carries no other
meaning; see `docs/adr/0002-skia-fork-tags-are-milestone-plus-ordinal.md`.

Determine `OLD_TAG` from `git -C skia-bindings/skia describe --tags`, and
`NEW_TAG` from `git -C skia-bindings/skia tag --list 'm<MILESTONE>*'`. For a new
milestone it is `mNEW_MILESTONE.0`. For a refresh of the current milestone, add one to
the largest ordinal already used by any tag for that milestone. Under the previous tag
scheme that number is the trailing component of the tag, so `m154-0.153.4` counts as
ordinal 4:

```sh
git -C skia-bindings/skia tag --list 'm154*' | grep -oE '[0-9]+$' | sort -n | tail -1
```

Read the tag list by eye as well: a tag with a non-numeric suffix such as
`m153-0.101.0-x` is skipped by that command and can hide a larger ordinal.

A published tag is immutable: never rewrite or delete one. An unpushed tag that was cut
too early is re-cut under the same ordinal, while a repair of a published tag takes the
next ordinal.

Choose `OLD_TAG` as the newest previous-milestone tag: patches can land on it after
an earlier tag was cut, and a milestone based on the stale one drops them. The
`just diff-skia` checklist item below is what catches this.

Name the temporary local working branch distinctly from every tag, for example
`mNEW_MILESTONE-refresh` (`m154-refresh`). Never give a branch the same name as a
tag. With both refs present Git warns that the refname is ambiguous, and
`git rev-parse <name>` resolves the tag rather than the branch. In `git log`
decorations the tag is prefixed with `tag:` and the branch is not, which is easy to
misread as two tags on the same commit. For the same reason, always spell out
`refs/tags/…` or `refs/heads/…` in the commands below instead of bare ref names.

Note that the version component of a Skia fork tag does not track the milestone for
tags up to `m152` (for example `m152-0.100.0`), and only aligns with it from `m153`
on. Take both tags from the repository instead of deriving one from the other.

## Records

The PR description is the record of the update: what each check found and the decision made
for it. Reviewers read it there, and it survives the wait for the Chrome release date. Open the
PR as a draft from `PR_TEMPLATE.md` as soon as the parent branch is pushed (step 8 of the
rebase below), with the records of every check that finished before.

The template has one section per check of this skill, and each section says what it records:
Release notes, Build organization, Third-party dependencies, Dawn headers, Release poll, `wgpu`
versions, Toolchain requirements, and Wrapper updates. Each check below ends by recording
its outcome in its section. Update the description with `gh`:

```sh
gh pr view --json body --jq .body > "$TMPDIR/NEW_TAG-pr.md"
# Edit the section in "$TMPDIR/NEW_TAG-pr.md".
gh pr edit --body-file "$TMPDIR/NEW_TAG-pr.md"
```

The records are complete when every section holds at least one entry, with `no change` written
out where the check found nothing, so a skipped check stands out from a clean one.

The one working list outside the PR is the per-header list of the public header check, in
`$TMPDIR/NEW_TAG-headers.md`. It is used before the wrapper edits start, and its outcome goes
into Wrapper updates.

## Rebase the fork patch stack

Use this sequence when `upstream/chrome/mXX` advances after the initial milestone
tag, and for a new milestone. Preserve the rust-skia patch stack by rebasing it; do not
merge upstream into the fork tag.

For a new milestone, `OLD_BASE` comes from the old milestone's branch:
`git -C skia-bindings/skia merge-base refs/tags/OLD_TAG upstream/chrome/mOLD_MILESTONE`.
Every other `upstream/chrome/mXX` below is `upstream/chrome/mNEW_MILESTONE`. A base taken
from the new branch is the point where the two branches diverged, and the rebase then
replays the old milestone's upstream-only commits as if they were fork patches.

1. Refresh both repositories and inspect the new upstream commits:

   ```sh
   git fetch --all --prune
   git -C skia-bindings/skia fetch --all --prune
   git -C skia-bindings/skia rev-list --left-right --count refs/tags/OLD_TAG...upstream/chrome/mXX
   git -C skia-bindings/skia log --oneline refs/tags/OLD_TAG..upstream/chrome/mXX
   ```

   Then confirm that the parent branch contains rust-skia's `master`
   (`upstream/master` when `origin` is a personal fork):

   ```sh
   git merge-base --is-ancestor upstream/master HEAD
   ```

   If it does not, rebase the parent branch onto `upstream/master` before anything else, and
   re-read this skill from there. `master` can gain fork tags, CI checks, and changes to
   this skill while a milestone branch is open. A branch that misses them builds on a fork
   stack without `master`'s latest fork patches and follows an outdated copy of this skill.

   Then list the `master` commits that moved the Skia gitlink since `OLD_TAG` was cut:

   ```sh
   git log --oneline --since="$(git -C skia-bindings/skia log -1 --format=%cI refs/tags/OLD_TAG)" upstream/master -- skia-bindings/skia
   ```

   A commit that moves the gitlink to a tag other than `OLD_TAG` brings fork patches that
   `OLD_TAG` lacks. List them with `git -C skia-bindings/skia log --oneline
   refs/tags/TAG_BEFORE..refs/tags/TAG_AFTER`, and cherry-pick each onto the rebased
   stack after step 3, before the tag is cut.

2. Review the upstream diff. If it changes public headers, complete the full header
   accounting below before updating wrappers. Record internal-only changes as requiring
   no binding update.

3. Find the old upstream base (for a new milestone, see above), create the temporary
   branch at `OLD_TAG`, and rebase the complete rust-skia patch stack onto the refreshed
   branch:

   ```sh
   git -C skia-bindings/skia merge-base refs/tags/OLD_TAG upstream/chrome/mXX
   git -C skia-bindings/skia switch -c mNEW_MILESTONE-refresh refs/tags/OLD_TAG
   git -C skia-bindings/skia rebase --onto upstream/chrome/mXX OLD_BASE
   ```

   If the temporary branch already exists, switch to it instead of recreating it. `git
   switch` accepts only a bare branch name, which the naming rule above keeps distinct
   from every tag: `git -C skia-bindings/skia switch mNEW_MILESTONE-refresh`.

4. Verify that every downstream patch is unchanged and that the result is based on the
   refreshed upstream tip:

   ```sh
   git -C skia-bindings/skia range-diff OLD_BASE..refs/tags/OLD_TAG upstream/chrome/mXX..HEAD
   git -C skia-bindings/skia rev-list --left-right --count upstream/chrome/mXX...HEAD
   ```

   Require every `range-diff` entry to be `=` and the second command to report `0 N`,
   where `N` is the number of rust-skia patches.

5. Tag the rebased tip with the next ordinal, using an explicit ref. Keep `NEW_TAG` in
  `[package.metadata].skia` in sync when you do. For example, when the newest m154 tag
  is `m154.5`:

  ```sh
  git -C skia-bindings/skia tag m154.6 refs/heads/m154-refresh
  ```

  Do not bump the Rust crate versions for a same-milestone upstream refresh.

6. Update `[package.metadata].skia`, the README comparison links, the milestone line of
   `skia-safe/docs-porting-tracker.md` (it names the current tag), and the parent
   repository's submodule gitlink. Stage all of them before running Cargo or another
   build job:

   ```sh
   git add README.md skia-bindings/Cargo.toml skia-safe/docs-porting-tracker.md skia-bindings/skia
   ```

   The gitlink must name the rebased tip before any build that starts without a `skia/`
   checkout, because the build script initializes a missing submodule from it: that is
   what CI and fresh clones do. `just check-skia-submodule-tag` compares the tag in
   `[package.metadata].skia` with the tags at the submodule *checkout* HEAD, so it fails
   while the checkout is at the new tag and the gitlink is not staged yet; step 7 below
   verifies the staged gitlink itself. The build never moves an initialized checkout, so
   it cannot reset the rebased tip in this working tree (`docs/adr/0001`). Never rewrite
   a tag that is already published.

   Confirm that these files differ from `master` by the milestone deltas **only**:

   ```sh
   git diff master -- README.md skia-bindings/Cargo.toml skia-safe/Cargo.toml
   ```

   When the milestone commit was created before `master` gained changes to those files,
   a conflict resolution that keeps the milestone side (`--ours`/`--theirs`, "take mine")
   silently reverts `master`'s changes to them, such as new README content or a
   `bindgen` upgrade. No build or test fails, because a stale README is just text and the
   older `bindgen` still builds, so `git diff` is the only reliable detector. Every removed
   line that `master` added is a regression, even though `git log mNEW_MILESTONE..master`
   is empty (the history is still an ancestor relationship; only the tree was damaged).
   Re-apply the milestone deltas on top of `master`'s content and re-verify before
   proceeding.

7. Run `just diff-skia`, then
   `cargo clippy -p skia-safe --features all-<host> --all-targets -- -D warnings`, where
   `<host>` is `macos`, `linux`, or `windows`, then the platform tests with the same
   features. The default features leave feature-gated wrappers such as Graphite's
   uncompiled, and one Skia build with `all-<host>` serves the check, the lints, and the
   tests.
   Reconfirm after the builds, with explicit refs, that the checkout, the new tag, and
   the staged gitlink all point at the rebased tip:

   ```sh
   git -C skia-bindings/skia rev-parse HEAD
   git -C skia-bindings/skia rev-parse refs/tags/NEW_TAG^{commit}
   git ls-files -s skia-bindings/skia
   ```

8. Push the new Skia tag only with explicit user authorization, then verify the remote
   tag points at the expected commit:

   ```sh
   git -C skia-bindings/skia push origin refs/tags/NEW_TAG
   git -C skia-bindings/skia ls-remote --tags origin refs/tags/NEW_TAG
   ```

   Pushing the temporary branch is not required.

   Push the parent branch with `git push --recurse-submodules=no` once the tag is on the
   remote. The fork publishes its commits only as tags, so Git's submodule push check, which
   looks for them on remote branches, rejects every push that changes the gitlink. Then open
   the draft PR (see Records above), unless it exists already.

## Notes that go beyond the wiki checklist

- **Versioning:** synchronize the Rust crate minor version with the numeric Skia
  milestone: milestone `mXX` uses crate version `0.XX.0`. The Skia fork tag is
  unrelated to the crate version — see the note under Inputs for its scheme.
  Update all of these together:
  - `skia-bindings/Cargo.toml` package version;
  - `skia-safe/Cargo.toml` package version and exact `skia-bindings` and `skia-wgpu`
    dependencies;
  - `skia-wgpu/Cargo.toml` package version and exact `skia-bindings` dependency;
  - `skia-bindings/Cargo.toml` `[package.metadata].skia`, matching `NEW_TAG`
    exactly;
  - `skia-bindings/Cargo.toml` `[package.metadata].dawn`, `.jinja2`, and `.markupsafe`,
    see the Dawn note below;
  - `skia-safe/src/core/milestone.rs` `MILESTONE` constant, matching `SK_MILESTONE`
    in `include/core/SkMilestone.h`.
  `Cargo.lock` is gitignored and untracked, so it is not part of the commit. Delete it
  when you bump the versions, so that the first build re-resolves it. A stale local lock
  fails the build even when the tree is correct, for example when it makes bindgen
  compile against two `syn` versions.
  Add the synchronized crate version to any new `deprecated` attributes
  (`since = "0.XX.0"`). For a same-milestone upstream refresh, leave the crate
  versions unchanged and increment only the Skia fork tag's ordinal.
- **PR title:** the milestone PR is titled `Milestone XX` (milestone number
  only, no `m` and no tag ordinal — e.g. `Milestone 155`), matching the
  previous milestone PRs. Update the title of an already opened PR if the
  milestone changes direction, but never during a same-milestone refresh.
- **Skia release notes:** read them first, before the build organization diff and the
  header accounting, and record a decision for every note in the PR's Release notes section:

  ```sh
  git -C skia-bindings/skia diff refs/tags/OLD_TAG..refs/tags/NEW_TAG -- RELEASE_NOTES.md relnotes/
  ```

  A note can announce a change that no header or build-file diff shows. A new minimum OS
  version, for example, can come from a single call in a source file. A new iOS minimum
  requires raising `MIN_IOS_VERSION` in `skia-bindings/build_support/platform/ios.rs`.
- **Build organization diff:** review whether the build organization changed
  significantly by diffing the build files between the old and new tags:

  ```sh
  git -C skia-bindings/skia diff --stat refs/tags/OLD_TAG..refs/tags/NEW_TAG -- .gn BUILD.gn 'gn/**' 'modules/*/BUILD.gn' 'modules/*/*.gni' 'third_party/*/BUILD.gn' 'third_party/*/*.gni'
  ```

  `.gn` belongs in the list because rust-skia builds Skia as the gn root, so the
  `default_args` in `.gn` apply to our build. A flag that `gn/skia.gni` declares off can
  therefore be on, as `skia_enable_packed_422_yuv` is. The defines that bindgen
  actually sees are in `skia-defines.txt`, next to the generated `bindings.rs`.
  Record each file in the PR's Build organization section. Most
  source-list changes flow through existing Skia targets and need no rust-skia
  build glue change. However, any diff in the `gn`/`gni` files that looks
  relevant — one that may need special consideration or may indirectly affect
  the bindings or any build process — must be reviewed by the user. Do not
  silently decide on such a diff; surface it and get explicit confirmation
  before proceeding.
- **Third-party dependencies (`DEPS`):** review every `deps` entry of the fork's
  `DEPS` file on every tag cut. Upstream Skia's `DEPS` and `third_party/*/BUILD.gn`
  serve only Skia's standalone build. Chrome compiles Skia's code against Chromium's own
  copies of these libraries, so upstream rarely updates them. Its pins can lag months or
  years behind the libraries' releases.
  - Pin each library to the newer of two commits: the one Chromium uses for the same
    milestone, and Skia's own pin. One commit is newer when its history contains the
    other. When neither contains the other, the commit with the higher release version
    is newer, whatever the commit dates say. A maintenance branch, such as the
    `angle-vma-*` branches Chromium uses for VulkanMemoryAllocator, can carry recent
    commits on an older release line. When Chromium pins no copy of the library, keep Skia's pin. Chromium pins no
    D3D12MemoryAllocator, because Chrome never builds Skia's Direct3D backend. The
    milestone's Chromium branch number is `chromium_branch` in
    `https://chromiumdash.appspot.com/fetch_milestones?mstone=XX`. Chromium's `DEPS` on
    `refs/branch-heads/<branch>` pins most libraries, some of them through `vars`.
    libpng, brotli, and zlib are part of Chromium's own tree instead. For libpng and
    brotli, take the version from `third_party/<lib>/README.chromium` on that branch.
    For zlib, take the commit of the `chromium/src/third_party/zlib` mirror whose tree
    equals Chromium's `third_party/zlib` tree on that branch.
    `just chromium-deps-pins NEW_MILESTONE` performs these lookups. For each pinned
    library, it prints both pins and which one is newer by history. Pins it reports as
    diverged need the release versions compared by hand.
  - The one exception: when Chromium's commit has a known vulnerability that applies to
    rust-skia and a newer stable release fixes it, pin that release.
  - A vulnerability applies when rust-skia compiles the affected code. For example,
    harfbuzz's WebAssembly shaper is never compiled, because Skia does not define
    `HAVE_WASM`. Record every accepted vulnerability with its reason in
    `.github/scripts/audit-skia-deps.ignore`, and remove the entries that no longer
    match.
  - When a new pin needs build glue changes in `third_party/<lib>/BUILD.gn`, follow
    Chromium's `third_party/<lib>/BUILD.gn` on the same branch, and compile the upstream
    sources it compiles. Skia lists platform integrations and optional back ends under
    `unused_sources`, where Chromium compiles them and disables them with defines, so
    keep Skia's split. Surface these gn diffs to the user, as for the build
    organization diff above.
  - `just audit-skia-deps` must pass. It checks every pin against OSV.dev and applies
    the ignore file.
  - Verify the pins with a source build on the local machine, after touching
    `skia-bindings/build.rs`:

    ```sh
    touch skia-bindings/build.rs
    FORCE_SKIA_BUILD=1 cargo test -p skia-safe --features "textlayout,svg,webp,embed-freetype,freetype-woff2,pdf"
    ```

    The build script does not track `DEPS` or the `third_party` build files. Without
    the touch, Cargo reuses the previous Skia build and the new pins are never compiled.
    The local build compiles the pins its platform uses. macOS, iOS, and Windows render
    fonts with CoreText or DirectWrite and build Skia without freetype, so the build drops `embed-freetype` and `freetype-woff2` there as
    redundant. CI compiles the Linux pins. The Linux QA jobs on the PR build Skia from
    source with `svg`, `textlayout`, and `webp`, and the Linux binaries job on the release
    branch builds `embed-freetype`. No CI job builds `freetype-woff2`, so tell the user
    when a pin change affects woff2.
  - Only the current platform's branches of each `BUILD.gn` are compiled, so a stale
    path in another platform's branch, or in a header list, breaks no local build.
    `just check-third-party-sources` must pass. It checks every path the
    `third_party` build files list against the checkouts of the pinned libraries. It fails
    and names the libraries it skipped when their checkout is not at the pin.
  - Record the pins in the PR's Third-party dependencies section.
- **Dawn headers (`wgpu` feature):** source builds with `wgpu` generate Skia's WebGPU
  headers from the Dawn revision in `[package.metadata].dawn`
  (`docs/adr/0003`). No file in the repository shows how they change, so every tag cut
  diffs them.
  - The fork's `DEPS` has no entries for Dawn and for Jinja2 and MarkupSafe, the Python
    packages of Dawn's generator. Take their revisions from upstream, and copy them into
    `[package.metadata].dawn`, `.jinja2`, and `.markupsafe`:

    ```sh
    git -C skia-bindings/skia show upstream/chrome/mNEW:DEPS | grep -E 'dawn.git@|/jinja2@|/markupsafe@'
    ```

  - Before bumping, build with the old revision and keep its output. The generated files
    are in `out/dawn` of the `skia-bindings` build directory, next to `out/skia/bindings.rs`:

    ```sh
    cargo build -p skia-bindings --features wgpu
    cp -R target/debug/build/skia-bindings-*/out/dawn "$TMPDIR/dawn-old"
    ```

    When several `skia-bindings-*` build directories exist, take the newest one, as for
    `bindings.rs`.
  - Bump `[package.metadata].dawn`, build again, and diff:

    ```sh
    cargo build -p skia-bindings --features wgpu
    diff -ru "$TMPDIR/dawn-old" target/debug/build/skia-bindings-*/out/dawn
    ```

  - Read the whole diff of `include/dawn/webgpu.h`, and diff Skia's Dawn backend between
    the tags for new WebGPU calls:

    ```sh
    git -C skia-bindings/skia diff refs/tags/OLD_TAG..refs/tags/NEW_TAG -- src/gpu/graphite/dawn include/gpu/graphite/dawn
    ```

  - The `wgpu*` functions that `libskia-bindings.a` leaves undefined are the ones the
    `skia-wgpu` shim must export (`docs/adr/0004`). List them for the old and the new build,
    from the newest `skia-bindings-*` build directory each time, and diff the lists. The
    pattern skips the C++ namespace `wgpu` in mangled names:

    ```sh
    nm -u "$(ls -t target/debug/build/skia-bindings-*/out/skia/libskia-bindings.a | head -1)" | grep -oE 'wgpu[A-Z][A-Za-z]*' | sort -u
    ```

    Record the changes in the PR's Dawn headers section. Each changed function, struct,
    enum value, and feature name that Skia's Dawn backend uses, and every undefined `wgpu*`
    function that was added or removed, needs a matching change in the `skia-wgpu` shim. The
    `nm` list is the authoritative check: a function the shim lacks fails the link of the `wgpu` tests
    only if the tests keep the backend code that calls it. A changed signature fails the
    shim's `WGPUProc*` check at compile time. A new
    `wgpuDeviceHasFeature` check in Skia needs a decision whether the shim reports the
    feature, recorded in `skia-wgpu/FEATURES.md`, whose stated milestone moves to the new one.
    Surface these changes to the user, as for the build organization diff above.
  - Re-check the release poll. Two kinds of Skia callbacks run while the shim's device and
    Skia's shared context are being torn down: the poll at the release of the last device
    handle fires the callbacks with `AllowSpontaneous`, and the release of the instance fires
    every callback that has not fired yet with `CallbackCancelled`, whatever its mode. ADR 0003
    argues that both are safe (the consequence on the release poll). List every callback that
    Skia registers at the new tag. The pattern matches the C++ spelling `CallbackMode::` and the
    C spelling `WGPUCallbackMode_`, anywhere in Skia's sources:

    ```sh
    git -C skia-bindings/skia grep -n 'CallbackMode' refs/tags/NEW_TAG -- src include
    ```

    The step is done when every listed callback is read and recorded in the PR's Release poll
    section, and each one is confirmed safe in one of three ways: it uses only its own buffer or state
    that outlives the device and the shared context, Skia waits for it before that state
    goes away, or its body is empty. A new callback, or one that reaches the device or the
    shared context, invalidates the ADR's argument: surface it to the user, because the
    release poll needs a change before the update ships.
  - `skia-bindings` compiles Skia's Dawn backend itself, from the `.cpp` files of
    `skia_graphite_dawn_sources` in `gn/graphite.gni`, and leaves `skia_use_dawn` off. When the
    build organization diff changes what `skia_use_dawn` adds in `BUILD.gn` (sources, defines,
    include directories, dependencies), mirror the change in
    `skia-bindings/build_support/dawn.rs`.
- **`wgpu` versions:** `skia-wgpu` offers a feature `wgpu-NN` for each supported `wgpu` major
  version, and `skia-safe`'s feature `wgpu` enables the one a release recommends
  (`docs/adr/0003`). Removing a `wgpu-NN` feature or moving `skia-safe`'s `wgpu` to another one
  breaks applications, so both only happen with a milestone version bump, and this update is the
  place to propose them.
  - Look up the newest `wgpu` version on crates.io, and compare its major version with the newest
    `wgpu-NN` feature in `skia-wgpu/Cargo.toml`:

    ```sh
    cargo info wgpu
    ```

    A `wgpu` minor or patch release needs no change, because `wgpuNN = { package = "wgpu",
    version = "NN" }` already accepts it. Record the outcome in the PR's `wgpu` versions section.
  - When a newer major exists, propose the shim update to the user and implement it only after
    their decision. The proposal names the new feature `wgpu-NN` with its optional dependency
    `wgpuNN` and module `skia-wgpu/src/vNN.rs`, the `wgpu` API changes that the shim's source
    meets (from the `wgpu` changelog, and from `cargo check -p skia-wgpu --features wgpu-NN`),
    and which of them go into the per-version `compat` module. It checks each deviation in the
    previous version's `skia-wgpu/src/vNN/README.md` against the new version, so the new
    `compat` module and its own `README.md` keep only the workarounds the new version still
    needs. It also asks whether `skia-safe`'s
    feature `wgpu` moves to the new version, and whether an old `wgpu-NN` feature is removed.
  - A new `wgpu-NN` feature also needs its forwarding feature in `skia-safe/Cargo.toml`,
    `wgpu-NN = ["_wgpu", "skia-wgpu/wgpu-NN"]`. `skia-safe`'s `wgpu` code, its test
    `skia-safe/tests/wgpu.rs`, and the example `wgpu-window` depend only on the hidden feature
    `_wgpu`, so they change only where the new `wgpu` version changes an API that they use. Build
    them with the new version to find these places:

    ```sh
    cargo clippy -p skia-safe --features wgpu-NN --tests --examples
    ```

    Prebuilt binaries need no change, because `skia-bindings`' feature `wgpu` does not depend on
    the `wgpu` version.
  - `skia-wgpu/Cargo.toml` names the `wgpu-NN` feature that docs.rs documents, in
    `[package.metadata.docs.rs]`. Move it to the newest supported version.
- **Toolchain requirements:** list the upstream commits that may raise a compiler,
  C++ library, or OS minimum, and read each commit message:

  ```sh
  git -C skia-bindings/skia log --oneline -i -E \
    --grep='C\+\+2[0-9]|compiler|toolchain|deployment target|minimum (os|version|sdk)|libc\+\+|libstdc\+\+' \
    refs/tags/OLD_TAG..refs/tags/NEW_TAG
  ```

  The release notes miss some of these requirements. One new language or library feature
  can raise the minimums of several platforms, while the notes name only one of them. Local builds do not reproduce CI's minimums, because CI pins
  `MACOSX_DEPLOYMENT_TARGET` in `mk-workflows` and cross-compiles Linux aarch64 against
  the libstdc++ in the CI image (see `AGENTS.md`). Surface each requirement to the user as
  a decision: raise the minimum, or carry a fork patch. When a minimum rises, also check
  whether a `rust-skia:` fork patch existed only to support the old one, such as an
  `@available` workaround. Record the outcome in the PR's Toolchain requirements section.
- **Include diffs:** use direct `git -C skia-bindings/skia diff refs/tags/OLD_TAG..refs/tags/NEW_TAG -- ...`
  commands. `just diff-skia` only compares the fork patch stack against master (it is
  the "Do the fork patches ... match with `master`" checklist item).
- **Account for every changed public header** before editing wrappers. Start from the
  full list of changed public headers and the complete diff of all of them, then walk
  through every file before making any edits:

  ```sh
  git -C skia-bindings/skia diff --name-only refs/tags/OLD_TAG..refs/tags/NEW_TAG -- ':(glob)include/**/*.h' ':(glob)modules/*/include/**/*.h'
  ```

  The `:(glob)` prefix makes `**/` match zero directories as well, so the list includes
  headers that sit directly in a module's `include/`, such as
  `modules/skparagraph/include/Paragraph.h`. Without it, `**/` requires at least one subdirectory. Review
  inline function and method implementations as carefully as declarations: they are
  compiled into consumers and can change behavior without changing the API signature.
  Include changes that look trivial (include moves, comments, friend declarations,
  build-file public header lists). Common include-path-only moves (e.g.
  `include/private/base/*` -> `include/private/*`) need no binding/wrapper update —
  record them as `no change`.

  Do not cherry-pick a subset of the changed headers (for example, only the ones with
  the largest diffs). Process every header deterministically:

  1. Cross-reference every header in the list against the existing binding/wrapper
     surface — check whether a `skia-bindings/src/*.cpp` wraps any function/type from
     it, and whether a corresponding `skia-safe/src/...` wrapper module exists.
  2. Classify each changed header into one of:
     - **no binding exists, no wrapper exists** — usually no action unless it is a newly
       exposed public API that should be wrapped;
     - **binding exists, wrapper exists** — diff the header and update the C wrapper in
       `skia-bindings/src/*.cpp` and the Rust wrapper in `skia-safe/src/...` to match.
       For changed inline implementations, determine whether the C wrapper calls or
       relies on the affected behavior, and record that review even when no source edit
       is needed;
     - **new public header** — decide whether to add a binding + wrapper, and record
       the decision.
  3. Record every changed header in the header list, `$TMPDIR/NEW_TAG-headers.md`, with its
     decision: `no change`, `updated binding`, `updated wrapper`, `added new`, `skipped —
     internal only`, or `skipped — no binding/wrapper`. **Start editing wrappers only once
     every header has an entry.**
  4. Batch the headers by classification and process each batch in one pass, editing the
     matching `*.cpp` and `skia-safe/src/...` files together. Re-run
     `cargo check -p skia-bindings` (touching `bindings.cpp` first to force bindgen
     regeneration) and `cargo check -p skia-safe` after each batch.

  Cover every wrapper area systematically, mirroring the `skia-safe/src` layout:
  `codec/`, `core/`, `docs/`, `effects/`, `encode/`, `gpu/` (with `ganesh/`,
  `graphite/`, `mtl/`, `vk/`), `pathops/`, `svg/`, `utils/`, and `modules/` (with
  `paragraph/`, `shaper/`, `skottie/`, `skresources/`, `svg/`). For each area,
  cross-reference the changed headers against the corresponding `skia-bindings/src/*.cpp`
  and `skia-safe/src/...` wrapper module, and record the outcome in the header list.
- **Wrapper updates:** follow the `rust-skia-bindings` skill for how to add or change
  a C shim and its Rust wrapper. In short: preserve method/debug-field ordering aligned
  with the upstream C++ header, prefer direct field access over getter/setter shims for
  bindgen-generated data members of C-compatible types, add `todo!()` for anything that
  cannot be updated right now, stay compatible with previous versions of skia-safe
  without trying too hard before 1.0, use `#[deprecated]` if needed, look for `todo!()`
  macros that can now be resolved, and review `Send` & `Sync` and `Debug`
  implementations for new wrappers.
- **Ported docs:** record every doc ported or changed for the milestone in a
  `# mXX update` section of `skia-safe/docs-porting-tracker.md`, with the header each
  doc comes from and any C++ sentence left out on purpose. The milestone line at the top
  of the tracker points to that section.

## Release notes

Release notes are authored separately (typically by the maintainer at release time)
following `.github/release-notes-guidelines.md`. They are **not** part of the milestone
update PR process itself — do not draft or update GitHub release notes during the
milestone update unless explicitly asked.

## Style & conventions

See the `rust-skia-bindings` skill for binding and wrapper mechanics, and `AGENTS.md`
and `.github/copilot-instructions.md` for the full set. Highlights:
- Keep Rust method and debug-field ordering aligned with the upstream C++ header order.
- Keep top-level type declarations in the same sequence as the upstream C++ header.
- For nested C++ types, keep the parent Rust type first and define nested Rust types
  directly below the parent.
- Derive `Debug` for all public types unless there's a specific reason not to; place
  `Debug` first in the derive list.
- Access bindgen-generated fields of C-compatible types directly; do not wrap public
  data members in C getter/setter functions.
- Do not pass C++ class types by value across `extern "C"`; use pointers and/or
  out-parameters. Use placement new for non-trivial out-parameters.
- Match the surrounding code style; keep functions small and deterministic.
- Do not refactor adjacent working code unless required for correctness.
