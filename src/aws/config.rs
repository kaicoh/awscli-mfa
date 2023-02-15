use crate::Result;

use super::{aws_home, ConfFile, ConfLoader};
use anyhow::anyhow;
use std::path::{Path, PathBuf};

const PROFILE: &str = r"^\[profile\s+(.+)\]$";
const FILENAME: &str = "config";
const MFA_SERIAL: &str = "mfa_serial";

#[derive(Debug)]
pub struct Config {
    content: ConfFile,
}

impl Config {
    pub fn new() -> Result<Self> {
        let path = filepath()?;
        Self::load(path.as_path())
    }

    pub fn mfa_serial(&self, profile: &str) -> Result<&str> {
        self.content
            .profile(profile)
            .ok_or(anyhow!(
                "Not Found profile: {} at {}",
                profile,
                filepath()?.to_string_lossy(),
            ))?
            .get(MFA_SERIAL)
            .ok_or(anyhow!(
                "Not Found mfa_serial in profile {} at {}",
                profile,
                filepath()?.to_string_lossy(),
            ))
    }

    pub fn set_mfa_profile(self, src: &str, dst: &str) -> Result<Self> {
        let profile = self
            .content
            .profile(src)
            .cloned()
            .ok_or(anyhow!(
                "Not Found profile: {} at {}",
                src,
                filepath()?.to_string_lossy(),
            ))?
            .rename(dst)
            .remove(MFA_SERIAL);

        let content = self.content.set(profile);

        Ok(Self { content })
    }

    pub fn save(&self) -> Result<()> {
        let path = filepath()?;
        self.write(path.as_path())
    }

    fn load(path: &Path) -> Result<Self> {
        let fmt = Box::new(|p: &str| {
            if p == "default" {
                "[default]".to_string()
            } else {
                format!("[profile {p}]")
            }
        });

        let content = ConfLoader::new()
            .set_path(path)
            .set_reg_profile(PROFILE)
            .set_formatter(fmt)
            .load()?;

        Ok(Self { content })
    }

    fn write(&self, path: &Path) -> Result<()> {
        self.content.write(path)
    }
}

fn filepath() -> Result<PathBuf> {
    Ok(aws_home()?.join(FILENAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build() -> Config {
        let path = Path::new("mock/test_config");
        Config::load(path).unwrap()
    }

    #[test]
    fn it_gets_mfa_serial_from_profile() {
        let config = build();

        let result = config.mfa_serial("test");
        assert!(result.is_ok());

        let mfa_serial = result.unwrap();
        assert_eq!(mfa_serial, "arn:aws:iam::999999999999:mfa/user");

        let result = config.mfa_serial("default");
        assert!(result.is_err());
    }

    #[test]
    fn it_copies_profile_without_mfa_serial() {
        let config = build();

        let result = config.set_mfa_profile("test", "test_v2");
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(
            config.mfa_serial("test").unwrap(),
            "arn:aws:iam::999999999999:mfa/user"
        );
        assert!(config.mfa_serial("test_v2").is_err());

        let test = config.content.profile("test").unwrap();
        let test_v2 = config.content.profile("test_v2").unwrap();

        assert_eq!(test.get("region"), test_v2.get("region"));
        assert_eq!(test.get("output"), test_v2.get("output"));
    }

    #[test]
    fn it_writes_to_file() {
        let config0 = build();
        let path = Path::new("mock/write_test_config");

        let result = config0.write(path);
        assert!(result.is_ok());

        let config1 = Config::load(path).unwrap();

        assert_eq!(config0.content, config1.content);
    }
}
