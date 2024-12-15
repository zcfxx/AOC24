mod part1;

use std::fs;

use part1::parse_data;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File not found!");

    let result = parse_data(&data);

    let total: isize = result.iter().sum();

    println!("sum: {total}");
}
