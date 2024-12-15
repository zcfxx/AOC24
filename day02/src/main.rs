use std::fs;

use part1::{count_safe, count_safe_v2};

mod part1;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File not found!");

    let result = count_safe_v2(&data);

    println!("result: {result}");
}
