use crate::Result;

use super::{aws_home, Content, ContentBuilder, PROFILE_CONFIG};
use std::path::PathBuf;

const FILENAME: &str = "config";
const MFA_SERIAL: &str = "mfa_serial";

#[derive(Debug)]
pub struct Config {
    content: Content,
}

impl Config {
    pub fn new() -> Result<Self> {
        let content = ContentBuilder::new()
            .set_path(filepath()?.as_path())
            .set_reg_profile(PROFILE_CONFIG)
            .load()?;

        Ok(Self { content })
    }

    pub fn mfa_serial(&self, profile: &str) -> Result<&str> {
        self.content.get(profile, MFA_SERIAL)
    }

    pub fn set_mfa_profile(self, src: &str, dst: &str) -> Result<Self> {
        let content = self.content
            .copy(src, dst)?
            .remove(dst, MFA_SERIAL)?;

        Ok(Self { content })
    }

    pub fn save(&self) -> Result<()> {
        let fmt = |p: &str| {
            if p == "default" {
                "[default]".to_string()
            } else {
                format!("[profile {p}]")
            }
        };

        self.content.write(filepath()?.as_path(), &fmt)
    }
}

fn filepath() -> Result<PathBuf> {
    Ok(aws_home()?.join(FILENAME))
}
