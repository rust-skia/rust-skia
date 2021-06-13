use crate::{Features, HostOS, Job, Target, Workflow, LINUX_JOB, MACOS_JOB, WINDOWS_JOB};

pub const DEFAULT_ANDROID_API_LEVEL: usize = 26;

pub fn workflows() -> Vec<Workflow> {
    [
        Workflow {
            host_os: HostOS::Windows,
            host_target: "x86_64-pc-windows-msvc",
            job_template: WINDOWS_JOB,
            targets: windows_targets(),
            host_bin_ext: ".exe",
        },
        Workflow {
            host_os: HostOS::Linux,
            host_target: "x86_64-unknown-linux-gnu",
            job_template: LINUX_JOB,
            targets: linux_targets(),
            host_bin_ext: "",
        },
        Workflow {
            host_os: HostOS::MacOS,
            host_target: "x86_64-apple-darwin",
            job_template: MACOS_JOB,
            targets: macos_targets(),
            host_bin_ext: "",
        },
    ]
    .into()
}

pub fn jobs(workflow: &Workflow) -> Vec<Job> {
    qa_jobs()
        .into_iter()
        .chain(binaries_jobs(workflow))
        .collect()
}

pub fn qa_jobs() -> Vec<Job> {
    [
        Job {
            name: "qa-stable-all-features",
            toolchain: "stable",
            features: "gl,vulkan,textlayout,webp".into(),
            example_args: Some("--driver cpu --driver pdf --driver svg".into()),
            ..Job::default()
        },
        Job {
            name: "qa-stable-all-features-debug",
            toolchain: "stable",
            features: "gl,vulkan,textlayout,webp".into(),
            skia_debug: true,
            ..Job::default()
        },
        Job {
            name: "qa-beta-all-features",
            toolchain: "beta",
            features: "gl,vulkan,textlayout,webp".into(),
            ..Job::default()
        },
    ]
    .into()
}

/// Jobs for releasing prebuilt binaries.
pub fn binaries_jobs(workflow: &Workflow) -> Vec<Job> {
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

    if workflow.host_os == HostOS::Linux {
        jobs.push(job("release-gl-x11", "gl,x11"));
        jobs.push(job("release-gl-textlayout-x11", "gl,textlayout,x11"));
    }

    if workflow.host_os == HostOS::Windows {
        jobs.push(job("release-d3d", "d3d"));
        jobs.push(job("release-d3d-textlayout", "d3d,textlayout"));
    }

    if workflow.host_os == HostOS::MacOS {
        jobs.push(job("release-metal", "metal"));
        jobs.push(job("release-metal-textlayout", "metal,textlayout"));
    }

    return jobs;

    fn job(name: &'static str, features: impl Into<Features>) -> Job {
        Job {
            name,
            toolchain: "stable",
            release_binaries: true,
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
