use std::fs;
use itertools::Itertools;

fn main() {
    let mut houses : Vec<Vec<i32>> = Vec::new();
    houses.push(vec![0i32, 0i32]);
    
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong when reading the file");

    for (i, c) in input.chars().enumerate(){
        match c {
            '^' => houses.push(vec![houses[i][0], houses[i][1] + 1]), 
            'v' => houses.push(vec![houses[i][0], houses[i][1] - 1]),
            '<' => houses.push(vec![houses[i][0] - 1, houses[i][1]]),
            '>' => houses.push(vec![houses[i][0] + 1, houses[i][1]]),
            _ => println!("Error, not valid character")
        }

    }

    for house in &houses {
        println!("Co-ordinates are: ({}, {})", house[0], house[1])
    }
    println!("Number of unique houses: {}", houses.into_iter().unique().count());
}


