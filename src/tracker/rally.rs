#![allow(non_snake_case)]

// TODO:
//  * Support use of system proxy
//  * Move Story out of here
//  * Accept credentials as arguments or environment variables
extern crate core;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::io::{Error, ErrorKind, Read};
use self::core::result;
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
        Story::new(id.into(), None, None)
    }
}

const URL: &str = "https://rally1.rallydev.com/slm/webservice/v2.0/hierarchicalrequirement";

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

    fn has_results(&self) -> bool {
      self.QueryResult.TotalResultCount > 0
    }
}

pub fn name_of(story_number: &str) -> Story {
    let query_url = format!(
      "{}?fetch=FormattedID,Name,ObjectID&query=(FormattedID%20%3D%20{})",
      URL,
      story_number);

    println!("using: {:?}", query_url);

    let client = reqwest::Client::new();
    let response = client.get(&query_url)
      .basic_auth("-", Some("--"))
      .send();

    let story = match response {
        Ok(mut r) => story_from(r.json()),
        Err(e) => {
          Err(Error::new(ErrorKind::Other, e))
        }
    };

    story.unwrap_or(Story::only_with(story_number))
}

fn story_from(body: reqwest::Result<QueryResponse>) -> result::Result<Story, Error> {
    match body {
        Ok(result) => {
          if (result.has_results()) {
            Ok(Story::new(
              result.first().id(),
              Some(result.first().name().to_string()),
              Some(format!("https://rally1.rallydev.com/#/detail/userstory/{}", result.first().internal_id()))))
          } else {
            Err(Error::new(ErrorKind::Other, "no stories were found"))
          }
        },
        Err(e) => {
          Err(Error::new(ErrorKind::Other, e))
        }
    }
}
