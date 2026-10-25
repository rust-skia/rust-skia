#!/usr/bin/env python3
"""Check the third-party repositories pinned in Skia's DEPS file against OSV.dev.

Skia's DEPS file pins the C and C++ libraries that `git-sync-deps` checks out and the build
compiles, for example libpng, libwebp, freetype, and harfbuzz. Each pin is a git commit. Cargo
tooling such as `cargo deny` cannot see these pins, because they are not Cargo dependencies.

OSV.dev (https://osv.dev) is a public vulnerability database that maps git commits to the
vulnerabilities that affect them. This script sends every pinned commit to OSV.dev, prints each
known vulnerability, and exits with status 1 if any were found that the ignore file does not
list.

OSV.dev finds a vulnerability by commit only when its record lists the affected git commits or
ranges. A record that names only release versions matches no commit, so the script cannot see
it. A pin without results is therefore free of the vulnerabilities that OSV.dev maps to commits,
not of all known ones.

The ignore file holds one vulnerability per line: its OSV ID or one of its aliases (such as a
CVE ID), followed by the reason it is accepted. Lines starting with `#` are comments. The
script reports ignore entries that no longer match any vulnerability, so they can be removed.

Usage: audit-skia-deps.py [path/to/DEPS [path/to/ignore-file]]
"""

import json
import sys
import urllib.request

DEFAULT_DEPS_PATH = "skia-bindings/skia/DEPS"
DEFAULT_IGNORE_PATH = ".github/scripts/audit-skia-deps.ignore"
OSV_API = "https://api.osv.dev/v1"


def parse_deps(path):
    """Return the DEPS file as a dictionary, parsed the same way `git-sync-deps` parses it."""
    dictionary = {}
    with open(path) as f:
        exec("def Var(x): return vars[x]\n" + f.read(), dictionary)
    return dictionary


def pinned_commits(deps):
    """Return (directory, repository, commit) for every git dependency `git-sync-deps` syncs.

    `git-sync-deps` only syncs entries whose value is a "repository@commit" string. It skips
    dictionary entries, which are CIPD packages or conditional checkouts.
    """
    pins = []
    for directory, value in sorted(deps["deps"].items()):
        if isinstance(value, str) and "@" in value:
            repository, commit = value.split("@", 1)
            pins.append((directory, repository, commit))
    return pins


def parse_ignores(path):
    """Return a dictionary that maps each ignored vulnerability ID to the reason given for it."""
    ignores = {}
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#"):
                id, _, reason = line.partition(" ")
                ignores[id] = reason.strip()
    return ignores


def post(endpoint, body):
    request = urllib.request.Request(
        f"{OSV_API}/{endpoint}",
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(request, timeout=60) as response:
        return json.load(response)


def get(endpoint):
    with urllib.request.urlopen(f"{OSV_API}/{endpoint}", timeout=60) as response:
        return json.load(response)


def main():
    deps_path = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_DEPS_PATH
    ignore_path = sys.argv[2] if len(sys.argv) > 2 else DEFAULT_IGNORE_PATH
    pins = pinned_commits(parse_deps(deps_path))
    ignores = parse_ignores(ignore_path)
    matched_ignores = set()

    # The batch endpoint returns one result per query, in query order, with vulnerability IDs only.
    results = post("querybatch", {"queries": [{"commit": commit} for _, _, commit in pins]})

    failures = 0
    for (directory, repository, commit), result in zip(pins, results["results"]):
        ids = [vuln["id"] for vuln in result.get("vulns", [])]
        if not ids:
            continue
        print(f"{directory} @ {commit}\n  {repository}")
        for id in ids:
            details = get(f"vulns/{id}")
            aliases = details.get("aliases", [])
            # Many CVE records have no summary, so the start of the details text stands in for it.
            summary = details.get("summary") or details.get("details", "").split("\n")[0][:120]
            print(f"  {id}" + (f" ({', '.join(aliases)})" if aliases else "") + f": {summary}")
            print(f"    https://osv.dev/vulnerability/{id}")
            ignored = [key for key in [id, *aliases] if key in ignores]
            if ignored:
                matched_ignores.update(ignored)
                print(f"    ignored: {ignores[ignored[0]]}")
            else:
                failures += 1

    for id in sorted(set(ignores) - matched_ignores):
        print(f"Stale ignore entry, no pinned commit is affected by it anymore: {id}")

    print(
        f"Checked {len(pins)} pinned commits from {deps_path}, "
        f"{failures} known vulnerabilities not ignored."
    )
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
