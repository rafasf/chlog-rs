extern crate regex;

use std::env;
use std::process::Command;
use regex::Regex;

pub mod commit;

use commit::Commit;

fn main() {
  let args: Vec<String> = env::args().collect();
  let repository_dir = &args[1];

  let separator = "|";
  let tags_re = Regex::new("^none").unwrap();

  println!("Getting log in: {:?}", repository_dir);

  let output = Command::new("git")
    .arg("--git-dir")
    .arg(repository_dir)
    .arg("log")
    .arg("--oneline")
    .arg("--pretty=format:%s|%an|%h")
    .arg("-1")
    .output()
    .unwrap_or_else(|e| panic!("Failed to get commits: {}", e));

  let some_stuff: Vec<Commit> = String::from_utf8_lossy(&output.stdout)
    .split("\n")
    .map(|raw_commit| Commit::from(raw_commit, separator, &tags_re))
    .collect();

  println!("{:?}", some_stuff);
}
