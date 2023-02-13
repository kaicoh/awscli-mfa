use crate::Result;

use super::{ConfigFileBase, Profile};
use anyhow::anyhow;

const MFA_SERIAL: &str = "mfa_serial";

#[derive(Debug)]
pub struct Config {
    profiles: Vec<Profile>,
}

impl ConfigFileBase for Config {
    const FILENAME: &'static str = "config";
    const PROFILE_PATTERN: &'static str = r"\[profile\s+(.+)\]";

    fn build(profiles: Vec<Profile>) -> Self {
        Self { profiles }
    }

    fn fmt_profile(profile: &Profile) -> String {
        format!("[profile {}]\n{}", profile.name, profile.lines.join("\n"))
    }

    fn profiles(&self) -> &[Profile] {
        &self.profiles
    }
}

impl Config {
    pub fn get_mfa_serial(&self, name: &str) -> Result<String> {
        self.get_attr(name, MFA_SERIAL)
    }

    pub fn set_mfa_profile(self, src: &str, dst: &str) -> Result<Self> {
        let lines = self
            .get(src)?
            .lines
            .into_iter()
            .filter(|line| match line.split_once('=') {
                Some((k, _)) => k.trim() != MFA_SERIAL,
                None => true,
            })
            .collect::<Vec<String>>();

        let profile = Profile {
            name: dst.into(),
            lines,
        };

        Ok(self.set(profile))
    }

    fn get_attr(&self, name: &str, key: &str) -> Result<String> {
        self.get(name)?
            .lines
            .iter()
            .find_map(|line| {
                line.split_once('=').and_then(|(k, v)| {
                    if k.trim() == key {
                        Some(v.trim())
                    } else {
                        None
                    }
                })
            })
            .map(String::from)
            .ok_or(anyhow!("Not Found key: {} in profile: {}", key, name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_reads_config() {
        let path = Path::new("mock/test_config");
        let config = Config::load(path);
        assert!(config.is_ok());

        let Config { profiles } = config.unwrap();
        assert_eq!(profiles.len(), 2);

        let profile = profiles.get(0).unwrap();
        assert_eq!(profile.name, "default");
        assert_eq!(profile.lines, vec!["region = us-east-1", "output = yaml",]);

        let profile = profiles.get(1).unwrap();
        assert_eq!(profile.name, "test");
        assert_eq!(
            profile.lines,
            vec!["region = ap-northeast-1", "output = json",]
        );
    }
}
