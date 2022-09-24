use super::cargo;

pub fn extra_skia_cflags() -> Vec<String> {
    let deployment_target = cargo::env_var("MACOSX_DEPLOYMENT_TARGET");

    if let Some(deployment_target) = deployment_target {
        let deployment_target = deployment_target_6(&deployment_target);
        return vec![format!(
            "-D__MAC_OS_X_VERSION_MAX_ALLOWED={deployment_target}"
        )];
    }
    Vec::new()
}

pub fn additional_clang_args() -> Vec<String> {
    extra_skia_cflags()
}

/// 6 digit deployment target.
fn deployment_target_6(macosx_deployment_target: &str) -> String {
    // use remove_matches as soon it's stable.
    let split: Vec<_> = macosx_deployment_target.split('.').collect();
    let joined = split.join("");
    dbg!(format!("{:0<6}", joined))
}

#[cfg(test)]
mod tests {
    use super::deployment_target_6;

    #[test]
    fn deployment_target_6_digit_conversion() {
        assert!(deployment_target_6("10.16"), "101600")
    }
}
