extern crate core;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::io::{Error, ErrorKind};
use regex::Regex;
use self::core::result;

use story::Story;
use tracker::Tracker;
use tracker::client::*;

const URL: &str = "https://rally1.rallydev.com/slm/webservice/v2.0/hierarchicalrequirement";

#[derive(Deserialize, Debug)]
struct Result {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "FormattedID")]
    formatted_id: String,
    #[serde(rename = "ObjectID")]
    object_id: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct QueryResult {
    total_result_count: i64,
    results: Vec<Result>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct QueryResponse {
    query_result: QueryResult,
}

impl QueryResponse {
    fn first(&self) -> &Result {
        &self.query_result.results[0]
    }

    fn name(&self) -> &str {
        &self.first().name
    }

    fn id(&self) -> &str {
        &self.first().formatted_id
    }

    fn internal_id(&self) -> &i64 {
        &self.first().object_id
    }

    fn has_results(&self) -> bool {
        self.query_result.total_result_count > 0
    }
}

pub struct Rally;

impl Tracker for Rally {
    fn story_id_pattern() -> Regex {
        Regex::new(r"^(US\w+)\s*").unwrap()
    }

    fn details_of(&self, story_identifer: &str) -> Story {
        let query_url = format!(
            "{}?fetch=FormattedID,Name,ObjectID&query=(FormattedID%20%3D%20{})",
            URL, story_identifer
        );

        let client = http_client("RALLY_USER", "RALLY_PWD");
        let response = client.get(&query_url).send();

        let story = match response {
            Ok(mut resp) => extract_story_from(resp.json()),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        };

        story.unwrap_or(Story::only_with(story_identifer))
    }
}

fn extract_story_from(body: reqwest::Result<QueryResponse>) -> result::Result<Story, Error> {
    match body {
        Ok(result) => {
            if result.has_results() {
                Ok(create_story_from(&result))
            } else {
                Err(Error::new(ErrorKind::Other, "no stories were found"))
            }
        }
        Err(e) => Err(Error::new(ErrorKind::Other, e)),
    }
}

fn create_story_from(response: &QueryResponse) -> Story {
    Story::new(
        response.id(),
        Some(response.name().to_string()),
        Some(format!(
            "https://rally1.rallydev.com/#/detail/userstory/{}",
            response.internal_id()
        )),
    )
}
