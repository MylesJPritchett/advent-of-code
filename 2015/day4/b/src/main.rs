use crypto::md5::Md5;
use crypto::digest::Digest;

use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong when reading the file");

    let mut hasher = Md5::new();
    let key = input.as_bytes();
    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        let mut output = [0; 16];

        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2]) as i32;
        if first_five == 0 {
            println!("{}", i);
            break;
        }
        hasher.reset();
    }
}
