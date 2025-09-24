use crate::{
    Features, HostOS, Job, TargetConf, Workflow, WorkflowKind, LINUX_JOB, MACOS_JOB,
    WINDOWS_ARM_JOB, WINDOWS_JOB,
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
    }
    workflows
}

pub fn jobs(workflow: &Workflow) -> Vec<Job> {
    match workflow.kind {
        WorkflowKind::QA => qa_jobs(),
        WorkflowKind::Release => release_jobs(workflow),
    }
}

pub fn qa_jobs() -> Vec<Job> {
    const QA_ALL_FEATURES: &str = "gl,vulkan,textlayout,svg,ureq,webp";
    [
        Job {
            name: "stable-all-features".into(),
            toolchain: "stable",
            features: QA_ALL_FEATURES.into(),
            example_args: Some("--driver cpu --driver pdf --driver svg".into()),
            ..Job::default()
        },
        /*
        Job {
            name: "stable-all-features-debug",
            toolchain: "stable",
            features: QA_ALL_FEATURES.into(),
            skia_debug: true,
            ..Job::default()
        },
        */
        Job {
            name: "beta-all-features".into(),
            toolchain: "beta",
            features: QA_ALL_FEATURES.into(),
            ..Job::default()
        },
    ]
    .into()
}

/// Jobs for releasing prebuilt binaries.
pub fn release_jobs(workflow: &Workflow) -> Vec<Job> {
    let mut jobs: Vec<_> = [
        release_job(""),
        release_job("gl"),
        release_job("vulkan"),
        release_job("textlayout"),
        release_job("gl,textlayout"),
        release_job("vulkan,textlayout"),
        release_job("gl,vulkan,textlayout"),
    ]
    .into();

    match workflow.host_os {
        HostOS::Windows | HostOS::WindowsArm => {
            jobs.push(release_job("d3d"));
            jobs.push(release_job("d3d,textlayout"));
            jobs.push(release_job("d3d,gl,textlayout"));
        }
        HostOS::Linux => {
            jobs.push(release_job("gl,x11"));
            jobs.push(release_job("gl,textlayout,x11"));
            // Full feature set: See skia-safe/Cargo.toml all-linux
            jobs.push(release_job("gl,egl,x11,wayland,vulkan,textlayout,svg,webp"))
        }
        HostOS::MacOS => {
            jobs.push(release_job("metal"));
            jobs.push(release_job("metal,textlayout"));
            jobs.push(release_job("metal,gl,textlayout"));
        }
    }

    jobs.extend(freya_release_jobs(workflow));
    jobs.extend(vizia_release_jobs(workflow));
    jobs.extend(skia_canvas_release_jobs(workflow));
    jobs.extend(grida_canvas_release_jobs(workflow));

    jobs
}

/// Specific binary releases for the Freya GUI library <https://github.com/marc2332/freya>
/// <https://github.com/rust-skia/rust-skia/issues/706>
fn freya_release_jobs(workflow: &Workflow) -> Vec<Job> {
    match workflow.host_os {
        HostOS::Windows | HostOS::MacOS => {
            vec![release_job("gl,textlayout,svg")]
        }
        HostOS::WindowsArm => {
            vec![]
        }
        HostOS::Linux => {
            vec![
                release_job("gl,textlayout,svg,x11"),
                // <https://github.com/rust-skia/rust-skia/issues/737>
                release_job("gl,textlayout,svg,wayland,x11"),
            ]
        }
    }
}

/// Specific binary releases for the Vizia GUI library <https://github.com/vizia/vizia>
/// <https://github.com/rust-skia/rust-skia/discussions/961#discussioncomment-10485430>
fn vizia_release_jobs(workflow: &Workflow) -> Vec<Job> {
    match workflow.host_os {
        HostOS::MacOS => {
            vec![release_job("gl,vulkan,textlayout,svg")]
        }
        HostOS::Windows => {
            vec![release_job("gl,vulkan,textlayout,svg,d3d")]
        }
        HostOS::WindowsArm => {
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
fn skia_canvas_release_jobs(workflow: &Workflow) -> Vec<Job> {
    match workflow.host_os {
        HostOS::MacOS => {
            vec![
                release_job("textlayout,webp,svg"),
                release_job("metal,textlayout,webp,svg"),
            ]
        }
        HostOS::Windows => {
            vec![release_job("vulkan,textlayout,webp,svg")]
        }
        HostOS::WindowsArm => {
            vec![release_job("vulkan,textlayout,webp,svg")]
        }
        HostOS::Linux => {
            vec![release_job("vulkan,textlayout,webp,svg")]
        }
    }
}

// <https://github.com/rust-skia/rust-skia/issues/1205>
//
// This is actually only used for the wasm32-unknown-enscripten target. But right now we
// can't be this specific.
fn grida_canvas_release_jobs(workflow: &Workflow) -> Vec<Job> {
    match workflow.host_os {
        HostOS::Linux => {
            vec![release_job("gl,textlayout,svg")]
        }
        _ => Vec::new(),
    }
}

fn release_job(features: impl Into<Features>) -> Job {
    let features = features.into();
    let name = {
        let name = features.name("-");
        if !name.is_empty() {
            format!("release-{name}")
        } else {
            "release".into()
        }
    };
    Job {
        name,
        toolchain: "stable",
        features,
        ..Job::default()
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
    targets.extend(wasm_targets());
    targets
}

fn macos_targets() -> Vec<TargetConf> {
    vec![
        TargetConf::new("aarch64-apple-darwin", "metal"),
        TargetConf::new("x86_64-apple-darwin", "metal"),
        TargetConf::new("aarch64-apple-ios", "metal"),
        TargetConf::new("aarch64-apple-ios-sim", "metal"),
        TargetConf::new("x86_64-apple-ios", "metal"),
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
        TargetConf::new("aarch64-linux-android", ""),
        TargetConf::new("x86_64-linux-android", ""),
        TargetConf::new("i686-linux-android", ""),
    ]
    .into()
}

fn wasm_targets() -> Vec<TargetConf> {
    // Compiling ureq-proto v0.3.0
    //   error[E0277]: the trait bound `SystemRandom: ring::rand::SecureRandom` is not satisfied
    [TargetConf::new("wasm32-unknown-emscripten", "").disable("ureq,x11,wayland,vulkan")].into()
}
