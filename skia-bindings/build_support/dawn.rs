//! Generates the Dawn headers and `dawn_proc` source that the `wgpu` feature builds Skia's Dawn
//! backend with, and adds the backend to the `cc` build of `skia-bindings`.
//!
//! Skia's Dawn backend is compiled against Dawn's generated WebGPU headers, and bindgen reads the
//! same headers for the Rust types of the `skia-wgpu` shim (ADR 0003). Dawn's generator produces
//! them from Dawn's `src/dawn/dawn.json`, at the Dawn revision in `[package.metadata].dawn`. Only
//! source builds with the `wgpu` feature run this. They download Dawn and the Python packages of
//! its generator, so offline builds do not support the `wgpu` feature. Prebuilt binaries already
//! contain everything compiled from these files.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::build_support::cargo;

/// The paths of Dawn that generation reads. The download unpacks only these.
const SOURCE_PATHS: &[&str] = &[
    "generator",
    "src/dawn/dawn.json",
    "include/webgpu",
    "include/dawn/dawn_proc.h",
    "LICENSE",
];

/// The Python packages that Dawn's generator imports. Each one is downloaded at the revision in the
/// `[package.metadata]` entry of the same name, and the generator receives its directory as the
/// argument `--<name>-path`.
const PYTHON_PACKAGES: &[&str] = &["jinja2", "markupsafe"];

/// Generates the Dawn files into [`dir`], unless they are already there for these revisions.
///
/// `offline` builds panic, because generation downloads Dawn and the generator's Python packages.
pub fn prepare(python: &Path, offline: bool) {
    assert!(
        !offline,
        "Offline builds do not support the `wgpu` feature, because it downloads Dawn and the \
         Python packages of Dawn's generator."
    );

    let revisions = Revisions::from_metadata();
    let stamp = revisions.stamp();
    let stamp_file = dir().join("REVISION");
    if fs::read_to_string(&stamp_file).is_ok_and(|s| s == stamp) {
        return;
    }

    let source_dir = cargo::output_directory().join("dawn-source");
    download(
        &format!(
            "https://codeload.github.com/google/dawn/tar.gz/{}",
            revisions.dawn
        ),
        // GitHub names the root directory of the archive `<repository>-<revision>`.
        &format!("dawn-{}", revisions.dawn),
        |path| SOURCE_PATHS.iter().any(|p| path.starts_with(p)),
        &source_dir,
    );

    // Dawn's generator imports each package from the directory that its `--<name>-path` argument
    // names, so each package is unpacked into a directory of its own name.
    let packages_dir = cargo::output_directory().join("dawn-python");
    for (name, revision) in &revisions.python_packages {
        download(
            &format!(
                "https://chromium.googlesource.com/chromium/src/third_party/{name}/+archive/{revision}.tar.gz"
            ),
            // Gitiles archives have no root directory.
            "",
            |_| true,
            &packages_dir.join(name),
        );
    }

    let generated = cargo::output_directory().join("dawn-generated");
    generate(
        &source_dir,
        &revisions.dawn,
        &packages_dir,
        python,
        &generated,
    );
    assemble(&source_dir, &generated, &dir());
    fs::write(stamp_file, stamp).unwrap();
}

/// Adds Skia's Dawn backend and `dawn_proc.cpp` to the `cc` build of `skia-bindings`.
///
/// Skia's GN build compiles the Dawn backend only together with all of Dawn, which it builds with
/// CMake. `skia-bindings` therefore leaves `skia_use_dawn` off and compiles the backend's sources
/// itself, with the same defines as its own bindings. No Skia source outside the backend depends
/// on `SK_DAWN`. The caller adds [`include_dir`] for both `cc` and bindgen.
pub fn add_backend_sources(cc_build: &mut cc::Build, skia_source_dir: &Path) {
    for source in backend_sources(skia_source_dir) {
        cc_build.file(source);
    }
    cc_build.file(dir().join("src/dawn_proc.cpp"));
    // The replacements for the Dawn internals that `dawn_proc.cpp` includes. Only that file
    // includes them, and their paths (`src/utils/assert.h`, ...) exist nowhere in Skia, so they
    // cannot shadow a Skia header. Cargo runs build scripts in the package's root directory.
    let compat_dir = std::env::current_dir().unwrap().join("dawn/compat");
    cargo::rerun_if_file_changed(&compat_dir);
    cc_build.include(compat_dir);
    // Skia's internal headers leave many parameters unused, and Skia's own build turns this
    // warning off. `cc` enables it with `-Wall -Wextra` (`/W4` on MSVC) for the whole build.
    cc_build.flag_if_supported("-Wno-unused-parameter");
    cc_build.flag_if_supported("/wd4100");
}

