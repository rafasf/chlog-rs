extern crate regex;

pub mod client;
pub mod rally;

use regex::Regex;
use story::Story;

pub trait Tracker {
    fn story_id_pattern() -> Regex;
    fn details_of(&self, story_identifier: &str) -> Story;
}
