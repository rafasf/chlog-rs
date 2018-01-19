#![allow(non_snake_case)]

extern crate core;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::io::{Error, ErrorKind, Read};
use self::core::result;
use self::serde_json::Value;
use story::Story;
use tracker::client::*;

const URL: &str = "https://rally1.rallydev.com/slm/webservice/v2.0/hierarchicalrequirement";

#[derive(Deserialize, Debug)]
struct Result {
    Name: String,
    FormattedID: String,
    ObjectID: i64,
}

#[derive(Deserialize, Debug)]
struct QueryResult {
    TotalResultCount: i64,
    Results: Vec<Result>,
}

#[derive(Deserialize, Debug)]
struct QueryResponse {
    QueryResult: QueryResult,
}

impl QueryResponse {
    fn first(&self) -> &Result {
        &self.QueryResult.Results[0]
    }

    fn name(&self) -> &str {
        &self.first().Name
    }

    fn id(&self) -> &str {
        &self.first().FormattedID
    }

    fn internal_id(&self) -> &i64 {
        &self.first().ObjectID
    }

    fn has_results(&self) -> bool {
        self.QueryResult.TotalResultCount > 0
    }
}

pub fn details_of(story_identifer: &str) -> Story {
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

fn extract_story_from(body: reqwest::Result<QueryResponse>) -> result::Result<Story, Error> {
    match body {
        Ok(result) => {
            if (result.has_results()) {
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
