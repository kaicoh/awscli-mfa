use crate::Result;

use anyhow::anyhow;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const PROFILE_CREDENTIALS: &str = r"^\[(.+)\]$";
pub const PROFILE_CONFIG: &str = r"^\[profile\s+(.+)\]$";

#[derive(Debug, Clone)]
struct Fragment {
    profile: String,
    lines: Vec<String>,
}

impl Fragment {
    fn format<F: Fn(&str) -> String>(&self, f: F) -> String {
        format!("{}\n{}", f(&self.profile), self.lines.join("\n"))
    }
}

#[derive(Debug)]
pub struct Content {
    fragments: Vec<Fragment>,
}

impl Content {
    pub fn get(&self, profile: &str, key: &str) -> Result<&str> {
        self.get_fragment(profile)?
            .lines
            .iter()
            .find_map(|line| {
                line.split_once('=').and_then(|(k, v)| {
                    if k.trim() == key {
                        Some(v.trim())
                    } else {
                        None
                    }
                })
            })
            .ok_or(anyhow!("Not Found key: {} in profile: {}", key, profile))
    }

    pub fn add(self, profile: &str) -> Result<Self> {
        let Self { mut fragments } = self.remove_fragment(profile)?;

        fragments.push(Fragment {
            profile: profile.into(),
            lines: vec![],
        });

        Ok(Self { fragments })
    }

    pub fn set(self, profile: &str, key: &str, value: &str) -> Result<Self> {
        let mut lines = self
            .get_fragment(profile)?
            .lines
            .clone()
            .into_iter()
            .filter(|line| match line.split_once('=') {
                Some((k, _)) => k.trim() != key,
                None => true,
            })
            .collect::<Vec<String>>();

        lines.push(format!("{key} = {value}"));

        let fragment = Fragment {
            profile: profile.into(),
            lines,
        };

        self.set_fragment(fragment)
    }

    pub fn remove(self, profile: &str, key: &str) -> Result<Self> {
        let lines = self
            .get_fragment(profile)?
            .lines
            .clone()
            .into_iter()
            .filter(|line| match line.split_once('=') {
                Some((k, _)) => k.trim() != key,
                None => true,
            })
            .collect::<Vec<String>>();

        let fragment = Fragment {
            profile: profile.into(),
            lines,
        };

        self.set_fragment(fragment)
    }

    pub fn copy(self, src: &str, dst: &str) -> Result<Self> {
        let fragment = Fragment {
            profile: dst.into(),
            lines: self.get_fragment(src)?.lines.to_vec(),
        };
        self.set_fragment(fragment)
    }

    pub fn write<F: Fn(&str) -> String>(&self, path: &Path, fmt: &F) -> Result<()> {
        std::fs::write(path, self.format(fmt)).map_err(|e| {
            anyhow!(
                "Error writing to \"{}\". {}",
                path.to_str().unwrap_or("unknown path"),
                e
            )
        })
    }

    fn set_fragment(self, fragment: Fragment) -> Result<Self> {
        let profile = fragment.profile.as_str();
        let Self { mut fragments } = self.remove_fragment(profile)?;

        fragments.push(fragment);

        Ok(Self { fragments })
    }

    fn get_fragment(&self, profile: &str) -> Result<&Fragment> {
        self.fragments
            .iter()
            .find(|f| f.profile == profile)
            .ok_or(anyhow!("Cannot find profile: {profile}"))
    }

    fn remove_fragment(self, profile: &str) -> Result<Self> {
        let fragments = self
            .fragments
            .into_iter()
            .filter(|f| f.profile != profile)
            .collect::<Vec<Fragment>>();

        Ok(Self { fragments })
    }

    fn format<F: Fn(&str) -> String>(&self, fmt: &F) -> String {
        self.fragments
            .iter()
            .map(|f| f.format(fmt))
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

#[derive(Debug, Default)]
pub struct ContentBuilder<'a> {
    reg_profile: Option<&'a str>,
    path: Option<&'a Path>,
}

impl<'a> ContentBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_reg_profile(self, reg_profile: &'a str) -> Self {
        Self {
            reg_profile: Some(reg_profile),
            ..self
        }
    }

    pub fn set_path(self, path: &'a Path) -> Self {
        Self {
            path: Some(path),
            ..self
        }
    }

    pub fn load(self) -> Result<Content> {
        if self.reg_profile.is_none() {
            return Err(anyhow!("reg_profile is not set"));
        }
        let reg_profile = self.reg_profile.unwrap();

        if self.path.is_none() {
            return Err(anyhow!("path is not set"));
        }
        let path = self.path.unwrap();

        let reader = BufReader::new(File::open(path)?);
        let mut fragments: Vec<Fragment> = Vec::new();
        let mut profile = "".to_string();
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if let Some(_profile) = capture(reg_profile, &line) {
                if !profile.is_empty() {
                    fragments.push(Fragment {
                        profile: profile.clone(),
                        lines: lines.clone(),
                    });
                }

                profile = _profile.to_string();
                lines = Vec::new();
            } else if !line.is_empty() {
                lines.push(line);
            }
        }

        if !profile.is_empty() {
            fragments.push(Fragment { profile, lines });
        }

        Ok(Content { fragments })
    }
}

