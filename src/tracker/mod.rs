extern crate regex;

pub mod client;
pub mod jira;
pub mod rally;

use regex::Regex;
use story::Story;

pub trait Tracker {
    fn pattern(&self) -> &str;
    fn story_id_pattern(&self) -> Regex;
    fn details_of(&self, story_identifier: &str) -> Story;
}
