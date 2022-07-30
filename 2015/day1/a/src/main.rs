use std::fs;

fn main() {
    let mut floor : i32 = 0;
    let instructions = fs::read_to_string("input.txt")
        .expect("Something went wrong when reading the file");
    for c in instructions.chars(){
        if c == '(' {
            floor+= 1;
        } else if c ==')' {
            floor-= 1;
        }
    }
    println!("Floor is {}.", floor)
}
