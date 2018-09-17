use thelog::tag::*;

pub struct NewCommit {
    pub tag: TagMatch,
    pub hash: String,
    pub subject: String,
    pub author: String,
}

impl NewCommit {
    pub fn from(raw_commit: &str, separator: &str, tags: &Vec<Tag>) -> Self {
        let commit_parts = raw_commit.split(separator).collect::<Vec<&str>>();
        let (subject, author, hash, body) = (
            commit_parts[0],
            commit_parts[1],
            commit_parts[2],
            commit_parts[3],
        );

        let tag_match = tag_in(&format!("{} {}", subject, body), tags);
        let clean_subject = tag_match.re().replace(subject, "");

        NewCommit {
            tag: tag_match,
            hash: hash.into(),
            subject: clean_subject.into_owned(),
            author: author.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use thelog::new_commit::*;
    use thelog::tag::*;

    #[test]
    fn creates_commit_from_string_with_general_tag() {
        let commit = NewCommit::from("Sample message here-author-hash-body", "-", &vec![]);
        let expected_commit = NewCommit {
            tag: TagMatch {
                tag: GENERAL_TAG.clone(),
                component: None,
            },
            hash: "hash".to_string(),
            subject: "Sample message here".to_string(),
            author: "author".to_string(),
        };

        assert_eq!(expected_commit.tag.description(), commit.tag.description());
        assert_eq!(expected_commit.hash, commit.hash);
        assert_eq!(expected_commit.subject, commit.subject);
        assert_eq!(expected_commit.author, commit.author);
    }

    #[test]
    fn creates_commit_with_matched_tag_in_subject() {
        let tags = vec![
            Tag::from(r"[chore]\s*", "Chore"),
            Tag::from(r"(US\w+)\s*", "Story"),
        ];

        let chore_commit =
            NewCommit::from("Sample message [chore]-author-hash-body here", "-", &tags);
        let story_commit =
            NewCommit::from("Sample message-author-hash-Related to US123", "-", &tags);

        assert_eq!("Chore", chore_commit.tag.description());
        assert_eq!("Story", story_commit.tag.description());
    }

    #[test]
    fn creates_commit_with_matched_tag_in_body() {
        let tags = vec![
            Tag::from(r"[chore]\s*", "Chore"),
            Tag::from(r"(US\w+)\s*", "Story"),
        ];

        let story_commit =
            NewCommit::from("Sample message-author-hash-Related to US123", "-", &tags);

        assert_eq!("Story", story_commit.tag.description());
    }

    #[test]
    fn creates_commit_with_last_matched_tag_when_multiple() {
        let tags = vec![
            Tag::from(r"[chore]\s*", "Chore"),
            Tag::from(r"(US\w+)\s*", "Story"),
        ];

        let story_commit = NewCommit::from(
            "[chore] Sample message-author-hash-Related to US123",
            "-",
            &tags,
        );

        assert_eq!("Story", story_commit.tag.description());
    }
}
