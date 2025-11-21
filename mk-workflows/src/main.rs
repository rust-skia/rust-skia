//! This program builds the github workflow files for the rust-skia project.

// Allow uppercase acronyms like QA and MacOS.
#![allow(clippy::upper_case_acronyms)]

use std::{
    collections::{BTreeMap, HashSet},
    fmt, fs,
    ops::Sub,
    path::PathBuf,
};

use target::Target;

mod config;
mod target;

const QA_WORKFLOW: &str = include_str!("templates/qa-workflow.yaml");
const RELEASE_WORKFLOW: &str = include_str!("templates/release-workflow.yaml");
const LINUX_JOB: &str = include_str!("templates/linux-job.yaml");
const WINDOWS_JOB: &str = include_str!("templates/windows-job.yaml");
const WINDOWS_ARM_JOB: &str = include_str!("templates/windows-arm-job.yaml");
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
    targets: Vec<TargetConf>,
    host_bin_ext: &'static str,
}

impl fmt::Display for Workflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)?;
        f.write_str(": ")?;
        self.host_os.fmt(f)
    }
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
    WindowsArm,
    Linux,
    MacOS,
}

impl fmt::Display for HostOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use HostOS::*;
        f.write_str(match self {
            Windows => "windows",
            WindowsArm => "windows-arm",
            Linux => "linux",
            MacOS => "macos",
        })
    }
}

#[derive(Default)]
pub struct Job {
    name: String,
    toolchain: &'static str,
    features: Features,
    skia_debug: bool,
    // we may need to disable clippy for beta builds temporarily.
    disable_clippy: bool,
    example_args: Option<String>,
    matrix: BTreeMap<String, Vec<String>>,
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)?;
        f.write_str(", features: ")?;
        self.features.fmt(f)
    }
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
            let job_name = workflow_name.clone() + "-" + &job.name;
            let job_name = format!("{job_name}:").indented(1);
            parts.push(job_name);
        }

        {
            let job_header = build_job(workflow, job_template, job, targets).indented(2);
            parts.push(job_header);
        }

        let generic_steps = render_generic_steps(workflow, job).indented(2);
        parts.push(generic_steps);
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

fn build_job(workflow: &Workflow, template: &str, job: &Job, targets: &[TargetConf]) -> String {
    let skia_debug = if job.skia_debug { "1" } else { "0" };

    let mut replacements = vec![
        ("rustToolchain".into(), job.toolchain.into()),
        ("skiaDebug".into(), skia_debug.into()),
    ];

    if let Some(macosx_deployment_target) = macosx_deployment_target(workflow, job, targets) {
        replacements.push((
            "macosxDeploymentTarget".into(),
            macosx_deployment_target.into(),
        ))
    }

    let mut rendered = render_template(template, &replacements);

    let mut matrix_lines = Vec::new();
    matrix_lines.push("strategy:".to_string());
    matrix_lines.push("  fail-fast: false".to_string());
    matrix_lines.push("  matrix:".to_string());

    if let Some(features) = job.matrix.get("features") {
        matrix_lines.push(format!("    features: {:?}", features));
    }

    let target_names: Vec<String> = targets.iter().map(|t| t.target.to_string()).collect();
    matrix_lines.push(format!("    target: {:?}", target_names));

    matrix_lines.push("    include:".to_string());
    for target in targets {
        let android_env = target.android_env();
        let emscripten_env = target.emscripten_env();
        let native_target = workflow.host_target == target.target.to_string();
        let run_clippy = native_target && !job.disable_clippy;
        let run_tests = native_target;
        let example_args = if native_target {
            job.example_args.clone()
        } else {
            None
        }
        .unwrap_or_default();
        let generate_artifacts = !example_args.is_empty();

        let features_val = if workflow.kind == WorkflowKind::QA {
            effective_features(workflow, job, target).to_string()
        } else {
            String::new()
        };

        matrix_lines.push(format!("      - target: {}", target.target));
        matrix_lines.push(format!("        androidEnv: {}", android_env));
        matrix_lines.push(format!("        emscriptenEnv: {}", emscripten_env));
        matrix_lines.push(format!("        runClippy: {}", run_clippy));
        matrix_lines.push(format!("        runTests: {}", run_tests));
        matrix_lines.push(format!("        exampleArgs: '{}'", example_args));
        matrix_lines.push(format!("        generateArtifacts: {}", generate_artifacts));
        if workflow.kind == WorkflowKind::QA {
            matrix_lines.push(format!("        features: '{}'", features_val));
        }
    }

    if workflow.kind == WorkflowKind::Release {
        if let Some(features_list) = job.matrix.get("features") {
            let mut excludes = Vec::new();
            for feature_str in features_list {
                let features = Features::from(feature_str.as_str());
                for target in targets {
                    let mut disabled = false;
                    for f in &features.0 {
                        if target.disabled_features.contains(f) {
                            disabled = true;
                            break;
                        }
                    }
                    if disabled {
                        excludes.push((target.target.to_string(), feature_str.clone()));
                    }
                }
            }

            if !excludes.is_empty() {
                matrix_lines.push("    exclude:".to_string());
                for (t, f) in excludes {
                    matrix_lines.push(format!("      - target: {}", t));
                    matrix_lines.push(format!("        features: '{}'", f));
                }
            }
        }
    }

    let strategy = matrix_lines.join("\n");
    rendered = format!("{}\n{}", strategy, rendered);

    rendered
}