fn capture<'a, 'b>(pattern: &'a str, line: &'b str) -> Option<&'b str> {
    let line = line.trim();

    if line == "[default]" {
        return Some("default");
    }

    Regex::new(pattern)
        .unwrap()
        .captures(line)
        .and_then(|caps| caps.get(1))
        .map(|mat| mat.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod content {
        use super::*;
        use std::path::Path;

        fn build_content() -> Content {
            let path = Path::new("mock/test_config");

            ContentBuilder::new()
                .set_path(path)
                .set_reg_profile(PROFILE_CONFIG)
                .load()
                .unwrap()
        }

        #[test]
        fn it_gets_file_content_from_builder() {
            let content = build_content();
            assert_eq!(content.fragments.len(), 2);

            let fragment = content.fragments.get(0).unwrap();
            assert_eq!(fragment.profile, "default");
            assert_eq!(
                fragment.lines,
                vec!["region = us-east-1".to_owned(), "output = yaml".to_owned(),]
            );

            let fragment = content.fragments.get(1).unwrap();
            assert_eq!(fragment.profile, "test");
            assert_eq!(
                fragment.lines,
                vec![
                    "region = ap-northeast-1".to_owned(),
                    "output = json".to_owned(),
                ]
            );
        }

        #[test]
        fn it_gets_profile_value_from_key() {
            let content = build_content();
            assert_eq!(content.get("test", "region").unwrap(), "ap-northeast-1");
            assert!(content.get("unknown", "region").is_err());
            assert!(content.get("test", "unknown").is_err());
        }

        #[test]
        fn it_adds_empty_profile() {
            let content = build_content().add("test_v2").unwrap();

            assert_eq!(content.fragments.len(), 3);

            let fragment = content.fragments.get(2).unwrap();
            assert_eq!(fragment.profile, "test_v2");
            assert!(fragment.lines.is_empty());
        }

        #[test]
        fn it_sets_profile_value() {
            let content = build_content()
                .set("default", "mfa_serial", "ABCDEFGHIJKLMNOPQRST")
                .unwrap();

            assert_eq!(
                content.get("default", "mfa_serial").unwrap(),
                "ABCDEFGHIJKLMNOPQRST",
            );
        }

        #[test]
        fn it_removes_profile_value() {
            let content = build_content().remove("test", "region").unwrap();
            assert!(content.get("test", "region").is_err());
        }

        #[test]
        fn it_copies_profile() {
            let content = build_content().copy("test", "test_v2").unwrap();

            assert_eq!(content.fragments.len(), 3);

            let fragment = content.fragments.get(2).unwrap();
            assert_eq!(fragment.profile, "test_v2");
            assert_eq!(
                fragment.lines,
                vec![
                    "region = ap-northeast-1".to_owned(),
                    "output = json".to_owned(),
                ]
            );
        }
    }

    mod fn_capture {
        use super::*;

        #[test]
        fn it_capture_profile_line_from_credentials() {
            let pattern = PROFILE_CREDENTIALS;

            let profile = "[default]";
            assert_eq!(capture(pattern, profile), Some("default"));

            let profile = "[tanaka]";
            assert_eq!(capture(pattern, profile), Some("tanaka"));

            let profile = "[suzuki]";
            assert_eq!(capture(pattern, profile), Some("suzuki"));

            let profile = " [satoh]   ";
            assert_eq!(capture(pattern, profile), Some("satoh"));

            let non_profile = "access_key_id = AAAAAAAAAAAAAAAAAA";
            assert_eq!(capture(pattern, non_profile), None);

            let non_profile = "session_token = abcde[fghijk]lmn";
            assert_eq!(capture(pattern, non_profile), None);
        }

        #[test]
        fn it_capture_profile_line_from_config() {
            let pattern = PROFILE_CONFIG;

            let profile = "[default]";
            assert_eq!(capture(pattern, profile), Some("default"));

            let profile = "[profile tanaka]";
            assert_eq!(capture(pattern, profile), Some("tanaka"));

            let profile = "[profile suzuki]";
            assert_eq!(capture(pattern, profile), Some("suzuki"));

            let profile = " [profile satoh]   ";
            assert_eq!(capture(pattern, profile), Some("satoh"));

            let non_profile = "region = ap-northeast-1";
            assert_eq!(capture(pattern, non_profile), None);

            let non_profile = "foo = bar[profile baz]foobar";
            assert_eq!(capture(pattern, non_profile), None);
        }
    }
}
