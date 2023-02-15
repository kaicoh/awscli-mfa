use crate::Result;

use super::{aws_home, sts::StsCredential, ConfFile, ConfLoader, Profile};
use std::path::{Path, PathBuf};

const PROFILE: &str = r"^\[(.+)\]$";
const FILENAME: &str = "credentials";

#[derive(Debug)]
pub struct Credentials {
    content: ConfFile,
}

impl Credentials {
    pub fn new() -> Result<Self> {
        let path = filepath()?;
        Self::load(path.as_path())
    }

    pub fn set_cred(self, name: &str, cred: StsCredential) -> Self {
        let StsCredential {
            access_key_id,
            secret_access_key,
            session_token,
            ..
        } = cred;

        let profile = Profile::new(name)
            .set("aws_access_key_id", &access_key_id)
            .set("aws_secret_access_key", &secret_access_key)
            .set("aws_session_token", &session_token);

        let content = self.content.set(profile);

        Self { content }
    }

    pub fn save(&self) -> Result<()> {
        let path = filepath()?;
        self.write(path.as_path())
    }

    fn load(path: &Path) -> Result<Self> {
        let fmt = Box::new(|p: &str| format!("[{p}]"));
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

    fn build() -> Credentials {
        let path = Path::new("mock/test_credentials");
        Credentials::load(path).unwrap()
    }

    #[test]
    fn it_loads_credentials() {
        let creds = build();
        let result = creds.content.profile("tanaka");
        assert!(result.is_some());

        let profile = result.unwrap();
        assert_eq!(
            profile.get("aws_access_key_id"),
            Some("ABCDEFGHIJKLMNOPQRST")
        );
        assert_eq!(
            profile.get("aws_secret_access_key"),
            Some("abcdefghijklmnopqrstuvwxyz+-#$1234567890")
        );
    }

    #[test]
    fn it_sets_profile_from_sts_credentials() {
        let mut cred = StsCredential::default();
        cred.access_key_id = "access_key_id".to_string();
        cred.secret_access_key = "secret_access_key".to_string();
        cred.session_token = "session_token".to_string();

        let creds = build().set_cred("test", cred);
        let result = creds.content.profile("test");
        assert!(result.is_some());

        let profile = result.unwrap();
        assert_eq!(profile.get("aws_access_key_id"), Some("access_key_id"));
        assert_eq!(
            profile.get("aws_secret_access_key"),
            Some("secret_access_key")
        );
        assert_eq!(profile.get("aws_session_token"), Some("session_token"));
    }

    #[test]
    fn it_writes_to_file() {
        let path = Path::new("mock/write_test_credentials");
        let creds0 = build();
        let result = creds0.write(path);
        assert!(result.is_ok());

        let creds1 = Credentials::load(path).unwrap();
        assert_eq!(creds0.content, creds1.content);
    }
}