fn macosx_deployment_target(
    workflow: &Workflow,
    job: &Job,
    targets: &[TargetConf],
) -> Option<&'static str> {
    if let HostOS::MacOS = workflow.host_os {
        let metal = "metal".to_owned();
        if targets
            .iter()
            .any(|target| effective_features(workflow, job, target).contains(&metal))
        {
            return Some("10.14");
        } else {
            return Some("10.13");
        }
    }
    None
}

fn render_generic_steps(workflow: &Workflow, _job: &Job) -> String {
    let release_binaries = workflow.kind == WorkflowKind::Release;

    let template_arguments: &[(&'static str, &str)] = &[
        ("target", "${{ matrix.target }}"),
        ("androidEnv", "${{ matrix.androidEnv }}"),
        ("emscriptenEnv", "${{ matrix.emscriptenEnv }}"),
        (
            "androidAPILevel",
            &config::DEFAULT_ANDROID_API_LEVEL.to_string(),
        ),
        ("features", "${{ matrix.features }}"),
        ("runTests", "${{ matrix.runTests }}"),
        ("runClippy", "${{ matrix.runClippy }}"),
        ("exampleArgs", "${{ matrix.exampleArgs }}"),
        ("generateArtifacts", "${{ matrix.generateArtifacts }}"),
        ("releaseBinaries", &release_binaries.to_string()),
        ("hostBinExt", workflow.host_bin_ext),
        ("stepIf", "true"),
    ];

    let replacements: Vec<(String, String)> = template_arguments
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect();

    render_template(TARGET_TEMPLATE, &replacements)
}

fn effective_features(workflow: &Workflow, job: &Job, target: &TargetConf) -> Features {
    let mut features = job.features.clone();
    // if we are releasing binaries, we want the exact set of features specified.
    if workflow.kind == WorkflowKind::QA {
        features = features.join(&target.platform_features);
    }
    features.sub(&target.disabled_features)
}

fn render_template(template: &str, replacements: &[(String, String)]) -> String {
    let mut template = template.to_owned();

    replacements.iter().for_each(|(pattern, value)| {
        template = template.replace(&format!("$[[{pattern}]]"), value)
    });

    assert!(
        !template.contains("$[["),
        "Template contains template patterns after replacement: \n{template}"
    );

    template
}

#[derive(Clone, Debug)]
struct TargetConf {
    target: Target,
    /// Platform specific features.
    platform_features: Features,
    /// Features currently disabled for some reason.
    disabled_features: Features,
}

impl fmt::Display for TargetConf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)
    }
}

impl TargetConf {
    pub fn new(target: impl AsRef<str>, platform_features: impl Into<Features>) -> Self {
        Self {
            target: target.as_ref().parse().unwrap(),
            platform_features: platform_features.into(),
            disabled_features: Features::default(),
        }
    }

    pub fn disable(mut self, disabled_features: impl Into<Features>) -> Self {
        self.disabled_features = disabled_features.into();
        self
    }

    pub fn android_env(&self) -> bool {
        self.target.is_android()
    }

    pub fn emscripten_env(&self) -> bool {
        self.target.is_emscripten()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct Features(HashSet<String>);

impl Features {
    #[must_use]
    pub fn join(&self, other: &Self) -> Self {
        let mut features = self.0.clone();
        features.extend(other.0.iter().cloned());
        Self(features)
    }

    #[must_use]
    pub fn sub(&mut self, other: &Self) -> Self {
        Self(self.0.sub(&other.0))
    }

    pub fn contains(&self, feature: impl AsRef<str>) -> bool {
        self.0.contains(feature.as_ref())
    }

    /// Create a stable, comparable name for a feature combination, separated by a separator.
    pub fn name(&self, separator: &str) -> String {
        let mut strings: Vec<_> = self.0.iter().map(|s| s.as_str()).collect();
        strings.sort();
        strings.join(separator)
    }
}

impl fmt::Display for Features {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name(","))
    }
}

impl From<&str> for Features {
    fn from(s: &str) -> Self {
        let strs: HashSet<String> = s
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
        let indented_lines: Vec<String> = self
            .lines()
            .map(|l| {
                if !l.is_empty() {
                    prefix.clone() + l
                } else {
                    "".into()
                }
            })
            .collect();

        indented_lines.join("\n")
    }
}
