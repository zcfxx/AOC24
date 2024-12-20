use std::str::FromStr;

use part1::{Guard, Laboratory};

mod part1;

fn main() {
    let puzzle = include_str!("../input.txt");

    let mut lab = Laboratory::from_str(puzzle).unwrap();
    lab.init_cells();

    let mut guard = Guard::default();

    guard.starting_position(&lab);

    while guard.move_to_direction(&lab) {}
    println!("ending guard: {:?}", guard.get_step_count());
}
