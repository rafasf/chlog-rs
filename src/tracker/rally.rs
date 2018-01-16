#![allow(non_snake_case)]

// TODO:
//  * Move Story out of here
extern crate core;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;
use std::io::{Error, ErrorKind, Read};
use self::core::result;
use self::serde_json::Value;

#[derive(Debug)]
pub struct Story {
    pub id: String,
    pub name: Option<String>,
    pub link: Option<String>,
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

    let client = http_client();
    let response = client.get(&query_url).send();

    let story = match response {
        Ok(mut r) => story_from(r.json()),
        Err(e) => {
          Err(Error::new(ErrorKind::Other, e))
        }
    };

    story.unwrap_or(Story::only_with(story_number))
}

fn http_client() -> reqwest::Client {
  let mut client = reqwest::Client::builder();

  if let Some(proxy) = proxy_config() {
    client.proxy(proxy);
  }

  if let Some(headers) = credentials() {
    client.default_headers(headers);
  }

  client.build().unwrap()
}

fn proxy_config() -> Option<reqwest::Proxy> {
  env::var("http_proxy")
    .map(|value| reqwest::Proxy::all(&value).unwrap())
    .ok()
}

fn credentials() -> Option<reqwest::header::Headers> {
  let username = env::var("RALLY_USER").ok();
  let password = env::var("RALLY_PWD").ok();

  if (username.is_some() && password.is_some()) {
    let mut headers = reqwest::header::Headers::new();

    headers.set(
      reqwest::header::Authorization(
        reqwest::header::Basic {
          username: username.unwrap(),
          password: password
        })
      );

    Some(headers)
  } else {
    None
  }
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
