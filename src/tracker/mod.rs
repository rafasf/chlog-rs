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
) -> Option<Box<Tracker + 'static>> {
    match tracker_name {
        "jira" => Some(jira::Jira::new(
            client::http_client_no_proxy("TRACKER_USER", "TRACKER_PWD"),
            tracker_url.to_string(),
            story_pattern,
        )),
        "rally" => Some(rally::Rally::new(
            client::http_client("TRACKER_USER", "TRACKER_PWD"),
            tracker_url.to_string(),
            story_pattern,
        )),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use tracker::*;

    fn creates_jira() {
        let tracker = tracker_for("jira", "http://jira.com", "Prefix-1".into()).unwrap();
        assert_eq!("Prefix-1".to_string(), tracker.pattern())
    }

    fn creates_rally() {
        let tracker = tracker_for("rally", "http://rally.com", "Prefix-1".into()).unwrap();
        assert_eq!("Prefix-1".to_string(), tracker.pattern())
    }

    fn returns_none_when_not_supported() {
        let tracker = tracker_for("shiny", "http://shiny.com", "Prefix-1".into());
        assert!(tracker.is_none())
    }
}
