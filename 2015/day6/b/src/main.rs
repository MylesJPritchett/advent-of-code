use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total = 0;
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lights = vec![vec![0; 1000] ; 1000];

    for line in reader.lines() {
        let line = line.unwrap();
        let chunks: Vec<&str> = line.rsplitn(6, |c| c == ' ' || c == ',').collect();
        execute_command(chunks[4].parse::<usize>().unwrap(), chunks[3].parse::<usize>().unwrap(), chunks[1].parse::<usize>().unwrap(), chunks[0].parse::<usize>().unwrap(), chunks[5], &mut lights);
    }
    //execute_command(0, 0, 0, 0, "turn on", &mut lights);
    for row in lights.iter(){
        for col in row.iter(){
            total += col;
        }
    }
    println!("{}", total);
}

fn execute_command(x1: usize, y1: usize, x2: usize, y2: usize, command: &str, lights: &mut Vec<Vec<i32>>){
    for x in x1..=x2 {
        for y in y1..=y2 {
            match command {
                "turn on" => lights[x][y] += 1,
                "turn off" => if lights[x][y] > 0 {lights[x][y] -= 1},
                "toggle" => lights[x][y] += 2,
                _ => lights[x][y] = lights[x][y],
            }
        }
    }
}


