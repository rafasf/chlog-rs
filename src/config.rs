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
                Tag::from(r"chore\(?(\w+)?\)?:?\s*", "Chore"),
                Tag::from(r"doc\(?(\w+)?\)?:?\s*", "Documentation"),
                Tag::from(r"style\(?(\w+)?\)?:?\s*", "Style"),
                Tag::from(r"refactor\(?(\w+)?\)?:?\s*", "Refactor"),
                Tag::from(r"feat\(?(\w+)?\)?:?\s*", "Feature"),
                Tag::from(r"ci\(?(\w+)?\)?:?\s*", "CI"),
            ],
            format: format!("--pretty=format:%s{s}%an{s}%h{s}%b", s = separator),
            separator: separator.to_string(),
        }
    }
}
