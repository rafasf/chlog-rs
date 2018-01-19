extern crate reqwest;

use std::env;
use self::reqwest::{Client, Proxy};
use self::reqwest::header::{Authorization, Basic, Headers};

pub fn http_client(username_var: &str, password_var: &str) -> Client {
    let mut client = Client::builder();

    if let Some(proxy) = proxy_config() {
        client.proxy(proxy);
    }

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

    if (username.is_some() && password.is_some()) {
        let mut headers = Headers::new();

        headers.set(Authorization(Basic {
            username: username.unwrap(),
            password: password,
        }));

        Some(headers)
    } else {
        None
    }
}

