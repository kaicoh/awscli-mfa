use crate::Result;

mod base;
mod config;
mod credentials;
mod sts;

use anyhow::anyhow;
use base::{Content, ContentBuilder, PROFILE_CONFIG, PROFILE_CREDENTIALS};
use config::Config;
use credentials::Credentials;
use std::path::PathBuf;
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

    pub fn mfa_serial(&self, name: &str) -> Result<String> {
        self.config.mfa_serial(name).map(String::from)
    }

    pub fn set_cred(self, src: &str, dst: &str, cred: sts::StsCredential) -> Result<Self> {
        let Self {
            config,
            credentials,
        } = self;

        Ok(Self {
            config: config.set_mfa_profile(src, dst)?,
            credentials: credentials.set_cred(dst, cred)?,
        })
    }

    pub fn save(&self) -> Result<()> {
        self.config.save()?;
        self.credentials.save()
    }
}

fn aws_home() -> Result<PathBuf> {
    dirs::home_dir()
        .ok_or(anyhow!("Failed to get home directory."))
        .map(|p| p.join(".aws"))
}
