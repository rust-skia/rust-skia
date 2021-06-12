use crate::{Job, Target, Workflow, LINUX_JOB, MACOS_JOB, WINDOWS_JOB};

pub const DEFAULT_ANDROID_API_LEVEL: usize = 26;

pub fn workflows() -> Vec<Workflow> {
    [
        Workflow {
            os: "windows",
            build_host: "x86_64-pc-windows-msvc",
            job_template: WINDOWS_JOB,
            targets: windows_targets(),
            host_bin_ext: ".exe",
        },
        Workflow {
            os: "linux",
            build_host: "x86_64-unknown-linux-gnu",
            job_template: LINUX_JOB,
            targets: linux_targets(),
            host_bin_ext: "",
        },
        Workflow {
            os: "macos",
            build_host: "x86_64-apple-darwin",
            job_template: MACOS_JOB,
            targets: macos_targets(),
            host_bin_ext: "",
        },
    ]
    .into()
}

pub fn jobs() -> Vec<Job> {
    [
        Job {
            name: "stable-all-features",
            toolchain: "stable",
            base_features: "gl,vulkan,textlayout,webp".into(),
            example_args: Some("--driver cpu --driver pdf --driver svg".into()),
            ..Job::default()
        },
        Job {
            name: "stable-all-features-debug",
            toolchain: "stable",
            base_features: "gl,vulkan,textlayout,webp".into(),
            skia_debug: true,
            ..Job::default()
        },
        Job {
            name: "beta-all-features",
            toolchain: "beta",
            base_features: "gl,vulkan,textlayout,webp".into(),
            ..Job::default()
        },
    ]
    .into()
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
