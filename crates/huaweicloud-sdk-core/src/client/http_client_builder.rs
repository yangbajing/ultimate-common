use crate::{client::ClientConfig, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue, IntoHeaderName},
    Method, RequestBuilder, Url,
};

use super::client_config::sign_headers;

#[derive(Debug)]
pub struct HttpClientBuilder {
    pub method: Method,
    pub url: Url,
    pub headers: HeaderMap,
    pub body: String,
}

impl HttpClientBuilder {
    pub fn new(endpoint: &str) -> Result<Self> {
        let url = Url::parse(endpoint)?;
        let b = Self { method: Method::GET, url, headers: HeaderMap::new(), body: "".into() };
        Ok(b)
    }

    pub fn build(mut self, client_config: &ClientConfig) -> Result<RequestBuilder> {
        sign_headers(client_config, &mut self)?;

        let b = reqwest::Client::new().request(self.method, self.url).headers(self.headers).body(self.body);
        Ok(b)
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }

    pub fn resource_path(mut self, path: &str) -> Self {
        self.url.set_path(path);
        self
    }

    pub fn endpoint(mut self, endpoint: &str) -> Result<Self> {
        self.url.set_host(Some(endpoint))?;
        Ok(self)
    }

    pub fn query(mut self, query: &str) -> Self {
        self.url.set_query(Some(query));
        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub fn header<K>(&mut self, key: K, value: HeaderValue) -> Option<HeaderValue>
    where
        K: IntoHeaderName,
    {
        self.headers.insert(key, value)
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = body;
        println!("set body is {}", self.body);
        self
    }

    pub fn json(self, json: serde_json::Value) -> Self {
        self.body(json.to_string())
    }
}
