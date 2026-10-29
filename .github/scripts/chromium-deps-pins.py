#!/usr/bin/env python3
"""Compare the third-party pins in Skia's DEPS file with the ones Chromium uses for a milestone.

Chrome compiles Skia's code against Chromium's own copies of the third-party libraries, so
Chromium's pins for a milestone are the reference for rust-skia's pins (see the DEPS step of
`.github/skills/skia-milestone-update/SKILL.md`). This script looks up Chromium's pin for every
library that Skia's DEPS file pins under `third_party/externals`, and decides which of the two
commits is newer:

- Chromium's branch for the milestone is `chromium_branch` in chromiumdash's milestone data.
- Most pins come from Chromium's DEPS file on that branch, matched by repository path.
- libpng, brotli, and zlib are part of Chromium's own tree. libpng's pin is the release tag that
  `third_party/libpng/README.chromium` names, and brotli's pin is the revision its
  `README.chromium` names. zlib's pin is the commit of the zlib mirror whose tree equals
  Chromium's `third_party/zlib` tree.

One commit is newer when its history contains the other. The script fetches Chromium's commit
into the library's checkout to decide this. When neither history contains the other, the
script reports the pins as diverged, and the release versions decide which is newer.

Usage: chromium-deps-pins.py MILESTONE [path/to/skia]
"""

import base64
import json
import os
import re
import subprocess
import sys
import urllib.request

DEFAULT_SKIA_PATH = "skia-bindings/skia"
EXTERNALS_PREFIX = "third_party/externals/"
CHROMIUMDASH = "https://chromiumdash.appspot.com/fetch_milestones?mstone="
CHROMIUM_SRC = "https://chromium.googlesource.com/chromium/src/+/refs/branch-heads"

PLACEHOLDER = re.compile(r"\{(\w+)\}")


def fetch(url):
    with urllib.request.urlopen(url, timeout=60) as response:
        return response.read().decode()


def chromium_file(branch, path):
    """Return the contents of `path` in Chromium's source tree on `branch`."""
    return base64.b64decode(fetch(f"{CHROMIUM_SRC}/{branch}/{path}?format=TEXT")).decode()


def chromium_tree_id(branch, directory):
    """Return the git tree ID of `directory` in Chromium's source tree on `branch`."""
    parent, name = directory.rsplit("/", 1)
    # Gitiles prefixes its JSON responses with ")]}'" to prevent their use as scripts.
    listing = json.loads(fetch(f"{CHROMIUM_SRC}/{branch}/{parent}/?format=JSON")[4:])
    return next(entry["id"] for entry in listing["entries"] if entry["name"] == name)


def parse_deps(text):
    """Return a DEPS file as a dictionary, evaluated the way gclient evaluates it."""
    dictionary = {"Str": str}
    exec("def Var(x): return vars[x]\n" + text, dictionary)
    return dictionary


def repository_pins(deps):
    """Return a dictionary that maps each repository path of a DEPS file to its pinned commit.

    The repository path is the URL without its host and without a `.git` suffix, because
    Skia and Chromium fetch some repositories from different hosts.
    """
    variables = deps.get("vars", {})
    pins = {}
    for value in deps["deps"].values():
        url = value.get("url") if isinstance(value, dict) else value
        if not isinstance(url, str) or "@" not in url:
            continue
        url = PLACEHOLDER.sub(lambda m: str(variables.get(m.group(1), m.group(0))), url)
        repository, commit = url.rsplit("@", 1)
        pins[repository_path(repository)] = commit
    return pins


def repository_path(repository):
    path = re.sub(r"^[a-z]+://[^/]+", "", repository)
    return path.removesuffix(".git")


def git(checkout, *args):
    return subprocess.run(["git", "-C", checkout, *args], capture_output=True, text=True)


def has_commit(checkout, commit):
    return git(checkout, "cat-file", "-e", f"{commit}^{{commit}}").returncode == 0


def ensure_commit(checkout, commit):
    """Fetch `commit` into `checkout` unless it is already there."""
    if not has_commit(checkout, commit):
        git(checkout, "fetch", "--quiet", "origin", commit)
    return has_commit(checkout, commit)


def readme_field(branch, library, field):
    text = chromium_file(branch, f"third_party/{library}/README.chromium")
    return re.search(rf"^{field}:\s*(\S+)", text, re.MULTILINE).group(1)


def in_tree_pin(branch, library, checkout):
    """Return Chromium's pin for a library that is part of Chromium's own tree, or None."""
    if library == "libpng":
        version = readme_field(branch, "libpng", "Version")
        git(checkout, "fetch", "--quiet", "--tags", "origin")
        tag = git(checkout, "rev-parse", f"v{version}^{{commit}}")
        return tag.stdout.strip() if tag.returncode == 0 else None
    if library == "brotli":
        return readme_field(branch, "brotli", "Revision")
    if library == "zlib":
        tree = chromium_tree_id(branch, "third_party/zlib")
        git(checkout, "fetch", "--quiet", "origin")
        log = git(checkout, "log", "--all", "--format=%H %T").stdout.split("\n")
        return next((line.split()[0] for line in log if line.endswith(" " + tree)), None)
    return None


def verdict(checkout, ours, theirs):
    if ours == theirs:
        return "same"
    if not ensure_commit(checkout, theirs):
        return "Chromium's commit could not be fetched"
    if git(checkout, "merge-base", "--is-ancestor", ours, theirs).returncode == 0:
        return "Chromium's is newer"
    if git(checkout, "merge-base", "--is-ancestor", theirs, ours).returncode == 0:
        return "ours is newer"
    return "diverged, compare the release versions"


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    milestone = sys.argv[1]
    skia_path = sys.argv[2] if len(sys.argv) > 2 else DEFAULT_SKIA_PATH

    branch = json.loads(fetch(CHROMIUMDASH + milestone))[0]["chromium_branch"]
    print(f"Chromium m{milestone} is branch {branch}.")
    chromium_pins = repository_pins(parse_deps(chromium_file(branch, "DEPS")))

    with open(os.path.join(skia_path, "DEPS")) as f:
        skia_deps = parse_deps(f.read())
    for directory, value in sorted(skia_deps["deps"].items()):
        if not (directory.startswith(EXTERNALS_PREFIX) and isinstance(value, str) and "@" in value):
            continue
        library = directory[len(EXTERNALS_PREFIX):]
        checkout = os.path.join(skia_path, directory)
        repository, ours = value.rsplit("@", 1)

        theirs = chromium_pins.get(repository_path(repository))
        if theirs is None and os.path.isdir(checkout):
            theirs = in_tree_pin(branch, library, checkout)
        if theirs is None:
            print(f"{library}: ours {ours[:10]}, Chromium pins none")
            continue
        result = verdict(checkout, ours, theirs) if os.path.isdir(checkout) else "not checked out"
        print(f"{library}: ours {ours[:10]}, Chromium {theirs[:10]}, {result}")


if __name__ == "__main__":
    main()
