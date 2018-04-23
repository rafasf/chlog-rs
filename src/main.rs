#[macro_use]
extern crate serde_derive;

extern crate ansi_term;
extern crate clap;
extern crate regex;

use ansi_term::Style;
use clap::{App, Arg};
use regex::Regex;
use std::process::{Command, Output};

pub mod changelog;
pub mod commit;
mod config;
pub mod fmt;
mod show;
mod story;
pub mod tracker;

use changelog::Changelog;
use commit::{Commit, Commits};
use config::Config;
use fmt::markdown;
use show::*;
use tracker::{client, rally, Tracker};

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

    let config = Config::default();

    let tags_pattern = vec![
        rally::Rally::story_id_pattern().as_str().to_string(),
        config.tags_pattern(),
    ].join(&config.separator);

    let tags_re = Regex::new(&tags_pattern).unwrap();

    show(format!(
        "Fetching log in {}",
        Style::new().bold().paint(repository_dir)
    ));

    let output = fetch_log(&repository_dir, &config.format, &range);

    let some_stuff: Commits = String::from_utf8_lossy(&output.stdout)
        .split("\n")
        .map(|raw_commit| Commit::from(raw_commit, &config.separator, &tags_re))
        .collect();

    let rally_tracker = rally::Rally::new(client::http_client("RALLY_USER", "RALLY_PWD"));

    let changelog_file = markdown::create(
        &Changelog::create(some_stuff, range),
        rally_tracker,
        matches.value_of("file"),
    );

    show(format!(
        "{} created!",
        Style::new().bold().paint(changelog_file.to_string())
    ));
}

fn fetch_log(repository_dir: &str, format: &str, range: &str) -> Output {
    let git_dir = if repository_dir.contains(".git") {
        repository_dir.to_string()
    } else {
        format!("{}/.git", repository_dir)
    };

    let possible_log = Command::new("git")
        .arg("--git-dir")
        .arg(&git_dir)
        .arg("log")
        .arg("--oneline")
        .arg("--no-merges")
        .arg(format)
        .arg(range)
        .output()
        .expect("Failed to interact with Git.");

    if possible_log.status.success() {
        possible_log
    } else {
        show_err("Unable to get commits, please check the information provided.".to_string());
        show_err(format!("Repository: {}, range: {}", &git_dir, range));
        panic!();
    }
}
