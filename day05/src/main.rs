mod part1;

use std::{fs, str::FromStr};

use part1::Paging;

fn main() {
    let file_str = fs::read_to_string("../input.txt").expect("File not found!");

    let paging = Paging::from_str(&file_str).unwrap();

    let sum = paging.validate_pages();

    println!("sum: {:?}", sum);
}
