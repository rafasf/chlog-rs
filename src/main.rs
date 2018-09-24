#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate ansi_term;
extern crate clap;
extern crate regex;
extern crate simplelog;

use clap::{App, Arg, ArgMatches};
use simplelog::*;

mod config;
mod fmt;
mod story;
mod thelog;
mod tracker;

use config::Config;
use fmt::markdown;
use thelog::changelog::Changelog;
use thelog::commit::{Commit, Commits};
use thelog::fetch_log;
use thelog::tag::Tag;
use tracker::{tracker_for, Tracker};

fn main() {
    let log_level = std::env::var("RUST_LOG").unwrap_or("info".into());
    CombinedLogger::init(vec![
        TermLogger::new(log_level.parse().unwrap(), simplelog::Config::default()).unwrap(),
    ]).unwrap();

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
        ).arg(
            Arg::with_name("range")
                .short("n")
                .long("range")
                .value_name("initial-hash..final-hash")
                .help("Range of commits to include (using Git style from..to)")
                .takes_value(true),
        ).arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("changelog output file name")
                .help("The name of the file to be created")
                .takes_value(true),
        ).arg(
            Arg::with_name("tracker")
                .long("tracker")
                .value_name("tracker name")
                .possible_values(&["jira", "rally"])
                .help("Inform which tracker to be used")
                .requires_all(&["tracker-url", "pattern"])
                .takes_value(true),
        ).arg(
            Arg::with_name("tracker-url")
                .long("tracker-url")
                .value_name("tracker URL")
                .help("The URL for stories lookup")
                .takes_value(true),
        ).arg(
            Arg::with_name("pattern")
                .long("pattern")
                .value_name("pattern regex")
                .help("The story pattern")
                .takes_value(true),
        ).get_matches();

    let repository_dir = matches.value_of("repository").unwrap();
    let range = match matches.value_of("range") {
        Some(range) => range,
        None => "HEAD",
    };

    let config = Config::default();

    let lookup_tracker = tracker_given(&matches);
    let tags = tags_given(&lookup_tracker, &config);

    info!("Repository: {}", repository_dir);
    debug!("Range: {}", range);
    debug!("Tags: {:?}", tags);

    let commits: Commits = fetch_log(&repository_dir, &config.format, &range)
        .iter()
        .map(|raw_commit| Commit::from(raw_commit, &config.separator, &tags))
        .collect();

    let changelog_file = markdown::create(
        &Changelog::create(commits, range),
        lookup_tracker,
        matches.value_of("file"),
    );

    info!("{} created!", changelog_file.to_string());
}

/// Creates a tracker if a names was provided.
///
/// If tracker does not exist or no name was given, `None` is returned.
fn tracker_given(matches: &ArgMatches) -> Option<Box<Tracker + 'static>> {
    match matches.value_of("tracker") {
        Some(tracker_name) => {
            let tracker_url = matches.value_of("tracker-url").unwrap();
            let raw_pattern = matches.value_of("pattern").unwrap();

            let story_pattern = format!(r"({})\s*", raw_pattern);

            tracker_for(tracker_name, tracker_url, story_pattern)
        }
        None => {
            warn!("Story links will not be created since a tracker wasn't provided");
            None
        }
    }
}

/// Combines configured tags with tracker's story pattern.
fn tags_given(lookup_tracker: &Option<Box<Tracker + 'static>>, config: &Config) -> Vec<Tag> {
    match lookup_tracker {
        Some(tracker) => vec![
            vec![Tag::from(tracker.pattern(), "Story")],
            config.tags.clone(),
        ].concat(),
        None => config.tags.clone(),
    }
}
