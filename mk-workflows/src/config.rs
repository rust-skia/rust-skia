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
    [
        Job {
            name: "stable-all-features".into(),
            toolchain: "stable",
            features: "gl,vulkan,textlayout,webp".into(),
            example_args: Some("--driver cpu --driver pdf --driver svg".into()),
            ..Job::default()
        },
        Job {
            name: "stable-all-features-debug".into(),
            toolchain: "stable",
            features: "gl,vulkan,textlayout,webp".into(),
            skia_debug: true,
            ..Job::default()
        },
        Job {
            name: "beta-all-features".into(),
            toolchain: "beta",
            features: "gl,vulkan,textlayout,webp".into(),
            ..Job::default()
        },
    ]
    .into()
}

/// Jobs for releasing prebuilt binaries.
pub fn release_jobs(workflow: &Workflow) -> Vec<Job> {
    let mut jobs: Vec<_> = [
        job("release", ""),
        job("release-gl", "gl"),
        job("release-vulkan", "vulkan"),
        job("release-textlayout", "textlayout"),
        job("release-gl-textlayout", "gl,textlayout"),
        job("release-vulkan-textlayout", "vulkan,textlayout"),
        job("release-gl-vulkan-textlayout", "gl,vulkan,textlayout"),
    ]
    .into();

    match workflow.host_os {
        HostOS::Windows => {
            jobs.push(job("release-d3d", "d3d"));
            jobs.push(job("release-d3d-textlayout", "d3d,textlayout"));

            let static_jobs: Vec<_> = jobs
                .iter()
                .cloned()
                .map(|j| Job {
                    name: j.name + "-static",
                    crt_static: true,
                    ..j
                })
                .collect();
            jobs.extend(static_jobs);
        }
        HostOS::Linux => {
            jobs.push(job("release-gl-x11", "gl,x11"));
            jobs.push(job("release-gl-textlayout-x11", "gl,textlayout,x11"));
        }
        HostOS::MacOS => {
            jobs.push(job("release-metal", "metal"));
            jobs.push(job("release-metal-textlayout", "metal,textlayout"));
        }
    }

    return jobs;

    fn job(name: &'static str, features: impl Into<Features>) -> Job {
        Job {
            name: name.into(),
            toolchain: "stable",
            features: features.into(),
            ..Job::default()
        }
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
