use std::fs::File;
use std::io::{BufRead, BufReader};
use fancy_regex::Regex;
use std::env;

fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let mut total : i32 = 0;
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);
    let rule1 = Regex::new(r"(..).*\1").unwrap();
    let rule2 = Regex::new(r"(.).\1").unwrap();
    
    for line in reader.lines() {
        let line = line.unwrap();
        if rule1.is_match(&line).unwrap() && rule2.is_match(&line).unwrap() {
            total += 1;
        }
    }
    println!("{}", total);
}