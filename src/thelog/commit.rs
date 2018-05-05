extern crate regex;

use self::regex::Regex;

pub type Commits = Vec<Commit>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Commit {
    pub tag: String,
    pub hash: String,
    pub subject: String,
    pub author: String,
}

impl Commit {
    pub fn from(raw_commit: &str, separator: &str, tags_re: &Regex) -> Self {
        let raw_commit: Vec<&str> = raw_commit.split(separator).collect();
        let (raw_subject, author, hash) = (raw_commit[0], raw_commit[1], raw_commit[2]);

        let possible_tag = match tags_re.captures(raw_subject) {
            Some(tag) => tag.iter()
                .filter_map(|possible_tag| {
                    if possible_tag.is_some() {
                        possible_tag
                    } else {
                        None
                    }
                })
                .last(),
            None => None,
        };

        let (tag, subject) = match possible_tag {
            Some(tag) => (tag.as_str(), tags_re.replace(raw_subject, "").into_owned()),
            None => ("", raw_subject.to_string()),
        };

        Commit {
            tag: tag.to_string(),
            hash: hash.to_string(),
            subject: subject.to_string(),
            author: author.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use thelog::commit::*;

    #[test]
    fn creates_commit_from_string() {
        let no_re = Regex::new(r"^none").unwrap();
        let commit = Commit::from("Sample message here-author-hash", "-", &no_re);
        let expected_commit = Commit {
            tag: "".to_string(),
            hash: "hash".to_string(),
            subject: "Sample message here".to_string(),
            author: "author".to_string(),
        };

        assert_eq!(expected_commit.tag, commit.tag);
        assert_eq!(expected_commit.hash, commit.hash);
        assert_eq!(expected_commit.subject, commit.subject);
        assert_eq!(expected_commit.author, commit.author);
    }

    #[test]
    fn creates_commit_with_tag_from_string() {
        let tags_re = Regex::new(r"^(US\w+)\s*").unwrap();
        let commit = Commit::from("US123 Sample message here|author|hash", "|", &tags_re);

        let expected_commit = Commit {
            tag: "US123".to_string(),
            hash: "hash".to_string(),
            subject: "Sample message here".to_string(),
            author: "author".to_string(),
        };

        assert_eq!(expected_commit.tag, commit.tag);
        assert_eq!(expected_commit.hash, commit.hash);
        assert_eq!(expected_commit.subject, commit.subject);
        assert_eq!(expected_commit.author, commit.author);
    }

    #[test]
    fn creates_with_matching_tag_from_string() {
        let tags_re = Regex::new(r"^(US\w+)\s*|^(feat):\s*|^(chore):\s*").unwrap();
        let commit_feat = Commit::from("feat: Sample message here|author|hash", "|", &tags_re);

        let commit_story = Commit::from("US123 Sample message here|author|hash", "|", &tags_re);

        let commit_chore = Commit::from("chore: Sample message here|author|hash", "|", &tags_re);

        assert_eq!(commit_feat.tag, "feat");
        assert_eq!(commit_feat.subject, "Sample message here");
        assert_eq!(commit_story.tag, "US123");
        assert_eq!(commit_story.subject, "Sample message here");
        assert_eq!(commit_chore.tag, "chore");
        assert_eq!(commit_chore.subject, "Sample message here");
    }
}
