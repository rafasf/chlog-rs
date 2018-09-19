extern crate chrono;

use self::chrono::prelude::*;
use std::collections::{HashMap, HashSet};
use thelog::commit::{Commit, Commits};

#[derive(Debug)]
pub struct Changelog {
    pub commits: Commits,
    pub title: String,
    pub created_at: Date<Utc>,
}

impl Changelog {
    pub fn create(commits: Commits, range: &str) -> Changelog {
        Changelog {
            commits: commits,
            title: range.to_string(),
            created_at: Utc::today(),
        }
    }

    pub fn commits_by_tag(&self) -> HashMap<String, Vec<Commit>> {
        let commits_by_tag =
            &self
                .commits
                .iter()
                .fold(HashMap::<String, Vec<Commit>>::new(), |mut acc, commit| {
                    acc.entry(commit.tag.description().into())
                        .or_insert(vec![])
                        .push(commit.clone());
                    acc
                });

        commits_by_tag.clone()
    }

    pub fn stories(&self) -> HashSet<String> {
        self.commits
            .iter()
            .filter_map(|commit| match commit.tag.description() {
                "Story" => commit.tag.component.clone(),
                _ => None,
            }).map(|comp| comp.to_string())
            .collect::<HashSet<String>>()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use thelog::changelog::Changelog;
    use thelog::commit::Commit;
    use thelog::tag::*;

    fn tag_match(description: &str, component: Option<String>) -> TagMatch {
        TagMatch {
            tag: Tag::from(".*", description),
            component: component,
        }
    }

    #[test]
    fn returns_commits_grouped_by_tag() {
        let commits = vec![
            Commit {
                tag: tag_match("t1", None),
                subject: "t1 commit".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: tag_match("t2", None),
                subject: "t2 commit".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: tag_match("t1", None),
                subject: "another t1 commit".into(),
                author: "".into(),
                hash: "".into(),
            },
        ];
        let commits_by_tag = Changelog::create(commits.clone(), "").commits_by_tag();

        assert_eq!(commits_by_tag.get("t1").unwrap()[0].subject, "t1 commit");
        assert_eq!(
            commits_by_tag.get("t1").unwrap()[1].subject,
            "another t1 commit"
        );
        assert_eq!(commits_by_tag.get("t2").unwrap()[0].subject, "t2 commit");
    }

    #[test]
    fn returns_unique_stories_given_a_pattern() {
        let commits = vec![
            Commit {
                tag: tag_match("Story", Some("US0192".into())),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: tag_match("doc", None),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: tag_match("Story", Some("US213".into())),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: tag_match("Story", Some("US0192".into())),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
        ];

        let changelog = Changelog::create(commits.clone(), "");

        let expected_stories: HashSet<String> =
            vec!["US0192".into(), "US213".into()].into_iter().collect();

        assert_eq!(expected_stories, changelog.stories());
    }
}
