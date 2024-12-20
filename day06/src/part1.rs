use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(i32, i32);

impl Default for Point {
    fn default() -> Self {
        Self(-1, -1)
    }
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self(row, col)
    }

    fn add(&self, point: &Point) -> Self {
        Self(self.0 + point.0, self.1 + point.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug)]
struct DirectionError;

impl FromStr for Direction {
    type Err = DirectionError;

    fn from_str(dir: &str) -> Result<Self, Self::Err> {
        match dir {
            "^" => Ok(Direction::UP),
            ">" => Ok(Direction::RIGHT),
            "v" => Ok(Direction::DOWN),
            "<" => Ok(Direction::LEFT),
            _ => return Err(DirectionError),
        }
    }
}

impl Direction {
    fn change_dir(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

pub struct Laboratory {
    grid: Vec<Vec<char>>,
    cells: BTreeMap<Point, char>,
}

impl Default for Laboratory {
    fn default() -> Self {
        Self {
            grid: vec![],
            cells: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum LabError {}

impl FromStr for Laboratory {
    type Err = LabError;

    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let grid = puzzle.lines().map(|line| line.chars().collect()).collect();

        Ok(Self {
            grid,
            ..Default::default()
        })
    }
}

impl Laboratory {
    pub fn init_cells(&mut self) {
        let mut cells: BTreeMap<Point, char> = BTreeMap::new();
        for (index_row, row) in self.grid.iter().enumerate() {
            for (index_col, col) in row.iter().enumerate() {
                cells.insert(Point(index_row as i32, index_col as i32), *col);
            }
        }
        self.cells = cells;
    }

    fn is_obstruction(&self, point: &Point) -> bool {
        for (index_row, row) in self.grid.iter().enumerate() {
            for (index_col, col) in row.iter().enumerate() {
                if index_row as i32 == point.0 && index_col as i32 == point.1 && *col == '#' {
                    return true;
                }
            }
        }
        false
    }

    fn is_exit(&self, point: &Point) -> bool {
        if point.0 < 0
            || point.0 >= self.grid.len() as i32
            || point.1 < 0
            || point.1 >= self.grid[0].len() as i32
        {
            return true;
        }
        false
    }
}

#[derive(Debug)]
pub struct Guard {
    position: Point,
    direction: Direction,
    steps: HashSet<Point>,
}

impl Default for Guard {
    fn default() -> Self {
        Self {
            position: Point(-1, -1),
            direction: Direction::UP,
            steps: HashSet::new(),
        }
    }
}

impl Guard {
    pub fn get_step_count(&mut self) -> usize {
        self.steps.len()
    }

    pub fn starting_position(&mut self, lab: &Laboratory) {
        for (index_row, row) in lab.grid.iter().enumerate() {
            for (index_col, col) in row.iter().enumerate() {
                if *col != '#' && *col != '.' {
                    self.position = Point(index_row as i32, index_col as i32);
                    let dir_str = col.to_string();
                    println!("starting char: {:?}", col);
                    self.direction = Direction::from_str(&dir_str).unwrap();
                    break;
                }
            }
        }
    }

    pub fn move_to_direction(&mut self, laboratory: &Laboratory) -> bool {
        let mut dir = self.direction;

        let point_diff = match dir {
            Direction::UP => Point(-1, 0),
            Direction::RIGHT => Point(0, 1),
            Direction::DOWN => Point(1, 0),
            Direction::LEFT => Point(0, -1),
        };

        let mut check_point = self.position.add(&point_diff);

        if laboratory.is_exit(&check_point) {
            return false;
        }

        if laboratory.is_obstruction(&check_point) {
            dir = dir.change_dir();
            check_point = self.position;
        } else {
            self.steps.insert(check_point);
        }

        self.position = check_point;
        self.direction = dir;

        true
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, str::FromStr};

    use super::*;

    #[test]
    fn test1() {
        let file_str = fs::read_to_string("example.txt").expect("File not found!");

        let mut lab = Laboratory::from_str(&file_str).unwrap();

        // println!("grid: {:?}", lab.grid);
        lab.init_cells();

        // println!("cells: {:?}", lab.cells);
    }

    #[test]
    fn test2() {
        let file_str = fs::read_to_string("example.txt").expect("File not found!");

        let mut lab = Laboratory::from_str(&file_str).unwrap();
        lab.init_cells();

        let mut guard = Guard::default();
        guard.starting_position(&lab);

        println!("starting guard: {:?}", guard);
        assert_eq!(guard.position, Point(6, 4));
        assert_eq!(guard.direction, Direction::UP);

        while guard.move_to_direction(&lab) {
            // println!("moving guard: {:?}", guard);
        }
        println!("ending guard: {:?}", guard.steps.len());
    }
}
