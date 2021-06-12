//! This files builds the github actions file.

use std::{fs, iter, ops::Deref, path::PathBuf};

fn main() {
    build("windows", WINDOWS_JOB, &windows_targets());
    build("linux", LINUX_JOB, &linux_targets());
    build("macos", MACOS_JOB, &macos_targets());
}

fn build(os: &str, job: &str, targets: &[Target]) {
    let workflow_name = format!("build-{}", os);
    let output_filename = PathBuf::new()
        .join(".github")
        .join("workflows")
        .join(format!("{}.yaml", workflow_name));

    let job_name = workflow_name;

    let header = WORKFLOW.to_string();
    let job_header = format!("{}:", job_name).indented(1);
    let job = job.to_string().indented(2);

    let targets: Vec<String> = targets
        .iter()
        .map(|t| build_target(&t).indented(2))
        .collect();

    let mut parts = vec![header, job_header, job];
    parts.extend(targets);

    // some parts won't end with \n, so be safe and join them with a newline.
    let contents = parts
        .iter()
        .map(|p| p.trim_end().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    fs::create_dir_all(output_filename.parent().unwrap()).unwrap();
    fs::write(output_filename, contents).unwrap();
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

fn windows_targets() -> Vec<Target> {
    let host = Target {
        target: "x86_64-pc-windows-msvc",
        ..Target::windows_default()
    };

    [host].into()
}

fn linux_targets() -> Vec<Target> {
    let host = Target {
        target: "x86_64-unknown-linux-gnu",
        ..Target::windows_default()
    };

    [host].into()
}

fn macos_targets() -> Vec<Target> {
    let host = Target {
        target: "x86_64-apple-darwin",
        ..Target::windows_default()
    };

    [host].into()
}

fn build_target(target: &Target) -> String {
    let template_arguments: &[(&'static str, &dyn TemplateArgument)] = &[
        ("target", &target.target),
        ("androidEnv", &target.android_env),
        ("features", &target.features),
        ("runTests", &target.run_tests),
        ("runClippy", &target.run_clippy),
        ("exampleArgs", &target.example_args),
        ("generateArtifacts", &target.generate_artifacts),
        ("releaseBinaries", &target.release_binaries),
    ];
    let replacements: Vec<(String, String)> = template_arguments
        .iter()
        .map(|(name, value)| {
            (
                format!("${{{{{}}}}}", TemplateArgument::to_string(name)),
                value.to_string(),
            )
        })
        .collect();

    let mut template = TARGET_TEMPLATE.to_owned();

    replacements
        .iter()
        .for_each(|(pattern, value)| template = template.replace(pattern, value));

    if template.contains("${{") {
        panic!(
            "Template contains template patterns after replacement: \n{}",
            template
        );
    }

    template
}

const WORKFLOW: &str = include_str!("templates/workflow.yaml");

const LINUX_JOB: &str = include_str!("templates/linux-job.yaml");
const WINDOWS_JOB: &str = include_str!("templates/windows-job.yaml");
const MACOS_JOB: &str = include_str!("templates/macos-job.yaml");

const TARGET_TEMPLATE: &str = include_str!("templates/target.yaml");

#[derive(Debug)]
struct Target {
    target: &'static str,
    android_env: bool,
    features: Features,
    run_tests: bool,
    run_clippy: bool,
    example_args: String,
    generate_artifacts: bool,
    release_binaries: bool,
}

impl Target {
    const fn windows_default() -> Self {
        Self {
            target: "",
            android_env: false,
            features: Features::none(),
            run_tests: true,
            run_clippy: true,
            example_args: String::new(),
            generate_artifacts: true,
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
}

impl Deref for Features {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

trait TemplateArgument {
    fn to_string(&self) -> String;
}

impl TemplateArgument for &str {
    fn to_string(&self) -> String {
        <Self as ToString>::to_string(self)
    }
}

impl TemplateArgument for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}

impl TemplateArgument for bool {
    fn to_string(&self) -> String {
        match self {
            true => "true",
            false => "false",
        }
        .into()
    }
}

impl TemplateArgument for Features {
    fn to_string(&self) -> String {
        self.join(",")
    }
}
