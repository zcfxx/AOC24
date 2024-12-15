use std::{char, str::FromStr};

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn add(&self, point: &Point) -> Self {
        Self(self.0 + point.0, self.1 + point.1)
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Debug)]
pub struct Puzzle {
    grid: Vec<Vec<char>>,
}

#[derive(Debug)]
pub enum PuzzleGridError {}

impl FromStr for Puzzle {
    type Err = PuzzleGridError;

    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let result = puzzle.lines().map(|line| line.chars().collect()).collect();
        Ok(Self { grid: result })
    }
}

impl Puzzle {
    fn find_xmas(&self, x_point: &Point, dir: &Direction) -> bool {
        let point_diff = match dir {
            Direction::Up => Point(-1, 0),
            Direction::UpRight => Point(-1, 1),
            Direction::Right => Point(0, 1),
            Direction::DownRight => Point(1, 1),
            Direction::Down => Point(1, 0),
            Direction::DownLeft => Point(1, -1),
            Direction::Left => Point(0, -1),
            Direction::UpLeft => Point(-1, -1),
        };

        let check = |p: &Point, c: char| {
            if p.0 < 0
                || p.0 >= self.grid.len() as i32
                || p.1 < 0
                || p.1 >= self.grid[0].len() as i32
            {
                return false;
            }

            self.grid[p.0 as usize][p.1 as usize] == c
        };

        let mut check_point = x_point.add(&point_diff);
        if !check(&check_point, 'M') {
            return false;
        }

        check_point = check_point.add(&point_diff);
        if !check(&check_point, 'A') {
            return false;
        }

        check_point = check_point.add(&point_diff);
        if !check(&check_point, 'S') {
            return false;
        } else {
            return true;
        }
    }

    fn count_xmas(&self, x_point: &Point) -> usize {
        let dirs: [Direction; 8] = [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ];

        dirs.iter()
            .filter(|dir| self.find_xmas(x_point, dir))
            .count()
    }

    pub fn search_for_x(&self) -> usize {
        let mut x_locs: Vec<Point> = vec![];
        let mut counter = 0 as usize;

        for (i, row) in self.grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 'X' {
                    let point = Point(i as i32, j as i32);
                    x_locs.push(point);
                }
            }
        }

        x_locs
            .iter()
            .for_each(|point| counter += self.count_xmas(point));

        counter
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Puzzle;

    const TEST_DATA: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test1() {
        let puzzle = Puzzle::from_str(TEST_DATA);

        println!("{:?}", puzzle);

        let count = puzzle.unwrap().search_for_x();

        println!("count: {:?}", count);
        assert_eq!(count, 18);
    }
}
