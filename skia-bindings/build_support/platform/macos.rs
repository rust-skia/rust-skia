use super::prelude::*;

pub struct MacOS;

impl TargetDetails for MacOS {
    fn args(&self, _config: &BuildConfiguration, builder: &mut ArgBuilder) {
        // Skia will take care to set a specific `--target` for the current macOS version. So we
        // don't push another target `--target` that may conflict.
        builder.target(None);

        // Add macOS specific environment variables that affect the output of a
        // build.
        cargo::rerun_if_env_var_changed("MACOSX_DEPLOYMENT_TARGET");

        builder.skia_target_os_and_default_cpu("mac");
    }
}
