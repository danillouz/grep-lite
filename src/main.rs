extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for _line in reader.lines() {
        let line = _line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

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
            Arg::with_name("input")
                .help("The file, or text from STDIN to search through.")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        read_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        read_lines(reader, re);
    }
}
