use crate::Result;

use super::{aws_home, sts::StsCredential, Content, ContentBuilder, PROFILE_CREDENTIALS};
use std::path::PathBuf;

const FILENAME: &str = "credentials";

#[derive(Debug)]
pub struct Credentials {
    content: Content,
}

impl Credentials {
    pub fn new() -> Result<Self> {
        let content = ContentBuilder::new()
            .set_path(filepath()?.as_path())
            .set_reg_profile(PROFILE_CREDENTIALS)
            .load()?;

        Ok(Self { content })
    }

    pub fn set_cred(self, profile: &str, cred: StsCredential) -> Result<Self> {
        let StsCredential {
            access_key_id,
            secret_access_key,
            session_token,
            ..
        } = cred;

        let content = self
            .content
            .add(profile)?
            .set(profile, "aws_access_key_id", &access_key_id)?
            .set(profile, "aws_secret_access_key", &secret_access_key)?
            .set(profile, "aws_session_token", &session_token)?;

        Ok(Self { content })
    }

    pub fn save(&self) -> Result<()> {
        let fmt = Box::new(|p: &str| format!("[{p}]"));
        self.content.write(filepath()?.as_path(), &fmt)
    }
}

fn filepath() -> Result<PathBuf> {
    Ok(aws_home()?.join(FILENAME))
}
