use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use regex::Captures;
use std::collection::HashMap;

fn main() {
    
    let input = File::open("../input.txt").unwrap();
    let reader = BufReader::new(input);

    let re = Regex::new(r"(?:(^[[:lower:]]+|^\d+) |)(?:([[:lower:]]+|[[:upper:]]+) |)(?:([[:lower:]]+|[[:upper:]]+|\d+|) |)-> ([[:lower:]]+)$").unwrap();
    
    let mut signals = HashMap::new();
    let mut chache = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let cap = re.captures(&line).expect(&format!("invalid input: {}", line));
        signals.insert(cap.get(4).to_string(), cap);
    }
    
}

fn get_val(signals: &HashMap<String, Captures>, cache: &mut HashMap<String, u16>, s: &String) -> u16 {
    if cahce.contains_key(s) {
        return *cache.get(s).unwrap();
    }
    let v: Result<u16, _> = s.parse();
    if v.is_ok(){
        return v.unwrap();
    }
    match cap.get(2).unwrap().as_str() {
        "AND" => 
        "OR" =>
        "LSHIFT" =>
        "RSHIFT" =>
        "NOT" =>
        _ => get_val(signals, cache, s)
    }
}