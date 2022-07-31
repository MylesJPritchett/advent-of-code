use std::fs;
use itertools::Itertools;

fn main() {
    let mut houses : Vec<Vec<i32>> = Vec::new();
    let mut number : usize = 1;
    houses.push(vec![0i32, 0i32]);
    
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong when reading the file");

    for c in input.chars(){
        match c {
            '^' => houses.push(vec![houses[number - 1][0], houses[number - 1][1] + 1]), 
            'v' => houses.push(vec![houses[number - 1][0], houses[number - 1][1] - 1]),
            '<' => houses.push(vec![houses[number - 1][0] - 1, houses[number - 1][1]]),
            '>' => houses.push(vec![houses[number - 1][0] + 1, houses[number - 1][1]]),
            _ => println!("Error, couldnt not valid character")
        }
        number += 1;
    }

    for house in &houses {
        println!("Co-ordinates are: ({}, {})", house[0], house[1])
    }
    println!("Number of unique houses: {}", houses.into_iter().unique().count());
}


