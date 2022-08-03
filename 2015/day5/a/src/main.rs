use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total : i32 = 0;
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);
    
    for line in reader.lines() {
        let line = line.unwrap();
        if parser(line) {
            total += 1;
        }
    }
    println!("{}", total);
}

fn parser(line: String) -> bool {
    if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy"){
        return false;
    }
    let mut previous: char = '\0';
    let mut flag = false;
    let mut vowel_count = 0;
    for c in line.chars(){
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowel_count += 1;
        }
        if c == previous {
            flag = true;
        }
        previous = c;
    }
    if vowel_count < 3 {
        return false;
    }
    return flag;
}
