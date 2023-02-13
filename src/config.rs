use crate::Result;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MfaConfig {
    secrets: Vec<Secret>,
}

impl fmt::Display for MfaConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.secrets.is_empty() {
            writeln!(
                f,
                "There are no secret keys in ~/.aws/awsmfa.yml. Use set command to register your first secret key."
            )
        } else {
            for s in self.secrets.iter() {
                writeln!(f, "{s}")?;
            }
            write!(f, "")
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Secret {
    profile: String,
    value: String,
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[profile {}]", self.profile)?;
        writeln!(f, "secret\t: {}", self.value)
    }
}

impl MfaConfig {
    pub fn new() -> Result<Self> {
        let path = Self::path()?;
        Self::load(path.as_path())
    }

    pub fn set(self, profile: &str, value: &str) -> Self {
        let mut secrets = self.remove(profile).secrets;

        secrets.push(Secret {
            profile: profile.into(),
            value: value.into(),
        });

        Self { secrets }
    }

    pub fn remove(self, profile: &str) -> Self {
        let secrets: Vec<Secret> = self
            .secrets
            .into_iter()
            .filter(|s| s.profile != profile)
            .collect();

        Self { secrets }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        self.write(path.as_path())
    }

    pub fn get_secret(&self, profile: &str) -> Result<String> {
        self.secrets
            .iter()
            .find_map(|s| {
                if s.profile == profile {
                    Some(s.value.to_string())
                } else {
                    None
                }
            })
            .ok_or(anyhow!("Not found mfa device for profile: {}", profile))
    }

    fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let config = std::fs::read_to_string(path)
                .map_err(|e| anyhow!("{}: {}", e, path.to_str().unwrap()))?;
            serde_yaml::from_str(&config).map_err(anyhow::Error::new)
        } else {
            Ok(Self::default())
        }
    }

    fn write(&self, path: &Path) -> Result<()> {
        let file = fs::File::create(path).map_err(anyhow::Error::new)?;
        serde_yaml::to_writer(file, self).map_err(anyhow::Error::new)
    }

    fn path() -> Result<PathBuf> {
        dirs::home_dir()
            .ok_or(anyhow!("Failed to get home directory."))
            .map(|p| p.join(".aws/awsmfa.yml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_config() {
        let path = Path::new("mock/test.yml");
        let config = MfaConfig::load(path);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.secrets.len(), 1);

        let secret = config.secrets.first().unwrap();
        assert_eq!(secret.profile, "test");
        assert_eq!(secret.value, "somesecret");
    }

    #[test]
    fn it_gets_secret() {
        let path = Path::new("mock/test.yml");
        let config = MfaConfig::load(path).unwrap();
        let secret = config.get_secret("test");
        assert!(secret.is_ok());
        assert_eq!(secret.unwrap(), "somesecret");
    }

    #[test]
    fn it_init_config_when_notfound() {
        let path = Path::new("mock/notfound.yml");
        let config = MfaConfig::load(path);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.secrets.len(), 0);
    }

    #[test]
    fn it_sets_new_device() {
        let path = Path::new("mock/test.yml");
        let config = MfaConfig::load(path)
            .unwrap()
            .set("new_profile", "new_secret");

        assert_eq!(config.secrets.len(), 2);

        let secret = config.secrets.iter().find(|d| d.profile == "new_profile");
        assert!(secret.is_some());

        let secret = secret.unwrap();
        assert_eq!(secret.profile, "new_profile");
        assert_eq!(secret.value, "new_secret");
    }

    #[test]
    fn it_writes_contents() {
        let path = Path::new("mock/write_test.yml");
        MfaConfig::load(Path::new("mock/test.yml"))
            .unwrap()
            .set("write_profile", "write_secret")
            .write(path)
            .unwrap();

        let config = MfaConfig::load(path).unwrap();
        assert_eq!(config.secrets.len(), 2);
    }
}
