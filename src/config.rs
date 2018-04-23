extern crate regex;

use regex::Regex;

#[derive(Debug)]
pub struct Tag {
    pattern: String,
    description: String,
}

impl Tag {
    pub fn new(pattern: &str, description: &str) -> Self {
        Tag {
            pattern: pattern.into(),
            description: description.into(),
        }
    }

    fn regex(&self) -> Regex {
        Regex::new(&self.pattern).unwrap()
    }
}

#[derive(Debug)]
pub struct Config {
    tags: Vec<Tag>,
    pub format: String,
    pub separator: String,
}

impl Config {
    pub fn default() -> Self {
        let separator = "|";

        Config {
            tags: vec![
                Tag::new(r"^(chore):\s*", "Chore"),
                Tag::new(r"^(doc):\s*", "Documentation"),
                Tag::new(r"^(style):\s*", "Style"),
                Tag::new(r"^(refactor):\s*", "Refactor"),
            ],
            format: format!("--pretty=format:%s{s}%an{s}%h", s = separator),
            separator: separator.to_string(),
        }
    }

    pub fn tags_pattern(&self) -> String {
        let tags: Vec<String> = self.tags.iter().map(|tag| tag.pattern.clone()).collect();

        tags.join("|")
    }
}

#[cfg(test)]
mod test {
    use config::{Config, Tag};

    #[test]
    #[should_panic]
    fn fails_when_regex_is_not_valid() {
        Tag::new(r"^(chore", "Chore").regex();
    }

    #[test]
    fn creates_with_default_tags() {
        let config = Config::default();
        assert_eq!(config.tags.len(), 4);
    }

    #[test]
    fn returns_regex_matching_all_tags() {
        let config = Config::default();
        assert_eq!(
            r"^(chore):\s*|^(doc):\s*|^(style):\s*|^(refactor):\s*",
            config.tags_pattern()
        );
    }
}
