#[macro_use]
extern crate serde_derive;

extern crate ansi_term;
extern crate clap;
extern crate regex;

use ansi_term::Style;
use clap::{App, Arg};
use regex::Regex;

mod config;
mod fmt;
mod show;
mod story;
mod thelog;
mod tracker;

use config::Config;
use fmt::markdown;
use show::*;
use thelog::changelog::Changelog;
use thelog::commit::{Commit, Commits};
use thelog::fetch_log;
use tracker::{client, jira, rally};

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
        .arg(
            Arg::with_name("tracker")
                .long("tracker")
                .value_name("tracker name")
                .help("Inform which tracker to be used")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tracker-url")
                .long("tracker-url")
                .value_name("tracker URL")
                .help("The URL for stories lookup")
                .takes_value(true)
                .required(true)
                .requires("tracker"),
        )
        .arg(
            Arg::with_name("pattern")
            .long("pattern")
            .value_name("pattern regex")
            .help("The story pattern")
            .takes_value(true)
            .required(true),
        )
        .get_matches();

    let repository_dir = matches.value_of("repository").unwrap();
    let range = match matches.value_of("range") {
        Some(range) => range,
        None => "HEAD",
    };
    let tracker = matches.value_of("tracker").unwrap();
    let tracker_url = matches.value_of("tracker-url").unwrap();
    let raw_pattern = matches.value_of("pattern").unwrap();

    let story_pattern = format!(r"^({})\s*", raw_pattern);

    let lookup_tracker = if tracker.to_lowercase() == "jira" {
        jira::Jira::new(
            client::http_client("TRACKER_USER", "TRACKER_PWD"),
            tracker_url.to_string(),
            story_pattern,
        )
    } else {
        rally::Rally::new(
            client::http_client("TRACKER_USER", "TRACKER_PWD"),
            "https://rally1.rallydev.com/slm/webservice/v2.0/hierarchicalrequirement".to_string(),
            story_pattern,
        )
    };

    let config = Config::default();

    let tags_pattern =
        vec![lookup_tracker.pattern().to_string(), config.tags_pattern()].join(&config.separator);

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

    let changelog_file = markdown::create(
        &Changelog::create(some_stuff, range),
        lookup_tracker,
        matches.value_of("file"),
    );

    show(format!(
        "{} created!",
        Style::new().bold().paint(changelog_file.to_string())
    ));
}
