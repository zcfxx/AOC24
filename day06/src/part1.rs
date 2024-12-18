use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn add(&self, point: &Point) -> Self {
        Self(self.0 + point.0, self.1 + point.1)
    }
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct Lab {
    grid: Vec<Vec<char>>,
}

#[derive(Debug)]
pub enum LabError {}

impl FromStr for Lab {
    type Err = LabError;

    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let grid = puzzle.lines().map(|line| line.chars().collect()).collect();

        Ok(Self { grid })
    }
}

impl Lab {
    fn find_starting(&self) -> Point {
        let mut starting_point = Point(-1, -1);
        for (i, row) in self.grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col != '#' && *col != '.' {
                    starting_point = Point(i as i32, j as i32);
                }
            }
        }
        starting_point
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, str::FromStr};

    use super::Lab;

    #[test]
    fn test1() {
        let file_str = fs::read_to_string("example.txt").expect("File not found!");

        let lab = Lab::from_str(&file_str).unwrap();

        println!("grid: {:?}", lab.grid);

        let starting_point = lab.find_starting();

        println!("starting point: {:?}", starting_point);
    }
}
