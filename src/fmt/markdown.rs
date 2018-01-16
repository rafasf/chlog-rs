extern crate regex;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Display, Path};
use regex::Regex;
use changelog::Changelog;
use tracker::rally;

pub fn create<'a>(changelog: &Changelog, story_re: &Regex) -> Display<'a> {
    let file_path = Path::new("CHANGELOG.md");

    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(e) => panic!(
            "couldn't create file {}: {}",
            file_path.display(),
            e.description()
        ),
    };

    writeln!(file, "## {} ({})", changelog.title, changelog.created_at);

    writeln!(file, "\n### {}", "Story Summary");
    changelog.stories(story_re).iter().for_each(|story| {
        let full_story = rally::name_of(&story);
        match full_story.link {
            Some(link) => writeln!(
                file,
                "* [{}]({}) {}",
                full_story.id,
                link,
                full_story.name.unwrap()
            ),
            None => writeln!(file, "* {}", story),
        };
    });

    for (tag, commits) in changelog.commits_by_tag() {
        match tag.is_empty() {
            true => writeln!(file, "\n#### General"),
            false => writeln!(file, "\n#### {}", tag),
        };

        commits.iter().for_each(|commit| {
            writeln!(file, "* {}", commit.subject);
        });
    }

    file_path.display()
}
