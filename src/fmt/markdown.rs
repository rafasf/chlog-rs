use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Display, Path};

use thelog::changelog::Changelog;
use thelog::commit::Commit;
use tracker::Tracker;

pub fn create<'a>(
    changelog: &Changelog,
    tracker: Option<Box<Tracker>>,
    output_file: Option<&'a str>,
) -> Display<'a> {
    let file_path = Path::new(output_file.unwrap_or("CHANGELOG.md"));
    let file = match File::create(&file_path) {
        Ok(file) => file,
        Err(e) => panic!(
            "couldn't create file {}: {}",
            file_path.display(),
            e.description()
        ),
    };

    write_title_into(&file, &changelog);
    if tracker.is_some() {
        write_story_summary_into(&file, &changelog.stories(), &tracker.unwrap());
    }
    write_commits_into(&file, &changelog.commits_by_tag());

    file_path.display()
}

fn write_title_into(mut file: &File, changelog: &Changelog) {
    writeln!(file, "## {} ({})", changelog.title, changelog.created_at).unwrap();
}

fn write_story_summary_into(
    mut file: &File,
    story_identifiers: &HashSet<String>,
    tracker: &Box<Tracker>,
) {
    writeln!(file, "\n### {}", "Story Summary").unwrap();

    for story_identifier in story_identifiers {
        let story = tracker.details_of(&story_identifier);

        match story.link {
            Some(link) => {
                writeln!(file, "* [{}]({}) {}", story.id, link, story.name.unwrap()).unwrap()
            }
            None => writeln!(file, "* {}", story_identifier).unwrap(),
        };
    }
}

fn write_commits_into(mut file: &File, commits_by_tag: &HashMap<String, Vec<Commit>>) {
    for (tag, commits) in commits_by_tag {
        writeln!(file, "\n#### {}", tag).unwrap();

        for commit in commits {
            writeln!(file, "* {}", commit.subject).unwrap();
        }
    }
}
