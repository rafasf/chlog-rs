use thelog::tag::*;

#[derive(Debug)]
pub struct Config {
    pub tags: Vec<Tag>,
    pub format: String,
    pub separator: String,
}

impl Config {
    pub fn default() -> Self {
        let separator = "|";

        Config {
            tags: vec![
                Tag::from(r"(chore):\s*", "Chore"),
                Tag::from(r"(doc):\s*", "Documentation"),
                Tag::from(r"(style):\s*", "Style"),
                Tag::from(r"feat\(?(\w+)?\)?:\s*", "Feature"), //example of optional component
                Tag::from(r"(refactor):\s*", "Refactor"),
            ],
            format: format!("--pretty=format:%s{s}%an{s}%h{s}%b", s = separator),
            separator: separator.to_string(),
        }
    }
}
