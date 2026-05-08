# rust-skia Release Notes Guidelines

This document defines the release notes format for rust-skia so release notes can be produced consistently without referencing prior releases.

## Scope

Use this format for rust-skia crate releases (for example `0.97.0`, `0.97.1`).

## Output Format

Release notes should use this section order:

1. Intro paragraph
2. `## Notable Changes`
3. `## What's Changed`
4. `## New Contributors` (only if applicable)
5. `Full Changelog: <compare URL>`

## Intro Paragraph Rules

- Start with what the release aligns to and what range it covers.
- For milestone releases, include milestone PR references in this paragraph.
- Keep it concise (1 sentence, optionally 2 short sentences).
- Include the upstream Skia release notes URL when applicable.

Recommended pattern:

`This release aligns the Rust bindings with Skia milestones A, B, and C in #<pr>, #<pr>, and #<pr>, and includes cumulative updates since <last-tag> (upstream Skia notes: <url>).`

## Notable Changes Rules

- Include 4-8 high-impact bullets.
- Group related work when possible (for example Vulkan fixes across 2 PRs).
- Do not repeat milestone entries here if already covered in the intro paragraph.
- Use plain GitHub references only: `#1234`, `@user`.

## What's Changed Rules

- Use one bullet per merged PR that should appear in release notes.
- Format each bullet like this:

`- <summary> by @<author> in #<pr>[, reported by @<reporter> in #<issue>]`

- Use `reported by` attribution when there is a related issue reporter or explicit suggestion issue.
- Do not use `closes`, `fixes`, or similar wording in the release note text.
- Do not use Markdown links for PRs/users/issues; rely on GitHub auto-linking.
- Do not include milestone PR bullets here when those were already stated in the intro paragraph.

## New Contributors Rules

- Include this section only if first-time contributors exist in the release range.
- Format:

`- @<user> made their first contribution in #<pr>`

## Full Changelog Rule

Always end with a compare URL in plain text:

`Full Changelog: https://github.com/rust-skia/rust-skia/compare/<from-tag>...<to-tag>`

## Data Collection Checklist

Before finalizing notes:

- Identify release range (`<from-tag>..<to-tag>`).
- Collect merged PRs in range.
- Capture PR authors.
- Extract issue associations for `reported by` attribution.
- Identify first-time contributors.
- Verify milestone PR numbers and upstream Skia notes URL.

## Quality Checklist

- Milestones are mentioned once (intro only).
- No Markdown links for PRs/users/issues.
- `reported by` wording is used consistently.
- Section order matches this document.
- Compare URL is present and correct.
