use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total : i32 = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<i32> = line.split("x").map(|s| s.parse().unwrap()).collect();
        total += surface(dimensions[0], dimensions[1], dimensions[2]);
    }
    
    
    println!("Total: {}", total); 
}

fn surface(l : i32, w : i32, h :i32) -> i32{
    let s1 : i32 = l * w;
    let s2 : i32 = w * h;
    let s3 : i32 = h * l;
    (s1.min(s2).min(s3)) + (2 * s1) + (2 * s2) + (2 * s3)
}
