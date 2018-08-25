extern crate chrono;
extern crate regex;

use self::chrono::prelude::*;
use self::regex::Regex;
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
        let commits_by_tag = &self.commits.iter().fold(
            HashMap::<String, Vec<Commit>>::new(),
            |mut acc, commit| {
                acc.entry(commit.tag.to_string()).or_insert(vec![]).push(
                    commit.clone(),
                );
                acc
            },
        );

        commits_by_tag.clone()
    }

    pub fn stories(&self, story_re: &Regex) -> HashSet<String> {
        let stories: Vec<String> = self.commits
            .iter()
            .filter(|commit| story_re.is_match(&commit.tag))
            .map(|commit| commit.tag.clone())
            .collect();

        stories.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use regex::Regex;
    use std::collections::{HashMap, HashSet};
    use thelog::changelog::Changelog;
    use thelog::commit::Commit;

    #[test]
    fn returns_commits_grouped_by_tag() {
        let commits = vec![
            Commit {
                tag: "t1".into(),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: "t2".into(),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: "t1".into(),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
        ];
        let changelog = Changelog::create(commits.clone(), "");

        let mut expected_groups = HashMap::new();
        expected_groups.insert("t1".into(), vec![commits[0].clone(), commits[2].clone()]);
        expected_groups.insert("t2".into(), vec![commits[1].clone()]);

        assert_eq!(expected_groups, changelog.commits_by_tag());
    }

    #[test]
    fn returns_unique_stories_given_a_pattern() {
        let story_re = Regex::new(r"^(US\w+)").unwrap();
        let commits = vec![
            Commit {
                tag: "US0192".into(),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: "doc".into(),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: "US213".into(),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
            Commit {
                tag: "US0192".into(),
                subject: "".into(),
                author: "".into(),
                hash: "".into(),
            },
        ];

        let changelog = Changelog::create(commits.clone(), "");

        let expected_stories: HashSet<String> =
            vec!["US0192".into(), "US213".into()].into_iter().collect();

        assert_eq!(expected_stories, changelog.stories(&story_re));
    }
}
