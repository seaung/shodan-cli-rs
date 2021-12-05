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

    pub fn api_info(&self) {
        println!("shodan api info")
    }

    pub fn search_host(&self, host: &str) {
        println!("{}", host);
    }

    pub fn search_domain(&self, domain: &str) {
        println!("{}", domain);
    }
}
