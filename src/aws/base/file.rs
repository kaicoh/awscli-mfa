use super::Profile;
use crate::Result;

use anyhow::anyhow;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct ConfFile {
    formatter: Box<dyn Fn(&str) -> String>,
    profiles: Vec<Profile>,
}

impl ConfFile {
    pub fn profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.name() == name)
    }

    pub fn set(self, profile: Profile) -> Self {
        let name = profile.name();
        let mut profiles = self
            .profiles
            .into_iter()
            .filter(|p| p.name() != name)
            .collect::<Vec<Profile>>();

        profiles.push(profile);

        Self { profiles, ..self }
    }

    pub fn write(&self, path: &Path) -> Result<()> {
        std::fs::write(path, self.format()).map_err(|err| {
            anyhow!(
                "Error writing to \"{}\". {}",
                path.to_str().unwrap_or("unknown path"),
                err
            )
        })
    }

    fn format(&self) -> String {
        let fmt = &self.formatter;
        self.profiles
            .iter()
            .map(|p| p.format(fmt))
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl fmt::Debug for ConfFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfFile")
            .field("profiles", &self.profiles)
            .finish()
    }
}

impl PartialEq for ConfFile {
    fn eq(&self, other: &Self) -> bool {
        if self.profiles.len() != other.profiles.len() {
            return false;
        }

        self.profiles.iter().all(|p| match other.profile(p.name()) {
            Some(other_p) => *p == *other_p,
            None => false,
        })
    }
}

#[derive(Default)]
pub struct ConfLoader<'a> {
    reg_profile: Option<&'a str>,
    path: Option<&'a Path>,
    formatter: Option<Box<dyn Fn(&str) -> String>>,
}

impl<'a> ConfLoader<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_reg_profile(self, reg_profile: &'a str) -> Self {
        Self {
            reg_profile: Some(reg_profile),
            ..self
        }
    }

    pub fn set_path(self, path: &'a Path) -> Self {
        Self {
            path: Some(path),
            ..self
        }
    }

    pub fn set_formatter(self, formatter: Box<dyn Fn(&str) -> String>) -> Self {
        Self {
            formatter: Some(formatter),
            ..self
        }
    }

    pub fn load(self) -> Result<ConfFile> {
        let reg_profile = match self.reg_profile {
            Some(r) => r,
            None => return Err(anyhow!("reg_profile is not set")),
        };

        let path = match self.path {
            Some(p) => p,
            None => return Err(anyhow!("path is not set")),
        };

        let formatter = match self.formatter {
            Some(f) => f,
            None => return Err(anyhow!("formatter is not set")),
        };

        let reader = BufReader::new(File::open(path)?);
        let mut profiles: Vec<Profile> = vec![];
        let mut profile = Profile::new("");

        for line in reader.lines() {
            let line = line?;

            if let Some(name) = capture(reg_profile, &line) {
                if !profile.name().is_empty() {
                    profiles.push(profile.clone());
                }

                profile = Profile::new(name);
            } else if !line.is_empty() {
                profile = profile.push(&line);
            }
        }

        if !profile.name().is_empty() {
            profiles.push(profile);
        }

        Ok(ConfFile {
            formatter,
            profiles,
        })
    }
}

fn capture<'a>(pattern: &'a str, line: &'a str) -> Option<&'a str> {
    let line = line.trim();

    if line == "[default]" {
        return Some("default");
    }

    Regex::new(pattern)
        .unwrap()
        .captures(line)
        .and_then(|caps| caps.get(1))
        .map(|mat| mat.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod conf_file {
        use super::*;

        const REG_PROFILE: &str = r"^\[profile (.+)\]";

        fn build() -> ConfFile {
            let fmt = Box::new(|p: &str| {
                if p == "default" {
                    "[default]".to_string()
                } else {
                    format!("[profile {p}]")
                }
            });

            ConfLoader::new()
                .set_path(Path::new("mock/test_base"))
                .set_reg_profile(REG_PROFILE)
                .set_formatter(fmt)
                .load()
                .unwrap()
        }

        #[test]
        fn it_loads_config_file() {
            let conf = build();
            assert_eq!(conf.profiles.len(), 2);

            let profile = conf.profile("default").cloned();
            assert!(profile.is_some());

            let expected = Profile::new("default")
                .set("region", "us-east-1")
                .set("output", "yaml");

            assert_eq!(profile.unwrap(), expected);

            let profile = conf.profile("test").cloned();
            assert!(profile.is_some());

            let expected = Profile::new("test")
                .set("region", "ap-northeast-1")
                .set("output", "json");

            assert_eq!(profile.unwrap(), expected);
        }

        #[test]
        fn it_sets_new_profile() {
            let conf = build();
            assert_eq!(conf.profiles.len(), 2);

            let profile = Profile::new("test_v2");
            let conf = conf.set(profile);

            assert_eq!(conf.profiles.len(), 3);

            // profile already exists
            let profile = Profile::new("test")
                .set("region", "us-west-2")
                .set("output", "string");
            let conf = conf.set(profile);

            // not change profiles size
            assert_eq!(conf.profiles.len(), 3);

            // but the profile is overwritten.
            let profile = conf.profile("test").cloned().unwrap();
            let expected = Profile::new("test")
                .set("region", "us-west-2")
                .set("output", "string");

            assert_eq!(profile, expected);
        }

        #[test]
        fn it_regards_same_if_profiles_is_matched() {
            let fmt = Box::new(|p: &str| format!("[{p}]"));

            let conf0 = ConfLoader::new()
                .set_path(Path::new("mock/test_base_equiv"))
                .set_reg_profile(REG_PROFILE)
                .set_formatter(fmt)
                .load()
                .unwrap();
            let conf1 = build();

            assert_eq!(conf0, conf1);
        }

        #[test]
        fn it_writes_to_file() {
            let path = Path::new("mock/write_test_base");
            let conf0 = build();
            let result = conf0.write(path);
            assert!(result.is_ok());

            let conf1 = ConfLoader::new()
                .set_path(path)
                .set_reg_profile(REG_PROFILE)
                .set_formatter(Box::new(|p| format!("[{p}]")))
                .load()
                .unwrap();

            assert_eq!(conf0, conf1);
        }
    }
}
