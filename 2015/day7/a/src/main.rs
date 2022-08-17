use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use regex::Captures;
use std::collections::HashMap;

fn main() {
    
    let input = File::open("../input.txt").unwrap();
    let reader = BufReader::new(input);

    let re = Regex::new(r"(?:(^[[:lower:]]+|^\d+) |)(?:([[:lower:]]+|[[:upper:]]+) |)(?:([[:lower:]]+|[[:upper:]]+|\d+|) |)-> ([[:lower:]]+)$").unwrap();
    
    let mut signals: HashMap<String, Captures> = HashMap::new();
    //let mut cache = HashMap::new();

    for line in reader.lines() {
        
        let line: String = line.unwrap();
        let cap: Captures = re.captures(line).expect(&format!("invalid input: {}", line));
        signals.insert(cap[4].to_string(), cap);
    }
    
}

// fn get_val(signals: &HashMap<String, Captures>, cache: &mut HashMap<String, u16>, s: &String) -> u16 {
//     if cache.contains_key(s) {
//         return *cache.get(s).unwrap();
//     }
//     let v: Result<u16, _> = s.parse();
//     if v.is_ok(){
//         return v.unwrap();
//     }
//     let r = match signals.get(s).unwrap().get(2).unwrap() {
//         "AND" => println!("G1: {} and G3:{} -> G4:{}", signals.get(s).unwrap().get(1).unwrap().to_string(), signals.get(s).unwrap().get(3).unwrap().to_string(), signals.get(s).unwrap().get(4).unwrap().to_string()),
//         "OR" => println!("G1: {} or G3:{} -> G4:{}", signals.get(s).get(1), signals.get(s).get(3), signals.get(s).get(4)),
//         "LSHIFT" => println!("G1: {} ls G3:{} -> G4:{}", signals.get(s).get(1), signals.get(s).get(3), signals.get(s).get(4)),
//         "RSHIFT" => println!("G1: {} rs G3:{} -> G4:{}", signals.get(s).get(1), signals.get(s).get(3), signals.get(s).get(4)),
//         "NOT" => println!("G1: {} not G3:{} -> G4:{}", signals.get(s).get(1), signals.get(s).get(3), signals.get(s).get(4)),
//         _ => println!("G1: {} -> G4:{}", signals.get(s).get(1), signals.get(s).get(4)),
//     };
//     return 0;
// }