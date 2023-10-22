use regex::Regex;
use clap::{App, Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;// for implement BufReader lines()

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searched for patterns")
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

    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();
        let contains_substring = re.find(&line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
