use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Target {
    pub architecture: String,
    pub vendor: String,
    pub system: String,
    pub abi: Option<String>,
}

impl Target {
    pub fn is_android(&self) -> bool {
        self.system == "android"
    }

    pub fn is_emscripten(&self) -> bool {
        self.system == "emscripten"
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            &self.architecture, &self.vendor, &self.system
        )?;

        if let Some(ref abi) = self.abi {
            write!(f, "-{abi}")
        } else {
            Result::Ok(())
        }
    }
}

impl FromStr for Target {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_target(s))
    }
}

pub fn parse_target(target_str: impl AsRef<str>) -> Target {
    let target_str = target_str.as_ref();
    let target: Vec<String> = target_str.split('-').map(|s| s.into()).collect();

    if target.len() >= 3 {
        let abi = if target.len() > 3 {
            Some(target[3].clone())
        } else {
            None
        };

        Target {
            architecture: target[0].clone(),
            vendor: target[1].clone(),
            system: target[2].clone(),
            abi,
        }
    } else if target.len() == 2 {
        Target {
            architecture: target[0].clone(),
            vendor: String::new(),
            system: target[1].clone(),
            abi: None,
        }
    } else {
        panic!("Failed to parse TARGET {target_str}");
    }
}
