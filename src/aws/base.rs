use crate::Result;

use anyhow::anyhow;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub lines: Vec<String>,
}

pub trait ConfigFileBase: Sized {
    const FILENAME: &'static str;
    const PROFILE_PATTERN: &'static str;

    fn new() -> Result<Self> {
        let path = Self::filepath()?;
        Self::load(path.as_path())
    }

    fn set(self, profile: Profile) -> Self {
        let name = profile.name.as_str();
        let mut profiles: Vec<Profile> = self
            .profiles()
            .iter()
            .filter(|p| p.name != name)
            .cloned()
            .collect();

        profiles.push(profile);

        Self::build(profiles)
    }

    fn get(&self, name: &str) -> Result<Profile> {
        self.profiles()
            .iter()
            .find(|p| p.name == name)
            .cloned()
            .ok_or(anyhow!(
                "Failed to get profile: {} in ~/.aws/{}",
                name,
                Self::FILENAME
            ))
    }

    fn save(&self) -> Result<()> {
        let path = Self::filepath()?;
        self.write(path.as_path())
    }

    fn build(profiles: Vec<Profile>) -> Self;

    fn load(path: &Path) -> Result<Self> {
        let reader = BufReader::new(File::open(path)?);
        let mut profiles: Vec<Profile> = Vec::new();
        let mut name = "".to_string();
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if let Some(n) = capture(Self::PROFILE_PATTERN, &line) {
                if !name.is_empty() {
                    profiles.push(Profile {
                        name: name.clone(),
                        lines: lines.clone(),
                    });
                }

                name = n.to_string();
                lines = Vec::new();
            } else if !line.is_empty() {
                lines.push(line);
            }
        }

        if !name.is_empty() {
            profiles.push(Profile { name, lines });
        }

        Ok(Self::build(profiles))
    }

    fn filepath() -> Result<PathBuf> {
        dirs::home_dir()
            .ok_or(anyhow!("Failed to get home directory."))
            .map(|p| p.join(".aws").join(Self::FILENAME))
    }

    fn fmt_profile(profile: &Profile) -> String;

    fn profiles(&self) -> &[Profile];

    fn fmt(&self) -> String {
        self.profiles()
            .iter()
            .map(Self::fmt_profile)
            .collect::<Vec<String>>()
            .join("\n\n")
    }

    fn write(&self, path: &Path) -> Result<()> {
        std::fs::write(path, self.fmt())
            .map_err(|e| anyhow!("Error writing to ~/.aws/{}: {}", Self::FILENAME, e))
    }
}

fn capture<'a>(pattern: &'static str, line: &'a str) -> Option<&'a str> {
    if line.trim() == "[default]" {
        return Some("default");
    }

    Regex::new(pattern)
        .unwrap()
        .captures(line)
        .and_then(|caps| caps.get(1))
        .map(|mat| mat.as_str())
}
