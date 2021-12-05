mod lib;
mod shodan;
mod utils;

use lib::options::Options;

fn main() {
    Options::parse();
}
