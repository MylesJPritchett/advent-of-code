use std::fs;
use itertools::Itertools;

fn main() {
    let mut santa_houses : Vec<Vec<i32>> = Vec::new();
    let mut robo_houses : Vec<Vec<i32>> = Vec::new();
    santa_houses.push(vec![0i32, 0i32]);
    robo_houses.push(vec![0i32, 0i32]);
    
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong when reading the file");

    for (i, c) in input.chars().enumerate(){
        if i % 2 == 0 {
            match c {
                '^' => santa_houses.push(vec![santa_houses[santa_houses.len() - 1][0], santa_houses[santa_houses.len() - 1][1] + 1]), 
                'v' => santa_houses.push(vec![santa_houses[santa_houses.len() - 1][0], santa_houses[santa_houses.len() - 1][1] - 1]),
                '<' => santa_houses.push(vec![santa_houses[santa_houses.len() - 1][0] - 1, santa_houses[santa_houses.len() - 1][1]]),
                '>' => santa_houses.push(vec![santa_houses[santa_houses.len() - 1][0] + 1, santa_houses[santa_houses.len() - 1][1]]),
                _ => println!("Error, not valid character")
            }
        } else {
            match c {
                '^' => robo_houses.push(vec![robo_houses[robo_houses.len() - 1][0], robo_houses[robo_houses.len() - 1][1] + 1]), 
                'v' => robo_houses.push(vec![robo_houses[robo_houses.len() - 1][0], robo_houses[robo_houses.len() - 1][1] - 1]),
                '<' => robo_houses.push(vec![robo_houses[robo_houses.len() - 1][0] - 1, robo_houses[robo_houses.len() - 1][1]]),
                '>' => robo_houses.push(vec![robo_houses[robo_houses.len() - 1][0] + 1, robo_houses[robo_houses.len() - 1][1]]),
                _ => println!("Error, not valid character")
            }
        }
    }
    santa_houses.append(&mut robo_houses);

    println!("Number of unique houses: {}", santa_houses.into_iter().unique().count());
}


