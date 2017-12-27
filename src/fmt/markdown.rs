extern crate regex;

use regex::Regex;
use changelog::Changelog;

pub fn create_from(changelog: &Changelog) {
  println!("## {:?}, {}", changelog.title, changelog.created_at);

  println!("\n## Story Summary");
  changelog.stories(&Regex::new(r"^test").unwrap())
    .iter()
    .for_each(|story| println!("* {}", story));

  for (tag, commits) in changelog.commits_by_tag() {
    println!("\n### {}", tag);
    commits.iter().for_each(|commit| println!("* {} ({})", commit.subject, commit.hash));
  }
}
