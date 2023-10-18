use crate::{
    Features, HostOS, Job, Target, Workflow, WorkflowKind, LINUX_JOB, MACOS_JOB, WINDOWS_JOB,
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
            host_os: HostOS::Linux,
            host_target: "x86_64-unknown-linux-gnu",
            job_template: LINUX_JOB,
            targets: linux_targets(),
            host_bin_ext: "",
        });
        workflows.push(Workflow {
            kind,
            host_os: HostOS::MacOS,
            host_target: "x86_64-apple-darwin",
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
    const QA_ALL_FEATURES: &str = "gl,vulkan,textlayout,svg,webp";
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
        HostOS::Windows => {
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
        }
    }

    jobs.extend(freya_release_jobs(workflow));

    jobs
}

/// Specific binary releases for the Freya GUI library <https://github.com/marc2332/freya>
/// <https://github.com/rust-skia/rust-skia/issues/706>
fn freya_release_jobs(workflow: &Workflow) -> Vec<Job> {
    match workflow.host_os {
        HostOS::Windows | HostOS::MacOS => {
            vec![release_job("gl,textlayout,svg")]
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

fn windows_targets() -> Vec<Target> {
    [Target {
        target: "x86_64-pc-windows-msvc",
        platform_features: "d3d".into(),
        ..Target::default()
    }]
    .into()
}

fn linux_targets() -> Vec<Target> {
    let mut targets = vec![Target {
        target: "x86_64-unknown-linux-gnu",
        platform_features: "egl,x11,wayland".into(),
        ..Default::default()
    }];
    targets.extend(android_targets());
    targets.extend(wasm_targets());
    targets
}

fn macos_targets() -> Vec<Target> {
    vec![
        Target {
            target: "x86_64-apple-darwin",
            platform_features: "metal".into(),
            ..Default::default()
        },
        Target {
            target: "aarch64-apple-darwin",
            platform_features: "metal".into(),
            ..Default::default()
        },
        Target {
            target: "aarch64-apple-ios",
            platform_features: "metal".into(),
            ..Default::default()
        },
        Target {
            target: "aarch64-apple-ios-sim",
            platform_features: "metal".into(),
            ..Default::default()
        },
        Target {
            target: "x86_64-apple-ios",
            platform_features: "metal".into(),
            ..Default::default()
        },
    ]
}

fn android_targets() -> Vec<Target> {
    [
        Target {
            target: "aarch64-linux-android",
            android_env: true,
            ..Default::default()
        },
        Target {
            target: "x86_64-linux-android",
            android_env: true,
            ..Default::default()
        },
        Target {
            target: "i686-linux-android",
            android_env: true,
            ..Default::default()
        },
    ]
    .into()
}

fn wasm_targets() -> Vec<Target> {
    [Target {
        target: "wasm32-unknown-emscripten",
        emscripten_env: true,
        // `svg` does not build in skia-safe because of the `ureq` dependency (although it builds in
        // skia-bindings just fine): <https://github.com/briansmith/ring/issues/1043>
        disabled_features: "svg".into(),
        ..Default::default()
    }]
    .into()
}
