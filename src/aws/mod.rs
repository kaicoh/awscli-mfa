use crate::Result;

mod base;
mod config;
mod credentials;
mod sts;

use base::{ConfigFileBase, Profile};
use config::Config;
use credentials::Credentials;
pub use sts::GetSessionToken;

#[derive(Debug)]
pub struct AwsConfigs {
    config: Config,
    credentials: Credentials,
}

impl AwsConfigs {
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: Config::new()?,
            credentials: Credentials::new()?,
        })
    }

    pub fn get_mfa_serial(&self, name: &str) -> Result<String> {
        self.config.get_mfa_serial(name)
    }

    pub fn set_profile(self, src: &str, dst: &str, cred: sts::StsCredential) -> Result<Self> {
        let Self { config, credentials } = self;

        Ok(Self {
            config: config.set_mfa_profile(src, dst)?,
            credentials: credentials.set_sts_cred(dst, cred),
        })
    }

    pub fn save(&self) -> Result<()> {
        self.config.save()?;
        self.credentials.save()
    }
}
