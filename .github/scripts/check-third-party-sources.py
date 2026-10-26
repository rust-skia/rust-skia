#!/usr/bin/env python3
"""Check that every source path in Skia's third-party build files exists in the pinned checkouts.

Skia's DEPS file pins the third-party libraries that `git-sync-deps` checks out into
`third_party/externals/<lib>`, and `third_party/<lib>/BUILD.gn` lists their source files. When a
pin moves to a newer commit, the library may have renamed or removed some of these files. A build
compiles only the current platform's branches of each `BUILD.gn`, so a stale path in another
platform's branch breaks only that platform's build, and a stale header path breaks no build.

This script checks the paths that point into a library the DEPS file pins, either written out as
"../externals/<lib>/..." or through a variable such as "$_src/...". The variable must be assigned
an "../externals/..." path in the same file. When a file assigns a variable more than once, a path
counts as present when any of the assignments resolves it. A variable that is appended to with
`+=` takes a value per platform and is not resolved, and neither is any other variable that
remains in a path, such as `$root_gen_dir`.

A library's checkout must be at the commit its DEPS entry pins, so that the check sees the files
of that commit. The script skips the libraries whose checkout is missing or at another commit and
names them, so they can be synced with `git-sync-deps` and checked again.

The script prints every missing path and exits with status 1 if there is one.

Usage: check-third-party-sources.py [path/to/skia]
"""

import glob
import os
import re
import subprocess
import sys

DEFAULT_SKIA_PATH = "skia-bindings/skia"
EXTERNALS_PREFIX = "third_party/externals/"

STRING = re.compile(r'"([^"]*)"')
EXTERNALS_ASSIGNMENT = re.compile(r'^\s*(\w+)\s*=\s*"(\.\./externals/[^"]*)"', re.MULTILINE)
APPEND = re.compile(r"^\s*(\w+)\s*\+=", re.MULTILINE)
VARIABLE_PATH = re.compile(r"^\$(\w+)(/.*)$")


def parse_deps(path):
    """Return the DEPS file as a dictionary, parsed the same way `git-sync-deps` parses it."""
    dictionary = {}
    with open(path) as f:
        exec("def Var(x): return vars[x]\n" + f.read(), dictionary)
    return dictionary


def pinned_libraries(skia_path):
    """Return the library directory names under `third_party/externals` that DEPS pins.

    Each name maps to whether its checkout is at the pinned commit.
    """
    deps = parse_deps(os.path.join(skia_path, "DEPS"))
    libraries = {}
    for directory, value in deps["deps"].items():
        if not (directory.startswith(EXTERNALS_PREFIX) and isinstance(value, str) and "@" in value):
            continue
        library = directory[len(EXTERNALS_PREFIX):]
        commit = value.split("@", 1)[1]
        head = subprocess.run(
            ["git", "-C", os.path.join(skia_path, directory), "rev-parse", "HEAD"],
            capture_output=True,
            text=True,
        )
        libraries[library] = head.returncode == 0 and head.stdout.strip() == commit
    return libraries


def external_paths(build_file):
    """Return every path in `build_file` that points into `third_party/externals`.

    Each path is returned as the list of its candidate spellings relative to the directory of
    `build_file`, one per assignment of the variable it starts with.
    """
    with open(build_file) as f:
        text = f.read()
    # Comments may quote patterns such as "wuffs-*.c", which are not paths.
    text = re.sub(r"#.*", "", text)

    appended = set(APPEND.findall(text))
    assignments = {}
    for name, value in EXTERNALS_ASSIGNMENT.findall(text):
        if name not in appended:
            assignments.setdefault(name, []).append(value)

    paths = []
    for string in STRING.findall(text):
        if string.startswith("../externals/"):
            candidates = [string]
        else:
            match = VARIABLE_PATH.match(string)
            if not (match and match.group(1) in assignments):
                continue
            name, rest = match.groups()
            candidates = [value + rest for value in assignments[name]]
        if "$" not in candidates[0]:
            paths.append(candidates)
    return paths


def main():
    skia_path = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_SKIA_PATH
    third_party_dir = os.path.join(skia_path, "third_party")
    libraries = pinned_libraries(skia_path)

    missing = 0
    checked = 0
    for build_file in sorted(glob.glob(os.path.join(third_party_dir, "*", "BUILD.gn"))):
        build_dir = os.path.dirname(build_file)
        for candidates in external_paths(build_file):
            library = candidates[0].split("/")[2]
            if not libraries.get(library):
                continue
            checked += 1
            if not any(os.path.exists(os.path.join(build_dir, c)) for c in candidates):
                missing += 1
                print(f"{os.path.relpath(build_file, skia_path)}: missing {candidates[0]}")

    unsynced = sorted(library for library, synced in libraries.items() if not synced)
    if unsynced:
        print(f"Skipped, not at their DEPS pin: {', '.join(unsynced)}")
    print(f"Checked {checked} paths into {len(libraries) - len(unsynced)} pinned libraries, "
          f"{missing} missing.")
    sys.exit(1 if missing else 0)


if __name__ == "__main__":
    main()
