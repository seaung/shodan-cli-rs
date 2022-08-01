use hyper::{body::Buf};
use hyper::{Body, Method, Client, Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIInfo {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {}

pub struct ShodanClient {
    api_key: &'static str,
}

const BASE_URI: &'static str = "https://api.shodan.io";

impl ShodanClient {
    pub fn new(api_key: &'static str) -> ShodanClient {
        ShodanClient {
            api_key: api_key,
        }
    }

    pub async fn api_info(&self) {
        let uri: hyper::Uri = format!("{}/api-info?key={}", BASE_URI, self.api_key);
        let client = Client::new();
        let resp = client.get(uri).await?;
        let body = hyper::body::aggregate(resp).await?;

        let result = serde_json::from_reader(body.reader())?;
        println!("{}", result);
    }

    pub async fn search_host(&self, host: &str) {
        let uri:hyper::Uri = format!("{}/shodan/host/{}?key={}", BASE_URI, host, self.api_key);
        println!("current uri = {}", uri);
        let client = Client::new();

        let resp = client.get(uri).await?;
        let body = hyper::body::aggregate(resp).await?;

        let result = serde_json::from_reader(body.reader())?;
        println!("{}", result);
    }

    pub async fn search_domain(&self, domain: &str) {
        let uri:hyper::Uri = format!("{}/shodan/host/search?key={}&query={}", BASE_URI, self.api_key, domain);
        println!("current uri = {}", uri);

        let client = Client::new();
        let resp = client.get(uri).await?;
        let body = hyper::body::aggregate(resp).await?;

        let result = serde_json::from_reader(body.reader())?;

        println!("{:?}", result);
    }
}