/// The include directory of the Dawn headers (`webgpu/webgpu.h`, `dawn/dawn_proc.h`, ...).
pub fn include_dir() -> PathBuf {
    dir().join("include")
}

/// The directory that [`prepare`] fills: `include/` and `src/dawn_proc.cpp`.
pub fn dir() -> PathBuf {
    cargo::output_directory().join("dawn")
}

/// The `.cpp` files of `skia_graphite_dawn_sources` in Skia's `gn/graphite.gni`, the list that
/// Skia's GN build compiles when `skia_use_dawn` is on.
fn backend_sources(skia_source_dir: &Path) -> Vec<PathBuf> {
    let gni = fs::read_to_string(skia_source_dir.join("gn/graphite.gni")).unwrap();
    let sources: Vec<PathBuf> = gni
        .lines()
        .skip_while(|line| *line != "skia_graphite_dawn_sources = [")
        .skip(1)
        .take_while(|line| *line != "]")
        .map(|line| line.trim().trim_end_matches(',').trim_matches('"'))
        .filter(|path| path.ends_with(".cpp"))
        .map(|path| {
            let path = path
                .strip_prefix("$_src/")
                .unwrap_or_else(|| panic!("Unexpected Dawn source path in graphite.gni: {path}"));
            skia_source_dir.join("src").join(path)
        })
        .collect();
    assert!(
        !sources.is_empty(),
        "`skia_graphite_dawn_sources` not found in gn/graphite.gni"
    );
    sources
}

/// The revisions of Dawn and of the [`PYTHON_PACKAGES`], from `[package.metadata]`. Skia's
/// upstream `DEPS` pins them for the current milestone, and the milestone update copies them.
struct Revisions {
    dawn: String,
    /// Each name of [`PYTHON_PACKAGES`] with its revision.
    python_packages: Vec<(&'static str, String)>,
}

impl Revisions {
    fn from_metadata() -> Self {
        let metadata = cargo::get_metadata();
        let revision = |name: &str| {
            metadata
                .iter()
                .find_map(|(key, value)| (key == name).then(|| value.clone()))
                .unwrap_or_else(|| {
                    panic!("`[package.metadata].{name}` is missing in skia-bindings/Cargo.toml")
                })
        };
        Self {
            dawn: revision("dawn"),
            python_packages: PYTHON_PACKAGES
                .iter()
                .map(|name| (*name, revision(name)))
                .collect(),
        }
    }

