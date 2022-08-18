use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use regex::Captures;
use std::collections::HashMap;

fn main() {
    //read file
    let input = File::open("../input.txt").unwrap();
    let reader = BufReader::new(input);

    //ugly regex, but it does exactly what I want it to
    let re = Regex::new(r"(?:(^[[:lower:]]+|^\d+) |)(?:([[:lower:]]+|[[:upper:]]+) |)(?:([[:lower:]]+|[[:upper:]]+|\d+|) |)-> ([[:lower:]]+)$").unwrap();
    
    //hashmap to store the 'instructions' which is the regex groups, with the key of group 4
    let mut instructions : HashMap<&str, Vec<&str>> = HashMap::new();

    for line in reader.lines().into_iter() {

        let line = line.unwrap();
        println!("Line Read: {}", line);

        for caps in re.captures_iter(line){ //this line is where I am having touble, with or without the '&' before line, I get a different error

            let groups: Vec<&str> = caps.iter()
                .map(|m| match m {
                    Some(value) => value.as_str(),
                    None => ""
                })
            .collect();

            println!("{:?}", groups); //this prints exactly what I want

            instructions.insert(groups[4], groups); //this line is the one that causes the error, because of a reference outside the loop?
        }
    }
}

//let mut signals: HashMap<String, i32> = HashMap::new();

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