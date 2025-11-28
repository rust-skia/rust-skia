use crate::{
    Features, HostOS, Job, JobFeatures, TargetConf, Workflow, WorkflowKind, LINUX_JOB, MACOS_JOB,
    WASM_JOB, WINDOWS_ARM_JOB, WINDOWS_JOB,
};

pub const DEFAULT_ANDROID_API_LEVEL: usize = 26;

pub fn workflows() -> Vec<Workflow> {
    let mut workflows = Vec::new();
    for kind in &[WorkflowKind::QA, WorkflowKind::Release] {
        let kind = *kind;
        workflows.push(Workflow {
            kind,
            host_os: HostOS::Windows,
            host_target: "x86_64-pc-windows-msvc",
            job_template: WINDOWS_JOB,
            targets: windows_targets(),
            host_bin_ext: ".exe",
        });
        workflows.push(Workflow {
            kind,
            host_os: HostOS::WindowsArm,
            host_target: "aarch64-pc-windows-msvc",
            job_template: WINDOWS_ARM_JOB,
            targets: windows_arm_targets(),
            host_bin_ext: ".exe",
        });
        workflows.push(Workflow {
            kind,
            host_os: HostOS::Linux,
            host_target: "x86_64-unknown-linux-gnu",
            job_template: LINUX_JOB,
            targets: linux_targets(),
            host_bin_ext: "",
        });
        workflows.push(Workflow {
            kind,
            host_os: HostOS::MacOS,
            host_target: "aarch64-apple-darwin",
            job_template: MACOS_JOB,
            targets: macos_targets(),
            host_bin_ext: "",
        });
        workflows.push(Workflow {
            kind,
            host_os: HostOS::Wasm,
            host_target: "wasm32-unknown-emscripten",
            job_template: WASM_JOB,
            targets: wasm_targets(),
            host_bin_ext: "",
        });
    }
    workflows
}

pub fn jobs(workflow: &Workflow) -> Vec<Job> {
    match workflow.kind {
        WorkflowKind::QA => qa_jobs(workflow),
        WorkflowKind::Release => release_jobs(workflow),
    }
}

pub fn qa_jobs(workflow: &Workflow) -> Vec<Job> {
    match workflow.host_os {
        HostOS::Wasm => {
            // WASM QA: Use features that work with WASM (no vulkan, ureq)
            vec![Job {
                name: "stable-all-features".into(),
                toolchain: "stable",
                features: JobFeatures::Direct("gl,textlayout,svg,webp".into()),
                ..Job::default()
            }]
        }
        _ => {
            const QA_ALL_FEATURES: &str = "gl,vulkan,textlayout,svg,ureq,webp";
            vec![Job {
                name: "stable-all-features".into(),
                toolchain: "stable",
                features: JobFeatures::Direct(QA_ALL_FEATURES.into()),
                example_args: Some("--driver cpu --driver pdf --driver svg".into()),
                ..Job::default()
            }]
        }
    }
}

/// Jobs for releasing prebuilt binaries.
pub fn release_jobs(workflow: &Workflow) -> Vec<Job> {
    let mut features: Vec<Features> = if workflow.host_os == HostOS::Wasm {
        // WASM: Only features that work (no vulkan, ureq, x11, wayland)
        vec![
            "".into(),
            "gl".into(),
            "textlayout".into(),
            "gl,textlayout".into(),
        ]
    } else {
        [
            "",
            "gl",
            "vulkan",
            "textlayout",
            "gl,textlayout",
            "vulkan,textlayout",
            "gl,vulkan,textlayout",
        ]
        .iter()
        .map(|s| (*s).into())
        .collect()
    };

    match workflow.host_os {
        HostOS::Windows | HostOS::WindowsArm => {
            features.extend_from_slice(&[
                "d3d".into(),
                "d3d,textlayout".into(),
                "d3d,gl,textlayout".into(),
            ]);
        }
        HostOS::Linux => {
            features.extend_from_slice(&[
                "gl,x11".into(),
                "gl,textlayout,x11".into(),
                // Full feature set: See skia-safe/Cargo.toml all-linux
                "gl,egl,x11,wayland,vulkan,textlayout,svg,webp".into(),
            ]);
        }
        HostOS::MacOS => {
            features.extend_from_slice(&[
                "metal".into(),
                "metal,textlayout".into(),
                "metal,gl,textlayout".into(),
            ]);
        }
        HostOS::Wasm => {
            // WASM-specific features added via grida_canvas_release_features
        }
    }

    features.extend(freya_release_features(workflow));
    features.extend(vizia_release_features(workflow));
    features.extend(skia_canvas_release_features(workflow));
    features.extend(grida_canvas_release_features(workflow));

    features.sort();
    features.dedup();

    vec![Job {
        name: "release".into(),
        toolchain: "stable",
        features: JobFeatures::Matrix(features),
        ..Job::default()
    }]
}

