extern crate regex;
extern crate chrono;

use std::collections::HashMap;
use changelog::chrono::prelude::*;
use commit::{Commit, Commits};

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
      created_at: Utc::today()
    }
  }

  pub fn commits_by_tag(&self) -> HashMap<String, Vec<Commit>>  {
    let commits_by_tag = &self.commits
      .iter()
      .fold(HashMap::<String, Vec<Commit>>::new(), |mut acc, commit| {
        acc.entry(commit.tag.to_string()).or_insert(vec![]).push(commit.clone());
        acc
      });

    commits_by_tag.clone()
  }

  pub fn stories(&self) -> Vec<String> {
    unimplemented!();
  }
}

mod test {
  use changelog::Changelog;
  use commit::Commit;
  use std::collections::HashMap;

  #[test]
  fn returns_commits_grouped_by_tag() {
    let commits = vec![
      Commit { tag: "t1".into(), subject: "".into(), author: "".into(), hash: "".into() },
      Commit { tag: "t2".into(), subject: "".into(), author: "".into(), hash: "".into() },
      Commit { tag: "t1".into(), subject: "".into(), author: "".into(), hash: "".into() }
    ];
    let changelog = Changelog::create(commits.clone(), "");

    let mut expected_groups = HashMap::new();
    expected_groups.insert(
      "t1".into(),
      vec![commits[0].clone(), commits[2].clone()]);
    expected_groups.insert(
      "t2".into(),
      vec![commits[1].clone()]);

    assert_eq!(expected_groups, changelog.commits_by_tag());
  }
}
