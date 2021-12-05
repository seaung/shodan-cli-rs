use ansi_term::Color::{ Red, Blue, Yellow, Green };

pub struct Commons;

impl Commons {
    pub fn error(msg: &str) {
        println!("{} - {}", Red.bold().paint("[-]"), msg);
    }

    pub fn info(msg: &str) {
        println!("{} - {}", Blue.bold().paint("[*]"), msg);
    }

    pub fn warnning(msg: &str) {
        println!("{} - {}", Yellow.bold().paint("[!]"), msg);
    }

    pub fn success(msg: &str) {
        println!("{} - {}", Green.bold().paint("[+]"), msg);
    }
}
