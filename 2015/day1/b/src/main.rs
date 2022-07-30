use std::env;
use std::fs;
use std::process;

fn main() {
    let mut floor : i32 = 0;
    let instructions = fs::read_to_string("input.txt")
        .expect("Something went wrong when reading the file");
    for (i, c) in instructions.chars().enumerate(){
        if c == '(' {
            floor+= 1;
        } else if c ==')' {
            floor-= 1;
        }
        if floor == -1 {
            println!("Goes negative at {}.", i + 1);
            process::exit(0);
        }
    }
}
