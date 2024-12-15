mod part1;

use std::fs;

use part1::parse_regex;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File not found!");

    let result = parse_regex(&data);

    let total: i32 = result.iter().sum();

    println!("total: {total}");
}
