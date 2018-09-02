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

pub fn tracker_for(
    tracker_name: &str,
    tracker_url: &str,
    story_pattern: String,
) -> Box<Tracker + 'static> {
    if tracker_name.to_lowercase() == "jira" {
        jira::Jira::new(
            client::http_client_no_proxy("TRACKER_USER", "TRACKER_PWD"),
            tracker_url.to_string(),
            story_pattern,
        )
    } else {
        rally::Rally::new(
            client::http_client("TRACKER_USER", "TRACKER_PWD"),
            tracker_url.to_string(),
            story_pattern,
        )
    }
}
