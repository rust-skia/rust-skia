use crate::{Job, LINUX_JOB, MACOS_JOB, Target, WINDOWS_JOB, Workflow};

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
            skia_debug: false,
            example_args: Some("--driver cpu --drive pdf --driver svg".into()),
        },
        Job {
            name: "stable-all-features-debug",
            toolchain: "stable",
            base_features: "gl,vulkan,textlayout,webp".into(),
            skia_debug: true,
            example_args: None,
        },
        Job {
            name: "beta-all-features",
            toolchain: "beta",
            base_features: "gl,vulkan,textlayout,webp".into(),
            skia_debug: false,
            example_args: None,
        },
    ]
    .into()
}

fn windows_targets() -> Vec<Target> {
    let host = Target {
        target: "x86_64-pc-windows-msvc",
        platform_features: "d3d".into(),
        ..Target::windows_default()
    };

    [host].into()
}

fn linux_targets() -> Vec<Target> {
    let host = Target {
        target: "x86_64-unknown-linux-gnu",
        platform_features: "".into(),
        ..Target::windows_default()
    };

    [host].into()
}

fn macos_targets() -> Vec<Target> {
    let host = Target {
        target: "x86_64-apple-darwin",
        platform_features: "metal".into(),
        ..Target::windows_default()
    };

    [host].into()
}
