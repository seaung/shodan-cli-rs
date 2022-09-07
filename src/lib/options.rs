use clap::{ App, Arg, SubCommand };
use crate::shodan::api::ShodanClient;
use crate::utils::prints::Commons;

pub struct Options;

impl Options {
    pub fn parse() {
        let matches = App::new("a shodan cli for rust.")
            .version("#dev 0.0.0")
            .author("seaung")
            .about("shodan cli tools")
            .subcommand(SubCommand::with_name("info").help("print shodan api help information.")
            )
            .subcommand(SubCommand::with_name("search")
                .arg(Arg::with_name("host"))
                .help("enter a target host address.")
            )
            .subcommand(SubCommand::with_name("domain")
                .arg(Arg::with_name("name"))
                .help("enter a target domain.")
            ).get_matches();

        let shodan_client = ShodanClient::new("shoan-api");
        match matches.subcommand() {
            ("search", Some(search_match)) => {
                let host = match search_match.value_of("host") {
                    Some(query) => query,
                    None => {
                        Commons::error("not found search params");
                        std::process::exit(1);
                    }
                };
                shodan_client.search_host(host);
            },
            ("domain", Some(domain_match)) => {
                let domain = match domain_match.value_of("domain") {
                    Some(query) => query,
                    None => {
                        Commons::error("not found domain");
                        std::process::exit(1);
                    }
                };

                let result = shodan_client.search_domain(domain);
            },
            ("info", Some(_)) => {
                let result = shodan_client.api_info();
            },
            _ => {}
        }
    }
}
