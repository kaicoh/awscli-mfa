use super::{sts::StsCredential, ConfigFileBase, Profile};

#[derive(Debug)]
pub struct Credentials {
    profiles: Vec<Profile>,
}

impl ConfigFileBase for Credentials {
    const FILENAME: &'static str = "credentials";
    const PROFILE_PATTERN: &'static str = r"\[(.+)\]";

    fn build(profiles: Vec<Profile>) -> Self {
        Self { profiles }
    }

    fn fmt_profile(profile: &Profile) -> String {
        format!("[{}]\n{}", profile.name, profile.lines.join("\n"))
    }

    fn profiles(&self) -> &[Profile] {
        &self.profiles
    }
}

impl Credentials {
    pub fn set_sts_cred(self, name: &str, cred: StsCredential) -> Self {
        let StsCredential {
            access_key_id,
            secret_access_key,
            session_token,
            ..
        } = cred;

        let profile = Profile {
            name: name.into(),
            lines: vec![
                format!("aws_access_key_id={access_key_id}"),
                format!("aws_secret_access_key={secret_access_key}"),
                format!("aws_session_token={session_token}"),
            ],
        };

        self.set(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_reads_credentials() {
        let path = Path::new("mock/test_credentials");
        let credentials = Credentials::load(path);
        assert!(credentials.is_ok());

        let Credentials { profiles } = credentials.unwrap();
        assert_eq!(profiles.len(), 2);

        let profile = profiles.get(0).unwrap();
        assert_eq!(profile.name, "tanaka");
        assert_eq!(
            profile.lines,
            vec![
                "aws_access_key_id=ABCDEFGHIJKLMNOPQRST",
                "aws_secret_access_key=abcdefghijklmnopqrstuvwxyz+-#$1234567890",
            ]
        );

        let profile = profiles.get(1).unwrap();
        assert_eq!(profile.name, "suzuki");
        assert_eq!(profile.lines, vec!["xxxxxxxxxxxxxxxx", "yyyyyyyyyyyy",]);
    }

    #[test]
    fn it_writes_credentials() {
        let credentials = Credentials {
            profiles: vec![
                Profile {
                    name: "tanaka".into(),
                    lines: vec!["foobarbaz".into()],
                },
                Profile {
                    name: "takahashi".into(),
                    lines: vec!["foo".into(), "bar".into()],
                },
                Profile {
                    name: "saito".into(),
                    lines: vec![],
                },
            ],
        };

        let path = Path::new("mock/write_test_credentials");
        credentials.write(path).unwrap();

        let credentials = Credentials::load(path);
        assert!(credentials.is_ok());

        let Credentials { profiles } = credentials.unwrap();
        assert_eq!(profiles.len(), 3);

        let profile = profiles.get(0).unwrap();
        assert_eq!(profile.name, "tanaka");
        assert_eq!(profile.lines, vec!["foobarbaz"]);

        let profile = profiles.get(1).unwrap();
        assert_eq!(profile.name, "takahashi");
        assert_eq!(profile.lines, vec!["foo", "bar"]);

        let profile = profiles.get(2).unwrap();
        assert_eq!(profile.name, "saito");
        assert!(profile.lines.is_empty());
    }
}