    /// Identifies the revisions, so that changing any of them generates again.
    fn stamp(&self) -> String {
        let mut stamp = self.dawn.clone();
        for (name, revision) in &self.python_packages {
            stamp += &format!(" {name}@{revision}");
        }
        stamp
    }
}

/// Downloads the archive at `url` and unpacks the files of its `root` directory that `include`
/// selects into `target`.
fn download(url: &str, root: &str, include: impl Fn(&Path) -> bool, target: &Path) {
    #[cfg(feature = "binary-cache")]
    {
        crate::build_support::binary_cache::download_archive(url, root, include, target);
    }
    #[cfg(not(feature = "binary-cache"))]
    {
        let _ = (root, include, target);
        panic!("The `wgpu` feature needs the `binary-cache` feature to download {url}.");
    }
}

/// Runs Dawn's generators for the C header, the C++ wrapper, the function table, the forwarding
/// functions of `dawn_proc`, and the revision stamp that `dawnProcSetProcs` checks.
///
/// `packages_dir` contains a directory for each of the [`PYTHON_PACKAGES`].
fn generate(source_dir: &Path, revision: &str, packages_dir: &Path, python: &Path, out: &Path) {
    if out.exists() {
        fs::remove_dir_all(out).unwrap();
    }
    fs::create_dir_all(out).unwrap();

    let generator = source_dir.join("generator");
    let mut common_args: Vec<OsString> = vec![
        "--template-dir".into(),
        generator.join("templates").into(),
        "--output-dir".into(),
        out.into(),
    ];
    for name in PYTHON_PACKAGES {
        common_args.push(format!("--{name}-path").into());
        common_args.push(packages_dir.join(name).into());
    }

    run_python(
        python,
        &generator.join("dawn_json_generator.py"),
        &common_args,
        &[
            "--dawn-json".into(),
            source_dir.join("src/dawn/dawn.json").into(),
            "--targets".into(),
            "headers,cpp_headers,proc".into(),
        ],
    );

    // The version generator reads the revision from git, unless a version file provides it. A
    // download has no git metadata, so the file always provides it.
    let version_file = out.join("dawn-version.txt");
    fs::write(&version_file, revision).unwrap();
    run_python(
        python,
        &generator.join("dawn_version_generator.py"),
        &common_args,
        &[
            "--dawn-dir".into(),
            source_dir.into(),
            "--version-file".into(),
            version_file.into(),
        ],
    );
}

fn run_python(python: &Path, script: &Path, common_args: &[OsString], args: &[OsString]) {
    let status = Command::new(python)
        .arg(script)
        .args(common_args)
        .args(args)
        .status()
        .unwrap();
    assert!(status.success(), "{} failed", script.display());
}

/// Where each file of [`dir`] comes from: the Dawn source or the generator output.
enum Origin {
    Source,
    Generated,
}

/// The files of [`dir`], as origin, path in the origin, and path in [`dir`].
const FILES: &[(Origin, &str, &str)] = &[
    // Dawn's static headers. The ones in `include/webgpu` forward to the generated ones in
    // `include/dawn`.
    (
        Origin::Source,
        "include/webgpu/webgpu.h",
        "include/webgpu/webgpu.h",
    ),
    (
        Origin::Source,
        "include/webgpu/webgpu_cpp.h",
        "include/webgpu/webgpu_cpp.h",
    ),
    (
        Origin::Source,
        "include/webgpu/webgpu_enum_class_bitmasks.h",
        "include/webgpu/webgpu_enum_class_bitmasks.h",
    ),
    (
        Origin::Source,
        "include/dawn/dawn_proc.h",
        "include/dawn/dawn_proc.h",
    ),
    (Origin::Source, "LICENSE", "LICENSE"),
    (
        Origin::Generated,
        "include/webgpu/webgpu_cpp_chained_struct.h",
        "include/webgpu/webgpu_cpp_chained_struct.h",
    ),
    (
        Origin::Generated,
        "include/dawn/webgpu.h",
        "include/dawn/webgpu.h",
    ),
    (
        Origin::Generated,
        "include/dawn/webgpu_cpp.h",
        "include/dawn/webgpu_cpp.h",
    ),
    (
        Origin::Generated,
        "include/dawn/dawn_proc_table.h",
        "include/dawn/dawn_proc_table.h",
    ),
    (
        Origin::Generated,
        "include/dawn/dawn_version.h",
        "include/dawn/dawn_version.h",
    ),
    (
        Origin::Generated,
        "src/dawn/dawn_proc.cpp",
        "src/dawn_proc.cpp",
    ),
];

fn assemble(source_dir: &Path, generated: &Path, target: &Path) {
    if target.exists() {
        fs::remove_dir_all(target).unwrap();
    }
    for (origin, from, to) in FILES {
        let from_dir = match origin {
            Origin::Source => source_dir,
            Origin::Generated => generated,
        };
        let to = target.join(to);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(from_dir.join(from), &to)
            .unwrap_or_else(|e| panic!("Failed to copy {from} to {}: {e}", to.display()));
    }
}
