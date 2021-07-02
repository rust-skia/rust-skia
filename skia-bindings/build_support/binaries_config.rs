use crate::build_support::{android, cargo, features, ios};
use std::path::PathBuf;

/// The libraries to link with.
pub mod lib {
    pub const SKIA: &str = "skia";
    pub const SKIA_BINDINGS: &str = "skia-bindings";
    pub const SK_SHAPER: &str = "skshaper";
    pub const SK_PARAGRAPH: &str = "skparagraph";
}

/// The configuration of the resulting binaries.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BinariesConfiguration {
    /// The feature identifiers we built with.
    pub feature_ids: Vec<String>,

    /// The output directory of the libraries we build and we need to inform cargo about.
    pub output_directory: PathBuf,

    /// The TARGET specific link libraries we need to inform cargo about.
    pub link_libraries: Vec<String>,

    /// The static Skia libraries built by ninja that dependent projects need to link with.
    pub ninja_built_libraries: Vec<String>,

    /// Other static Skia libraries skia-bindings provides that dependent projects need to link with.
    pub other_built_libraries: Vec<String>,

    /// Additional files relative to the output_directory
    /// that are needed to build dependent projects.
    pub additional_files: Vec<PathBuf>,

    /// `true` if the skia libraries are built with debugging information.
    pub skia_debug: bool,
}

const SKIA_OUTPUT_DIR: &str = "skia";
const ICUDTL_DAT: &str = "icudtl.dat";

impl BinariesConfiguration {
    /// Build a binaries configuration from a set of Skia features.
    pub fn from_features(features: &features::Features, skia_debug: bool) -> Self {
        let target = cargo::target();

        let mut ninja_built_libraries = Vec::new();
        let mut other_built_libraries = Vec::new();
        let mut additional_files = Vec::new();
        let feature_ids = features.ids();

        if features.text_layout {
            if target.is_windows() {
                additional_files.push(ICUDTL_DAT.into());
            }
            ninja_built_libraries.push(lib::SK_PARAGRAPH.into());
            ninja_built_libraries.push(lib::SK_SHAPER.into());
        }

        let mut link_libraries = Vec::new();

        match target.as_strs() {
            (_, "unknown", "linux", _) => {
                link_libraries.extend(vec!["stdc++", "fontconfig", "freetype"]);
                if features.gl {
                    if features.egl {
                        link_libraries.push("EGL");
                    }

                    if features.x11 {
                        link_libraries.push("GL");
                    }

                    if features.wayland {
                        link_libraries.push("wayland-egl");
                        link_libraries.push("GLESv2");
                    }
                }
            }
            (_, "apple", "darwin", _) => {
                link_libraries.extend(vec!["c++", "framework=ApplicationServices"]);
                if features.gl {
                    link_libraries.push("framework=OpenGL");
                }
                if features.metal {
                    link_libraries.push("framework=Metal");
                    // MetalKit was added in m87 BUILD.gn.
                    link_libraries.push("framework=MetalKit");
                    link_libraries.push("framework=Foundation");
                }
            }
            (_, _, "windows", Some("msvc")) => {
                link_libraries.extend(&["usp10", "ole32", "user32", "gdi32", "fontsub"]);
                if features.gl {
                    link_libraries.push("opengl32");
                }
                if features.d3d {
                    link_libraries.extend(&["d3d12", "dxgi", "d3dcompiler"]);
                }
            }
            (_, "linux", "android", _) | (_, "linux", "androideabi", _) => {
                link_libraries.extend(android::link_libraries(features));
            }
            (_, "apple", "ios", _) => {
                link_libraries.extend(ios::link_libraries(features));
            }
            _ => panic!("unsupported target: {:?}", cargo::target()),
        };

        let output_directory = cargo::output_directory()
            .join(SKIA_OUTPUT_DIR)
            .to_str()
            .unwrap()
            .into();

        ninja_built_libraries.push(lib::SKIA.into());
        other_built_libraries.push(lib::SKIA_BINDINGS.into());

        BinariesConfiguration {
            feature_ids: feature_ids.into_iter().map(|f| f.to_string()).collect(),
            output_directory,
            link_libraries: link_libraries
                .into_iter()
                .map(|lib| lib.to_string())
                .collect(),
            ninja_built_libraries,
            other_built_libraries,
            additional_files,
            skia_debug,
        }
    }

    pub fn built_libraries(&self) -> impl Iterator<Item = &str> {
        self.ninja_built_libraries
            .iter()
            .chain(self.other_built_libraries.iter())
            .map(|x| x.as_str())
    }

    /// Inform cargo that the library files of the given configuration are available and
    /// can be used as dependencies.
    pub fn commit_to_cargo(&self) {
        cargo::add_link_search(self.output_directory.to_str().unwrap());

        // On Linux, the order is significant, first the static libraries we built, and then
        // the system libraries.

        let target = cargo::target();

        cargo::add_static_link_libs(&target, self.built_libraries());
        cargo::add_link_libs(&self.link_libraries);
    }
}
