extern crate regex;
// TODO;
//  * Changelog to have two sections:
//    1. Stories summary
//    2. All the commits by tag
//  * Create a Markdown formatter
//  * Add proper claprs

use std::env;
use std::process::{Command, Output};
use regex::Regex;

pub mod commit;
pub mod changelog;

use commit::{Commit, Commits};
use changelog::Changelog;

fn main() {
  let args: Vec<String> = env::args().collect();
  let repository_dir = &args[1];
  let range = &args[2];

  // User config
  let tags_re = Regex::new(r"^(feat):\s*|^(chore):\s*|^(test):\s*").unwrap();

  // App config
  let separator = "|";
  let format = format!("--pretty=format:%s{s}%an{s}%h", s = separator);

  println!("Getting log in: {:?}", repository_dir);

  let output = fetch_log(&repository_dir, &format, &range);

  let some_stuff: Commits = String::from_utf8_lossy(&output.stdout)
    .split("\n")
    .map(|raw_commit| Commit::from(raw_commit, separator, &tags_re))
    .collect();

  let change = Changelog::create(some_stuff, range);

  println!("## {:?}, {}", change.title, change.created_at);
  for (tag, commits) in change.commits_by_tag() {
    println!("### {:?}", tag);
    commits.iter().for_each(|commit| println!("* {} ({})", commit.subject, commit.hash))
  }
}

fn fetch_log(repository_dir: &str, format: &str, range: &str) -> Output {
  Command::new("git")
    .arg("--git-dir")
    .arg(repository_dir)
    .arg("log")
    .arg("--oneline")
    .arg(format)
    .arg(range)
    .output()
    .unwrap_or_else(|e| panic!("Failed to get commits: {}", e))
}
