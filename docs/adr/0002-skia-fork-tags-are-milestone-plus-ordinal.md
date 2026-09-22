# Skia fork tags name the milestone and the cut, not the crate version

`skia-bindings/Cargo.toml` records the Skia revision to build in `[package.metadata].skia`, and
`just check-skia-submodule-tag` requires that value to be one of the tags on the `skia/` submodule
HEAD. The tag is `m<milestone>.<ordinal>`: the Skia milestone, then the position of the tag within
that milestone, counted from `0` over all tags of the milestone — including tags cut under the
previous scheme, whose ordinal is their trailing number. A new milestone starts at `m155.0`; further
tags for an already tagged milestone take the next number, so the first tag after `m154-0.153.4`
(ordinal 4) is `m154.5`. A published tag is never rewritten or moved: an unpushed tag that was cut
too early is re-cut under the same ordinal, while repairing a published one takes the next ordinal.
The previous scheme had no room for that (its name was derived from a crate version, so a re-cut
forced the ad-hoc suffix of `m153-0.101.0-x`), and the ordinal has to be monotone for the mapping
tag-to-revision to stay unambiguous.

The crate version left the name because it was already a step removed from the release it identified
(`m154-0.153.2` was tagged while the crates were at `0.154.0`) and because nothing consumes it. The
binary cache tags its releases with `CARGO_PKG_VERSION`, and the source download uses the metadata
value only as the git ref of a codeload URL and as the name of the unpacked root directory, which
means the tag must stay short because it becomes part of a deep path. The only reader of the version
component was the milestone procedure itself, which had to look it up before cutting, and the
duplicated information drifted: the porting tracker recorded `m154-0.153.3` while the tree was
already at `m154-0.153.4`. The milestone skill and `AGENTS.md` name the scheme, and
`docs-porting-tracker.md` records the tag that is current.

## Considered Options

- **Rejected: keep the previous crate version in the name.** Carries a number that nothing reads, and
  every cut pays for it with a lookup that is easy to get wrong, as the m154 transition showed.
- **Rejected: milestone only (`m154`).** A refresh of an already tagged milestone has no name left,
  and moving the tag ref would break every consumer of the tagged revision.
- **Rejected: count the tags of the milestone instead of taking the largest number in use.** The
  count would restart numbering in the middle of an existing sequence — `m154.3` is the fourth tag
  for m154 and would name a different revision than the published `m154-0.153.3`.
- **Rejected: zero-padded ordinal (`m154.05`).** No reader needs fixed-width lexicographic order; the
  number is compared and incremented numerically.
