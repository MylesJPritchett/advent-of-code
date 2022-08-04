use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total : i32 = 0;
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lights = [[false; 1000] ; 1000];

    for line in reader.lines() {
        let line = line.unwrap();
        let chunks: Vec<&str> = line.rsplitn(6, |c| c == ' ' || c == ',').collect();
        execute_command(chunks[4].parse::<usize>().unwrap(), chunks[3].parse::<usize>().unwrap(), chunks[1].parse::<usize>().unwrap(), chunks[0].parse::<usize>().unwrap(), chunks[5], &mut lights);
    }
    for row in lights.iter() {
        for col in row.iter() {
            if *col {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

fn execute_command(x1: usize, y1: usize, x2: usize, y2: usize, command: &str, lights: &mut[[bool; 1000]; 1000]){
    for x in x1..=x2 {
        for y in y1..=y2 {
            match command {
                "turn on" => lights[x][y] = true,
                "turn off" => lights[x][y] = false,
                "toggle" => lights[x][y] = !lights[x][y],
                _ => println!("Not Valid Command"),
            }
        }
    }
}

