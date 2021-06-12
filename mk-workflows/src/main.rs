//! This program builds the github workflow files for the rust-skia project.
use std::{fmt, fs, iter, ops::Deref, path::PathBuf};

const DEFAULT_ANDROID_API_LEVEL: usize = 26;
const WORKFLOW: &str = include_str!("templates/workflow.yaml");
const LINUX_JOB: &str = include_str!("templates/linux-job.yaml");
const WINDOWS_JOB: &str = include_str!("templates/windows-job.yaml");
const MACOS_JOB: &str = include_str!("templates/macos-job.yaml");
const TARGET_TEMPLATE: &str = include_str!("templates/target.yaml");

fn main() {
    for workflow in workflows() {
        build_workflow(&workflow, &jobs());
    }
}

struct Workflow {
    os: &'static str,
    build_host: &'static str,
    job_template: &'static str,
    targets: Vec<Target>,
    host_bin_ext: &'static str,
}

fn workflows() -> Vec<Workflow> {
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

fn jobs() -> Vec<Job> {
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

struct Job {
    name: &'static str,
    toolchain: &'static str,
    base_features: Features,
    skia_debug: bool,
    example_args: Option<String>,
}

fn build_workflow(workflow: &Workflow, jobs: &[Job]) {
    let os = workflow.os;
    let job_template = workflow.job_template;
    let targets = &workflow.targets;

    let workflow_name = os.to_owned();
    let output_filename = PathBuf::new()
        .join(".github")
        .join("workflows")
        .join(format!("{}.yaml", workflow_name));

    let header = build_header(&workflow_name);

    let mut parts = vec![header];

    for job in jobs {
        let job_name = workflow_name.clone() + "-" + job.name;
        let job_name = format!("{}:", job_name).indented(1);
        parts.push(job_name);

        let job_header = build_job(job_template, &job).indented(2);
        parts.push(job_header);

        let targets: Vec<String> = targets
            .iter()
            .map(|t| build_target(&workflow, &job, &t).indented(2))
            .collect();

        parts.extend(targets);
    }

    // some parts won't end with \n, so be safe and join them with a newline.
    let contents = parts
        .iter()
        .map(|p| p.trim_end().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    fs::create_dir_all(output_filename.parent().unwrap()).unwrap();
    fs::write(output_filename, contents).unwrap();
}

fn build_header(workflow_name: &str) -> String {
    let replacements = [("workflowName".to_owned(), workflow_name.to_owned())];
    render_template(WORKFLOW, &replacements)
}

fn build_job(template: &str, job: &Job) -> String {
    let skia_debug = if job.skia_debug { "1" } else { "0" };

    let replacements = [
        ("rustToolchain".into(), job.toolchain.into()),
        ("skiaDebug".into(), skia_debug.into()),
    ];
    render_template(template, &replacements)
}

fn build_target(workflow: &Workflow, job: &Job, target: &Target) -> String {
    let features = job.base_features.join(&target.platform_features);
    let native_target = workflow.build_host == target.target;
    let example_args = if native_target {
        job.example_args.clone()
    } else {
        None
    }
    .unwrap_or_default();
    let generate_artifacts = !example_args.is_empty();

    let template_arguments: &[(&'static str, &dyn fmt::Display)] = &[
        ("target", &target.target),
        ("androidEnv", &target.android_env),
        ("androidAPILevel", &DEFAULT_ANDROID_API_LEVEL),
        ("features", &features),
        ("runTests", &native_target),
        ("runClippy", &native_target),
        ("exampleArgs", &example_args),
        ("generateArtifacts", &generate_artifacts),
        ("releaseBinaries", &target.release_binaries),
        ("hostBinExt", &workflow.host_bin_ext),
    ];

    let replacements: Vec<(String, String)> = template_arguments
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect();

    render_template(TARGET_TEMPLATE, &replacements)
}

fn render_template(template: &str, replacements: &[(String, String)]) -> String {
    let mut template = template.to_owned();

    replacements.iter().for_each(|(pattern, value)| {
        template = template.replace(&format!("$[[{}]]", pattern), value)
    });

    if template.contains("$[[") {
        panic!(
            "Template contains template patterns after replacement: \n{}",
            template
        );
    }

    template
}

#[derive(Debug)]
struct Target {
    target: &'static str,
    android_env: bool,
    platform_features: Features,
    release_binaries: bool,
}

impl Target {
    const fn windows_default() -> Self {
        Self {
            target: "",
            android_env: false,
            platform_features: Features::none(),
            release_binaries: false,
        }
    }
}

#[derive(Default, Debug)]
struct Features(Vec<String>);

impl Features {
    pub const fn none() -> Self {
        Self(vec![])
    }

    pub fn join(&self, other: &Self) -> Self {
        let mut features = self.0.clone();
        features.extend(other.0.iter().cloned());
        features.sort();
        features.dedup();
        Self(features)
    }
}

impl Deref for Features {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Features {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let features = self.0.join(",");
        f.write_str(&features)
    }
}

impl From<&str> for Features {
    fn from(s: &str) -> Self {
        let strs: Vec<String> = s
            .split(',')
            .filter_map(|s| {
                let f = s.trim().to_owned();
                if !f.is_empty() {
                    Some(f)
                } else {
                    None
                }
            })
            .collect();
        Features(strs)
    }
}

trait Indent {
    fn indented(&self, i: usize) -> Self;
}

impl Indent for String {
    fn indented(&self, i: usize) -> String {
        let prefix: String = iter::repeat("  ").take(i).collect();
        let indented_lines: Vec<String> = self.lines().map(|l| prefix.clone() + l).collect();

        indented_lines.join("\n")
    }
}
