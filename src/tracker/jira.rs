extern crate core;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use self::core::result;
use self::reqwest::Client;
use regex::Regex;
use std::io::{Error, ErrorKind};

use story::Story;
use tracker::Tracker;

#[derive(Deserialize, Debug)]
struct Fields {
    summary: String,
}

#[derive(Deserialize, Debug)]
struct JiraResponse {
    fields: Fields,
}

pub struct Jira {
    client: Client,
    pattern: String,
    url: String,
}

impl Jira {
    pub fn new(client: Client, url: String, pattern: String) -> Box<Tracker + 'static> {
        Box::new(Jira {
            client: client,
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
        let query_url = format!("{}/rest/api/latest/issue/{}", &self.url, story_identifer);

        let response = self.client.get(&query_url).send();

        let story = match response {
            Ok(mut resp) => extract_story_from(resp.json(), &self.url, story_identifer),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        };

        story.unwrap_or(Story::new(
            story_identifer,
            Some("".to_string()),
            Some(format!("{}/browse/{}", &self.url, story_identifer)),
        ))
    }
}

fn extract_story_from(
    body: reqwest::Result<JiraResponse>,
    url: &str,
    story_identifer: &str,
) -> result::Result<Story, Error> {
    match body {
        Ok(result) => Ok(Story::new(
            story_identifer,
            Some(result.fields.summary.to_string()),
            Some(format!("{}/browse/{}", url, story_identifer)),
        )),
        Err(e) => Err(Error::new(ErrorKind::Other, e)),
    }
}
