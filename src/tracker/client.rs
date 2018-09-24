extern crate env_proxy;
extern crate reqwest;

use self::reqwest::header::{Authorization, Basic, Headers};
use self::reqwest::{Client, Proxy};
use std::env;

use show::*;

pub fn http_client_for(url: &str, username_var: &str, password_var: &str) -> Client {
    let mut client = Client::builder();
    if let Some(proxy) = env_proxy::for_url_str(&url).to_string() {
        client.proxy(Proxy::all(&proxy).unwrap());
    }

    if let Some(headers) = credentials(username_var, password_var) {
        client.default_headers(headers);
    }

    client.build().unwrap()
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
