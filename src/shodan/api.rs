use hyper::{body::Buf};
use hyper::{Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIInfo {
    pub query_credits: Option<i64>,
    pub scan_credits: Option<i64>,
    pub telnet: Option<String>,
    pub plan: Option<String>,
    pub unlocked: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostLocation {
	pub city: Option<String>,
	pub region_code: Option<String>,
	pub area_code: Option<String>,
	pub longitude: Option<String>,
	pub country_code3: Option<String>,
	pub country_name: Option<String>,
	pub postal_code: Option<String>,
	pub dma_code: Option<u64>,
	pub country_code: Option<String>,
	pub latitude: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
	pub os: Option<String>,
	pub timestamp: Option<String>,
	pub isp: Option<String>, 
	pub asn: Option<String>, 
	pub hostnames: Option<String>,
	pub location: Option<HostLocation>,
	pub ip: Option<u64>, 
	pub domains: Option<String>, 
	pub org: Option<String>, 
	pub data: Option<String>, 
	pub port: Option<u64>,
	pub ip_string: Option<String>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostSearch {
    pub matches: Option<Host>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Domains {
	pub domain: Option<String>,
	pub tags: Option<String>,
	pub subdomains: Option<String>,
	pub data: Option<DomainData>,
	pub more: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainData {
	pub subdomain: Option<String>,
	pub types: Option<String>,
	pub value: Option<String>,
	pub last_seen: Option<String>,
}

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

    pub async fn api_info(&self) -> anyhow::Result<APIInfo> {
        let u = format!("{}/api-info?key={}", BASE_URI, self.api_key);  
        let uri: hyper::Uri = u.parse::<hyper::Uri>().unwrap();
        let client = Client::new();
        let resp = client.get(uri).await?;
        let body = hyper::body::aggregate(resp).await?;

        let result: APIInfo = serde_json::from_reader(body.reader())?;
        println!("{:?}", result);
        Ok(result)
    }

    pub async fn search_host(&self, host: &str) -> anyhow::Result<HostSearch> {
        let u = format!("{}/shodan/host/{}?key={}", BASE_URI, host, self.api_key);
        let uri:hyper::Uri = u.parse::<hyper::Uri>().unwrap();
        println!("current uri = {}", uri);
        let client = Client::new();

        let resp = client.get(uri).await?;
        let body = hyper::body::aggregate(resp).await?;

        let result: HostSearch = serde_json::from_reader(body.reader())?;
        println!("{:?}", result);
        Ok(result)
    }

    pub async fn search_domain(&self, domain: &str) -> anyhow::Result<Domains> {
        let u = format!("{}/shodan/host/search?key={}&query={}", BASE_URI, self.api_key, domain);
        let uri:hyper::Uri = u.parse::<hyper::Uri>().unwrap();
        println!("current uri = {}", uri);

        let client = Client::new();
        let resp = client.get(uri).await?;
        let body = hyper::body::aggregate(resp).await?;

        let result: Domains = serde_json::from_reader(body.reader())?;

        println!("{:?}", result);
        Ok(result)
    }
}
