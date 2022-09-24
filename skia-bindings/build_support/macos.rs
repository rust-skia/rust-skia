/// 6 digit deployment target.
pub fn deployment_target_6(macosx_deployment_target: &str) -> String {
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
