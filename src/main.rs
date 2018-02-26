#[macro_use]
extern crate serde_derive;

extern crate ansi_term;
extern crate clap;
extern crate regex;

use std::process::{Command, Output};
use regex::Regex;
use clap::{App, Arg};
use ansi_term::Style;

pub mod commit;
pub mod changelog;
pub mod fmt;
pub mod tracker;
mod story;
mod show;

use commit::{Commit, Commits};
use changelog::Changelog;
use fmt::markdown;
use tracker::rally;
use show::*;

fn main() {
    let matches = App::new("Changelog")
        .version("0.1.0")
        .arg(
            Arg::with_name("repository")
                .short("r")
                .long("repository")
                .value_name("repository path")
                .help("The path to the repository")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("range")
                .short("n")
                .long("range")
                .value_name("initial-hash..final-hash")
                .help("Range of commits to include (using Git style from..to)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("changelog output file name")
                .help("The name of the file to be created")
                .takes_value(true),
        )
        .get_matches();

    let repository_dir = matches.value_of("repository").unwrap();
    let range = match matches.value_of("range") {
        Some(range) => range,
        None => "HEAD",
    };

    // User config
    let tags_re = Regex::new(r"^(US\w+)\s*|^(feat):\s*|^(chore):\s*|^(test):\s*").unwrap();

    // App config
    let separator = "|";
    let format = format!("--pretty=format:%s{s}%an{s}%h", s = separator);

    show(format!(
        "Fetching log in {}",
        Style::new().bold().paint(repository_dir)
    ));

    let output = fetch_log(&repository_dir, &format, &range);

    let some_stuff: Commits = String::from_utf8_lossy(&output.stdout)
        .split("\n")
        .map(|raw_commit| Commit::from(raw_commit, separator, &tags_re))
        .collect();

    let changelog_file = markdown::create(
        &Changelog::create(some_stuff, range),
        rally::Rally,
        matches.value_of("file"),
    );

    show(format!(
        "{} created!",
        Style::new().bold().paint(changelog_file.to_string())
    ));
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
