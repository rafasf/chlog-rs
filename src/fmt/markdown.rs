use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Display, Path};

use changelog::Changelog;
use tracker::Tracker;

pub fn create<'a, T>(changelog: &Changelog, tracker: T, output_file: Option<&'a str>) -> Display<'a>
where
    T: Tracker,
{
    let file_path = Path::new(output_file.unwrap_or("CHANGELOG.md"));

    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(e) => panic!(
            "couldn't create file {}: {}",
            file_path.display(),
            e.description()
        ),
    };

    writeln!(file, "## {} ({})", changelog.title, changelog.created_at).unwrap();

    writeln!(file, "\n### {}", "Story Summary").unwrap();
    changelog
        .stories(&T::story_id_pattern())
        .iter()
        .for_each(|story_identifier| {
            let story = tracker.details_of(&story_identifier);

            match story.link {
                Some(link) => writeln!(
                    file,
                    "* [{}]({}) {}",
                    story.id,
                    link,
                    story.name.unwrap()
                ).unwrap(),
                None => writeln!(file, "* {}", story_identifier).unwrap(),
            };
        });

    for (tag, commits) in changelog.commits_by_tag() {
        match tag.is_empty() {
            true => writeln!(file, "\n#### General").unwrap(),
            false => writeln!(file, "\n#### {}", tag).unwrap(),
        };

        commits.iter().for_each(|commit| {
            writeln!(file, "* {}", commit.subject).unwrap();
        });
    }

    file_path.display()
}
