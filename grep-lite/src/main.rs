extern crate regex;
extern crate clap;

use regex::Regex;
use clap::{App,Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = App::new("grep-lite")
        .version("0.2")
        .about("search for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("pattern").unwrap();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) { // line es un String entonces toma como argumento &str
            Some(_) => println!("{}", line),
            None => (), //() es un placeholder de null
        }
    }
}
