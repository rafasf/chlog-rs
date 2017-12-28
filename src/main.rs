extern crate regex;
extern crate clap;
// TODO;
//  * Changelog to have two sections:
//    1. Stories summary
//    2. All the commits by tag
//  * Create a Markdown formatter

use std::process::{Command, Output};
use regex::Regex;
use clap::{Arg, App};

pub mod commit;
pub mod changelog;
pub mod fmt;

use commit::{Commit, Commits};
use changelog::Changelog;
use fmt::markdown;

fn main() {
  let matches = App::new("Changelog")
    .version("0.1.0")
    .arg(Arg::with_name("repository")
         .short("r")
         .long("repository")
         .value_name("repository path")
         .help("The path to the repository")
         .required(true)
         .takes_value(true))
    .arg(Arg::with_name("range")
         .long("range")
         .value_name("initial-hash..final-hash")
         .help("Range of commits to include (using Git style from..to)")
         .takes_value(true))
    .get_matches();

  let repository_dir = matches.value_of("repository").unwrap();
  let range = match matches.value_of("range") {
    Some(range) => range,
    None => "HEAD"
  };

  // User config
  let tags_re = Regex::new(r"^(US\w+)\s*|^(feat):\s*|^(chore):\s*|^(test):\s*").unwrap();

  // App config
  let separator = "|";
  let format = format!("--pretty=format:%s{s}%an{s}%h", s = separator);

  println!("Getting log in: {:?}", repository_dir);

  let output = fetch_log(&repository_dir, &format, &range);

  let some_stuff: Commits = String::from_utf8_lossy(&output.stdout)
    .split("\n")
    .map(|raw_commit| Commit::from(raw_commit, separator, &tags_re))
    .collect();

  markdown::create(
    &Changelog::create(some_stuff, range),
    &Regex::new(r"^US\w+").unwrap());
}

fn fetch_log(repository_dir: &str, format: &str, range: &str) -> Output {
  Command::new("git")
    .arg("--git-dir")
    .arg(repository_dir)
    .arg("log")
    .arg("--oneline")
    .arg("--no-merges")
    .arg(format)
    .arg(range)
    .output()
    .unwrap_or_else(|e| panic!("Failed to get commits: {}", e))
}
