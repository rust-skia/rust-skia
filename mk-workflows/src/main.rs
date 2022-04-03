#![allow(clippy::upper_case_acronyms)]
// Allow uppercase acronyms like QA and MacOS.

//! This program builds the github workflow files for the rust-skia project.
use std::{fmt, fs, ops::Deref, path::PathBuf};

mod config;

const QA_WORKFLOW: &str = include_str!("templates/qa-workflow.yaml");
const RELEASE_WORKFLOW: &str = include_str!("templates/release-workflow.yaml");
const LINUX_JOB: &str = include_str!("templates/linux-job.yaml");
const WINDOWS_JOB: &str = include_str!("templates/windows-job.yaml");
const MACOS_JOB: &str = include_str!("templates/macos-job.yaml");
const TARGET_TEMPLATE: &str = include_str!("templates/target.yaml");

fn main() {
    for workflow in config::workflows() {
        build_workflow(&workflow, &config::jobs(&workflow));
    }
}

#[derive(Clone, Debug)]
pub struct Workflow {
    kind: WorkflowKind,
    host_os: HostOS,
    host_target: &'static str,
    job_template: &'static str,
    targets: Vec<Target>,
    host_bin_ext: &'static str,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum WorkflowKind {
    QA,
    Release,
}

impl fmt::Display for WorkflowKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = match self {
            WorkflowKind::QA => "qa",
            WorkflowKind::Release => "release",
        };
        f.write_str(n)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum HostOS {
    Windows,
    Linux,
    MacOS,
}

impl fmt::Display for HostOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use HostOS::*;
        f.write_str(match self {
            Windows => "windows",
            Linux => "linux",
            MacOS => "macos",
        })
    }
}

#[derive(Clone, Default)]
pub struct Job {
    name: String,
    toolchain: &'static str,
    features: Features,
    skia_debug: bool,
    crt_static: bool,
    // we may need to disable clippy for beta builds temporarily.
    disable_clippy: bool,
    example_args: Option<String>,
}

fn build_workflow(workflow: &Workflow, jobs: &[Job]) {
    let host_os = workflow.host_os;
    let job_template = workflow.job_template;
    let targets = &workflow.targets;

    let workflow_name = format!("{}-{}", host_os, workflow.kind);
    let output_filename = PathBuf::new()
        .join(".github")
        .join("workflows")
        .join(format!("{workflow_name}.yaml"));

    let header = build_header(&workflow_name, workflow.kind);

    let mut parts = vec![header];

    for job in jobs {
        {
            let job_name = &job.name;
            let job_name = format!("{workflow_name}-{job_name}");
            let job_name = format!("{job_name}:").indented(1);
            parts.push(job_name);
        }

        {
            let job_header = build_job(workflow, job_template, job, targets).indented(2);
            parts.push(job_header);
        }

        let targets: Vec<String> = targets
            .iter()
            .map(|t| build_target(workflow, job, t).indented(2))
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

fn build_header(workflow_name: &str, workflow_kind: WorkflowKind) -> String {
    let replacements: Vec<_> = [("workflowName".to_owned(), workflow_name.to_owned())].into();

    let workflow = match workflow_kind {
        WorkflowKind::QA => QA_WORKFLOW,
        WorkflowKind::Release => RELEASE_WORKFLOW,
    };

    render_template(workflow, &replacements)
}

fn build_job(workflow: &Workflow, template: &str, job: &Job, targets: &[Target]) -> String {
    let skia_debug = if job.skia_debug { "1" } else { "0" };

    let rust_flags = if job.crt_static {
        "-C target-feature=+crt-static"
    } else {
        ""
    };

    let mut replacements = vec![
        ("rustToolchain".into(), job.toolchain.into()),
        ("skiaDebug".into(), skia_debug.into()),
        ("rustFlags".into(), rust_flags.into()),
    ];

    if let Some(macosx_deployment_target) = macosx_deployment_target(workflow, job, targets) {
        replacements.push((
            "macosxDeploymentTarget".into(),
            macosx_deployment_target.into(),
        ))
    }

    render_template(template, &replacements)
}

fn macosx_deployment_target(
    workflow: &Workflow,
    job: &Job,
    targets: &[Target],
) -> Option<&'static str> {
    if let HostOS::MacOS = workflow.host_os {
        let metal = "metal".to_owned();
        if targets
            .iter()
            .any(|target| effective_features(workflow, job, target).contains(&metal))
        {
            Some("10.14")
        } else {
            Some("10.13")
        }
    } else {
        None
    }
}

fn build_target(workflow: &Workflow, job: &Job, target: &Target) -> String {
    let features = effective_features(workflow, job, target);
    let native_target = workflow.host_target == target.target;
    let example_args = if native_target {
        job.example_args.clone()
    } else {
        None
    }
    .unwrap_or_default();
    let generate_artifacts = !example_args.is_empty();
    let run_clippy = native_target && !job.disable_clippy;
    let release_binaries = workflow.kind == WorkflowKind::Release;

    let template_arguments: &[(&'static str, &dyn fmt::Display)] = &[
        ("target", &target.target),
        ("androidEnv", &target.android_env),
        ("androidAPILevel", &config::DEFAULT_ANDROID_API_LEVEL),
        ("features", &features),
        ("runTests", &native_target),
        ("runClippy", &run_clippy),
        ("exampleArgs", &example_args),
        ("generateArtifacts", &generate_artifacts),
        ("releaseBinaries", &release_binaries),
        ("hostBinExt", &workflow.host_bin_ext),
    ];

    let replacements: Vec<(String, String)> = template_arguments
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect();

    render_template(TARGET_TEMPLATE, &replacements)
}

fn effective_features(workflow: &Workflow, job: &Job, target: &Target) -> Features {
    let mut features = job.features.clone();
    // if we are releasing binaries, we want the exact set of features specified.
    if workflow.kind == WorkflowKind::QA {
        features = features.join(&target.platform_features);
    }
    features
}

fn render_template(template: &str, replacements: &[(String, String)]) -> String {
    let mut template = template.to_owned();

    replacements.iter().for_each(|(pattern, value)| {
        template = template.replace(&format!("$[[{}]]", pattern), value)
    });

    assert!(
        !template.contains("$[["),
        "Template contains template patterns after replacement: \n{}",
        template
    );

    template
}

#[derive(Clone, Default, Debug)]
struct Target {
    target: &'static str,
    android_env: bool,
    platform_features: Features,
}

#[derive(Clone, Default, Debug)]
struct Features(Vec<String>);

impl Features {
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

trait Indented {
    fn indented(&self, i: usize) -> Self;
}

impl Indented for String {
    fn indented(&self, i: usize) -> String {
        let prefix: String = "  ".repeat(i);
        let indented_lines: Vec<String> = self.lines().map(|l| prefix.clone() + l).collect();

        indented_lines.join("\n")
    }
}
