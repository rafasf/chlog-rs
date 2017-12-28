extern crate regex;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use changelog::Changelog;

pub fn create(changelog: &Changelog, story_re: &Regex) {
  let file_path = Path::new("CHANGELOG.md");

  let mut file = match File::create(&file_path) {
    Ok(file) => file,
    Err(e) => panic!("couldn't create file: {}", e.description())
  };

  writeln!(file, "## {} @ {}", changelog.title, changelog.created_at);

  writeln!(file, "\n### {}", "Story Summary");
  changelog.stories(story_re)
    .iter()
    .for_each(|story| {
      writeln!(file, "* {}", story);
    });

  for (tag, commits) in changelog.commits_by_tag() {
    writeln!(file, "\n#### {}", tag);
    commits
      .iter()
      .for_each(|commit| {
        writeln!(file, "* {} ({})", commit.subject, commit.hash);
      });
  }
}
