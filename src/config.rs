use crate::Result;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    devices: Vec<Device>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.devices.is_empty() {
            writeln!(f, "There are no mfa devices. Use set command to register your first mfa device.")
        } else {
            for d in self.devices.iter() {
                writeln!(f, "{d}")?;
            }
            write!(f, "")
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Device {
    profile: String,
    arn: String,
    secret: String,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[profile {}]", self.profile)?;
        writeln!(f, "arn\t: {}", self.arn)?;
        writeln!(f, "secret\t: {}", self.secret)
    }
}

impl Config {
    pub fn new() -> Result<Self> {
        let path = dirs::home_dir()
            .ok_or(anyhow!("Failed to get home directory."))?
            .join(".aws/mfa_config.yml");
        Self::load(path.as_path())
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_config() {
        let path = Path::new("mock/test.yml");
        let config = Config::load(path);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.devices.len(), 1);

        let device = config.devices.first().unwrap();
        assert_eq!(device.profile, "test");
        assert_eq!(device.arn, "arn:aws:iam::123456789012:mfa/mfa_device_name");
        assert_eq!(device.secret, "somesecret");
    }

    #[test]
    fn it_init_config_when_notfound() {
        let path = Path::new("mock/notfound.yml");
        let config = Config::load(path);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.devices.len(), 0);
    }
}
