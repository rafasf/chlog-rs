extern crate reqwest;

use self::reqwest::header::{Authorization, Basic, Headers};
use self::reqwest::{Client, Proxy};
use std::env;

use show::*;

pub fn http_client(username_var: &str, password_var: &str) -> Client {
    let mut client = Client::builder();

    if let Some(proxy) = proxy_config() {
        show(format!("Using proxy: {:?}", proxy));
        client.proxy(proxy);
    }

    if let Some(headers) = credentials(username_var, password_var) {
        client.default_headers(headers);
    }

    client.build().unwrap()
}

pub fn http_client_no_proxy(username_var: &str, password_var: &str) -> Client {
    let mut client = Client::builder();

    if let Some(headers) = credentials(username_var, password_var) {
        client.default_headers(headers);
    }

    client.build().unwrap()
}
fn proxy_config() -> Option<Proxy> {
    env::var("http_proxy")
        .map(|value| Proxy::all(&value).unwrap())
        .ok()
}

fn credentials(username_var: &str, password_var: &str) -> Option<Headers> {
    let username = env::var(username_var).ok();
    let password = env::var(password_var).ok();

    if username.is_some() && password.is_some() {
        let mut headers = Headers::new();
        let user = username.unwrap();

        show(format!("Using {} to fetch information from tracker", user));

        headers.set(Authorization(Basic {
            username: user,
            password: password,
        }));

        Some(headers)
    } else {
        None
    }
}
