#![allow(non_snake_case)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::io::Read;
use self::serde_json::Value;

#[derive(Debug)]
pub struct Story {
    id: String,
    name: Option<String>,
    link: Option<String>,
}

impl Story {
    fn new<T: Into<String>>(id: T, name: Option<String>, link: Option<String>) -> Story {
        Story {
            id: id.into(),
            name: name,
            link: link,
        }
    }

    fn only_with<T: Into<String>>(id: T) -> Story {
        Story::new(id, None, None)
    }
}

#[derive(Deserialize, Debug)]
struct Result {
    Name: String,
    FormattedID: String,
    ObjectID: i64,
}

impl Result {
    fn name(&self) -> &str {
        &self.Name
    }

    fn id(&self) -> &str {
        &self.FormattedID
    }

    fn internal_id(&self) -> &i64 {
        &self.ObjectID
    }
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
}

pub fn name_of(story_number: &str) -> Story {
    let rally1_url = "https://rally1.rallydev.com/slm/webservice/v2.0/hierarchicalrequirement";
    let url = format!(
        "{}?fetch=FormattedID,Name,ObjectID&query=(FormattedID%20%3D%20{})",
        rally1_url, story_number
    );

    println!("using: {:?}", url);

    let client = reqwest::Client::new();
    let response = client.get(&url).basic_auth("-", Some("--")).send();

    match response {
        Ok(mut r) => story_from(r.json(), &story_number),
        Err(e) => Story::only_with(story_number),
    }
}

fn story_from(body: reqwest::Result<QueryResponse>, story_number: &str) -> Story {
    match body {
        Ok(result) => Story::new(
            result.first().id().to_string(),
            Some(result.first().name().to_string()),
            Some(format!(
                "https://rally1.rallydev.com/#/detail/userstory/{}",
                result.first().internal_id()
            )),
        ),
        Err(e) => Story::only_with(story_number),
    }
}
