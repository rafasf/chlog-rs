use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Display, Path};

use thelog::changelog::Changelog;
use thelog::commit::Commit;
use tracker::Tracker;

pub fn create<'a, T>(changelog: &Changelog, tracker: T, output_file: Option<&'a str>) -> Display<'a>
where
    T: Tracker,
{
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
    write_story_summary_into(&file, &changelog.stories(&T::story_id_pattern()), &tracker);
    write_commits_into(&file, &changelog.commits_by_tag());

    file_path.display()
}

fn write_title_into(mut file: &File, changelog: &Changelog) {
    writeln!(file, "## {} ({})", changelog.title, changelog.created_at).unwrap();
}

fn write_story_summary_into<T>(mut file: &File, story_identifiers: &HashSet<String>, tracker: &T)
where
    T: Tracker,
{
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
        match tag.is_empty() {
            true => writeln!(file, "\n#### General").unwrap(),
            false => writeln!(file, "\n#### {}", tag).unwrap(),
        };

        for commit in commits {
            writeln!(file, "* {}", commit.subject).unwrap();
        }
    }
}
