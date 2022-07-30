use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total : i32 = 0;
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);
    
    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<i32> = line.split("x").map(|s| s.parse().unwrap()).collect();
        total += ribbon(dimensions);
    }
    
    
    println!("Total: {}", total); 
}

fn ribbon(mut dimensions: Vec<i32>) -> i32{
    dimensions.sort();
    let x = dimensions[0];
    let y = dimensions[1];
    let z = dimensions[2];

    (x + x + y + y) + (x * y * z)
}



