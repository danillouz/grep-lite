extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Search text for a pattern.
/// Based on this grep specification: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html
fn main() {
    let args = App::new("grep-lite")
        .version("0.1.0")
        .about("A lightweight version of the grep utility.")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .help("The file to search through.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let file = args.value_of("file").unwrap();
    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);

    for _line in reader.lines() {
        let line = _line.unwrap();
        match re.find(&line) {
            Some(_) => {
                println!("{}", line);
            }
            None => (),
        }
    }
}
