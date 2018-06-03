extern crate core;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use self::reqwest::Client;
use regex::Regex;

use story::Story;
use tracker::Tracker;

pub struct Jira {
    _client: Client,
    pattern: String,
    url: String,
}

impl Jira {
    pub fn new(client: Client, url: String, pattern: String) -> Box<Tracker + 'static> {
        Box::new(Jira {
            _client: client,
            url: url,
            pattern: pattern,
        })
    }
}

impl Tracker for Jira {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn story_id_pattern(&self) -> Regex {
        Regex::new(&self.pattern).unwrap()
    }

    fn details_of(&self, story_identifer: &str) -> Story {
        Story::new(
            story_identifer,
            Some("".to_string()),
            Some(format!("{}/{}", &self.url, story_identifer)))
    }
}
