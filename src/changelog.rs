extern crate regex;
extern crate chrono;

use changelog::chrono::prelude::*;
use commit::Commits;

#[derive(Debug)]
pub struct Changelog {
  commits: Commits,
  title: String,
  created_at: Date<Utc>,
}

impl Changelog {
  pub fn create(commits: Commits, range: &str) -> Changelog {
    Changelog {
      commits: commits,
      title: range.to_string(),
      created_at: Utc::today()
    }
  }

  pub fn commits_by_tag(&self) -> Commits {
    // delegate to commits
    unimplemented!();
  }

  pub fn stories(&self) -> Vec<String> {
    unimplemented!();
  }
}

mod test {
  use changelog::*;
  use commit::*;

  #[test]
  #[ignore]
  fn returns_commits_grouped_by_tag() {
    let commits = vec![
      Commit { tag: "t1".into(), subject: "".into(), author: "".into(), hash: "".into() },
      Commit { tag: "t2".into(), subject: "".into(), author: "".into(), hash: "".into() },
      Commit { tag: "t1".into(), subject: "".into(), author: "".into(), hash: "".into() }
    ];
    let changelog = Changelog::create(commits, "");

    changelog.commits_by_tag();
  }
}
