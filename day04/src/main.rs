mod part1;

use part1::Puzzle;
use std::{self, fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let puzzle = Puzzle::from_str(&input);

    let count = puzzle.unwrap().search_for_x();

    println!("count: {}", count);
}
