use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Profile {
    name: String,
    lines: Vec<String>,
}

impl Profile {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            lines: vec![],
        }
    }

    pub fn push(self, line: &str) -> Self {
        let mut lines = self.lines;
        lines.push(line.into());

        Self { lines, ..self }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rename(self, name: &str) -> Self {
        Self {
            name: name.into(),
            ..self
        }
    }

    pub fn format<F: Fn(&str) -> String>(&self, f: F) -> String {
        format!("{}\n{}", f(&self.name), self.lines.join("\n"))
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.pairs().get(key).copied()
    }

    pub fn set(self, key: &str, value: &str) -> Self {
        let mut lines = self.remove_line(key);
        lines.push(format!("{key} = {value}"));

        Self { lines, ..self }
    }

    pub fn remove(self, key: &str) -> Self {
        Self {
            lines: self.remove_line(key),
            ..self
        }
    }

    fn remove_line(&self, key: &str) -> Vec<String> {
        self.lines
            .clone()
            .into_iter()
            .filter(|line| match line.split_once('=') {
                Some((k, _)) => k.trim() != key,
                None => true,
            })
            .collect()
    }

    fn pairs(&self) -> HashMap<&str, &str> {
        let iterator = self
            .lines
            .iter()
            .filter_map(|line| line.split_once('=').map(|(k, v)| (k.trim(), v.trim())));

        HashMap::from_iter(iterator)
    }
}

impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        let self_pairs = self.pairs();
        let other_pairs = other.pairs();

        if self_pairs.len() != other_pairs.len() {
            return false;
        }

        self_pairs.iter().all(|(&k, &v)| match other_pairs.get(k) {
            Some(&_v) => v == _v,
            None => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build() -> Profile {
        Profile::new("test")
            .set("region", "us-east-1")
            .set("output", "json")
    }

    #[test]
    fn it_formats_to_string() {
        let p = build();
        let fmt = |p: &str| format!("[{p}]");
        assert_eq!(p.format(fmt), "[test]\nregion = us-east-1\noutput = json");
    }

    #[test]
    fn it_gets_value_from_key() {
        let p = build();
        assert_eq!(p.get("region"), Some("us-east-1"));
        assert_eq!(p.get("output"), Some("json"));
        assert_eq!(p.get("unknown"), None);
    }

    #[test]
    fn it_sets_value() {
        let p = build();
        let p = p.set("foo", "bar");

        // it adds key-value pair when key doesn't exist
        assert_eq!(p.get("foo"), Some("bar"));

        let p = p.set("foo", "foobar");

        // it overwrites when key exists
        assert_eq!(p.get("foo"), Some("foobar"));
    }

    #[test]
    fn it_removes_value() {
        let p = build();
        assert_eq!(p.get("region"), Some("us-east-1"));

        let p = p.remove("region");
        assert_eq!(p.get("region"), None);
    }

    #[test]
    fn it_equals_when_profile_and_key_values_are_all_matched() {
        let p0 = build();
        let p1 = Profile::new("test")
            .set("output", "json")
            .set("region", "us-east-1");

        let fmt = |p: &str| format!("[{p}]");

        // Since lines order is not same, formats are defferent.
        assert_ne!(p0.format(fmt), p1.format(fmt));

        // But profile and key-value pairs are same, Both are equal.
        assert_eq!(p0, p1);

        let p2 = Profile::new("test_v2")
            .set("region", "us-east-1")
            .set("output", "json");

        assert_ne!(p0, p2);

        let p3 = Profile::new("test")
            .set("output", "json")
            .set("region", "us-east-1")
            .set("foo", "bar");

        assert_ne!(p0, p3);
    }
}
