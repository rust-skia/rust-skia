# Image Comparison Gate

Run:

```console
just release-verify-images <previous-version> <release-commit>
```

The command downloads the `skia-org-images-x86_64-unknown-linux-gnu` artifact
from successful `linux-qa.yaml` runs at both commits and compares every PNG.
The candidate comes from the `master` QA run at the exact release commit,
because pushing `release` triggers binary workflows but not QA workflows.

Read `/tmp/rust-skia-release-images/comparison/summary.tsv` and inspect every
file under its `review` directory. Exit status 0 is an exact match, 1 means
differences require review, and 2 means the check could not run. Added, removed,
resized, blank, corrupt, clipped, missing-text, wrong-color, and unexplained
pixel changes block publication.

To select another workflow, artifact, or report directory, pass all preceding
optional arguments explicitly:

```console
just release-verify-images <previous> <commit> <report> <workflow> <artifact>
```

If the previous QA artifact has expired, explicitly opt into the published
website CPU images as a visual-only baseline:

```console
just release-verify-images <previous> <commit> <report> linux-qa.yaml skia-org-images-x86_64-unknown-linux-gnu true
```

Do not treat that cross-version website snapshot as an exact pixel oracle.