use crate::Result;

use super::sts;
use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::string::ToString;

lazy_static! {
    static ref RE_PROFILE: Regex = Regex::new(r"\[(.+)\]").unwrap();
}

#[derive(Debug)]
pub struct Config {
    credentials: Vec<Credential>,
}

#[derive(Debug)]
pub struct Credential {
    profile: String,
    lines: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let path = Self::path()?;
        Self::load(path.as_path())
    }

    pub fn set(self, credential: Credential) -> Self {
        let profile: &str = &credential.profile;
        let mut credentials: Vec<Credential> = self
            .credentials
            .into_iter()
            .filter(|c| c.profile != profile)
            .collect();

        credentials.push(credential);

        Self { credentials }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        self.write(path.as_path())
    }

    fn load(path: &Path) -> Result<Self> {
        let reader = BufReader::new(File::open(path)?);
        let mut credentials: Vec<Credential> = Vec::new();
        let mut profile = "".to_string();
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if let Some(p) = capture_profile(&line) {
                if !profile.is_empty() {
                    credentials.push(Credential {
                        profile: profile.clone(),
                        lines: lines.clone(),
                    });
                }

                profile = p.to_string();
                lines = Vec::new();
            } else if !line.is_empty() {
                lines.push(line);
            }
        }

        if !profile.is_empty() {
            credentials.push(Credential { profile, lines });
        }

        Ok(Self { credentials })
    }

    fn write(&self, path: &Path) -> Result<()> {
        std::fs::write(path, self.to_string())
            .map_err(|e| anyhow!("Error writing to credentials: {}", e))
    }

    fn path() -> Result<PathBuf> {
        dirs::home_dir()
            .ok_or(anyhow!("Failed to get home directory."))
            .map(|p| p.join(".aws/credentials"))
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        self.credentials
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl Credential {
    pub fn from_sts_cred(profile: &str, cred: sts::Credential) -> Self {
        let sts::Credential {
            access_key_id,
            secret_access_key,
            session_token,
        } = cred;

        Self {
            profile: profile.into(),
            lines: vec![
                format!("aws_access_key_id={access_key_id}"),
                format!("aws_secret_access_key={secret_access_key}"),
                format!("aws_session_token={session_token}"),
            ],
        }
    }
}

impl ToString for Credential {
    fn to_string(&self) -> String {
        format!("[{}]\n{}", self.profile, self.lines.join("\n"))
    }
}

fn capture_profile(line: &str) -> Option<&str> {
    RE_PROFILE
        .captures(line)
        .and_then(|caps| caps.get(1))
        .map(|mat| mat.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_credentials() {
        let path = Path::new("mock/test_credentials");
        let config = Config::load(path);
        assert!(config.is_ok());

        let Config { credentials } = config.unwrap();
        assert_eq!(credentials.len(), 2);

        let cred = credentials.get(0).unwrap();
        assert_eq!(cred.profile, "tanaka");
        assert_eq!(
            cred.lines,
            vec![
                "aws_access_key_id=ABCDEFGHIJKLMNOPQRST",
                "aws_secret_access_key=abcdefghijklmnopqrstuvwxyz+-#$1234567890",
            ]
        );

        let cred = credentials.get(1).unwrap();
        assert_eq!(cred.profile, "suzuki");
        assert_eq!(cred.lines, vec!["xxxxxxxxxxxxxxxx", "yyyyyyyyyyyy",]);
    }

    #[test]
    fn it_writes_credentials() {
        let config = Config {
            credentials: vec![
                Credential {
                    profile: "tanaka".into(),
                    lines: vec!["foobarbaz".into()],
                },
                Credential {
                    profile: "takahashi".into(),
                    lines: vec!["foo".into(), "bar".into()],
                },
                Credential {
                    profile: "saito".into(),
                    lines: vec![],
                },
            ],
        };

        let path = Path::new("mock/write_test_credentials");
        config.write(path).unwrap();

        let config = Config::load(path);
        assert!(config.is_ok());

        let Config { credentials } = config.unwrap();
        assert_eq!(credentials.len(), 3);

        let cred = credentials.get(0).unwrap();
        assert_eq!(cred.profile, "tanaka");
        assert_eq!(cred.lines, vec!["foobarbaz"]);

        let cred = credentials.get(1).unwrap();
        assert_eq!(cred.profile, "takahashi");
        assert_eq!(cred.lines, vec!["foo", "bar"]);

        let cred = credentials.get(2).unwrap();
        assert_eq!(cred.profile, "saito");
        assert!(cred.lines.is_empty());
    }
}