/// Specific binary releases for the Freya GUI library <https://github.com/marc2332/freya>
/// <https://github.com/rust-skia/rust-skia/issues/706>
fn freya_release_features(workflow: &Workflow) -> Vec<Features> {
    match workflow.host_os {
        HostOS::Windows | HostOS::MacOS => {
            vec!["gl,textlayout,svg".into()]
        }
        HostOS::WindowsArm | HostOS::Wasm => {
            vec![]
        }
        HostOS::Linux => {
            vec![
                "gl,textlayout,svg,x11".into(),
                // <https://github.com/rust-skia/rust-skia/issues/737>
                "gl,textlayout,svg,wayland,x11".into(),
            ]
        }
    }
}

/// Specific binary releases for the Vizia GUI library <https://github.com/vizia/vizia>
/// <https://github.com/rust-skia/rust-skia/discussions/961#discussioncomment-10485430>
fn vizia_release_features(workflow: &Workflow) -> Vec<Features> {
    match workflow.host_os {
        HostOS::MacOS => {
            vec!["gl,vulkan,textlayout,svg".into()]
        }
        HostOS::Windows => {
            vec!["gl,vulkan,textlayout,svg,d3d".into()]
        }
        HostOS::WindowsArm | HostOS::Wasm => {
            vec![]
        }
        HostOS::Linux => {
            // vec![release_job("gl,vulkan,textlayout,svg,wayland,x11")]
            // Alternative: Use the full feature set `gl,vulkan,textlayout,svg,wayland,x11,webp`
            vec![]
        }
    }
}

// Binaries for Skia Canvas: <https://github.com/samizdatco/skia-canvas>
// <https://github.com/rust-skia/rust-skia/pull/1068#issuecomment-2518894492>
fn skia_canvas_release_features(workflow: &Workflow) -> Vec<Features> {
    match workflow.host_os {
        HostOS::MacOS => {
            vec![
                "textlayout,webp,svg".into(),
                "metal,textlayout,webp,svg".into(),
            ]
        }
        HostOS::Windows => {
            vec!["vulkan,textlayout,webp,svg".into()]
        }
        HostOS::WindowsArm => {
            vec!["vulkan,textlayout,webp,svg".into()]
        }
        HostOS::Linux => {
            vec!["vulkan,textlayout,webp,svg".into()]
        }
        HostOS::Wasm => {
            vec![]
        }
    }
}

// <https://github.com/rust-skia/rust-skia/issues/1205>
fn grida_canvas_release_features(workflow: &Workflow) -> Vec<Features> {
    match workflow.host_os {
        HostOS::Wasm => {
            vec!["gl,textlayout,svg,webp".into()]
        }
        _ => Vec::new(),
    }
}

fn windows_targets() -> Vec<TargetConf> {
    [TargetConf::new("x86_64-pc-windows-msvc", "d3d")].into()
}

fn windows_arm_targets() -> Vec<TargetConf> {
    [TargetConf::new("aarch64-pc-windows-msvc", "d3d")].into()
}

fn linux_targets() -> Vec<TargetConf> {
    let mut targets = vec![TargetConf::new(
        "x86_64-unknown-linux-gnu",
        "egl,x11,wayland",
    )];
    targets.extend(linux_aarch64_targets());
    targets.extend(android_targets());
    targets
}

fn macos_targets() -> Vec<TargetConf> {
    vec![
        TargetConf::new("aarch64-apple-darwin", "metal"),
        TargetConf::new("x86_64-apple-darwin", "metal"),
        // iOS: Vulkan is not supported ("No Vulkan support on iOS yet" in Skia)
        TargetConf::new("aarch64-apple-ios", "metal").disable("vulkan"),
        TargetConf::new("aarch64-apple-ios-sim", "metal").disable("vulkan"),
        TargetConf::new("x86_64-apple-ios", "metal").disable("vulkan"),
    ]
}

fn linux_aarch64_targets() -> Vec<TargetConf> {
    [TargetConf::new(
        "aarch64-unknown-linux-gnu",
        "egl,x11,wayland",
    )]
    .into()
}

fn android_targets() -> Vec<TargetConf> {
    [
        TargetConf::new("aarch64-linux-android", "").disable("egl,x11,wayland"),
        TargetConf::new("x86_64-linux-android", "").disable("egl,x11,wayland"),
        TargetConf::new("i686-linux-android", "").disable("egl,x11,wayland"),
    ]
    .into()
}

pub fn wasm_targets() -> Vec<TargetConf> {
    // Compiling ureq-proto v0.3.0
    //   error[E0277]: the trait bound `SystemRandom: ring::rand::SecureRandom` is not satisfied
    [TargetConf::new("wasm32-unknown-emscripten", "").disable("ureq,x11,wayland,vulkan")].into()
}
