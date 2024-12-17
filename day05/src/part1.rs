use std::{collections::HashMap, fs, str::FromStr};

pub struct Paging {
    rules: HashMap<i32, Vec<i32>>,
    pages: Vec<Vec<i32>>,
}

#[derive(Debug)]
pub enum PagingError {}

impl FromStr for Paging {
    type Err = PagingError;

    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        puzzle
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let values = line
                    .split("|")
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<_>>();

                [values[0], values[1]]
            })
            // .collect::<HashMap<i32, Vec<i32>>>();
            .for_each(|arr| {
                if !rules.contains_key(&arr[0]) {
                    rules.insert(arr[1], vec![arr[0]]);
                } else {
                    rules.entry(arr[1]).or_insert(vec![]).push(arr[0]);
                }
            });

        let pages = puzzle
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(|line| {
                let values = line
                    .split(",")
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<i32>>();
                values
            })
            .collect::<Vec<_>>();

        Ok(Self { rules, pages })
    }
}

impl Paging {
    fn validate_pages(&self) -> i32 {
        self.pages
            .iter()
            .filter(|pages| {
                for (index, value) in pages.iter().enumerate() {
                    let rules = match self.rules.get(value) {
                        Some(rule) => rule,
                        None => continue,
                    };

                    for rule_value in rules {
                        if pages[index..].contains(rule_value) {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|pages| pages.get(pages.len() / 2).unwrap())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, str::FromStr};

    use super::Paging;

    const TEST_DATA: &str = r#"53|13

75,47,61,53,29"#;

    const TEST_DATA_2: &str = r#"47|53
97|13

97,61,53,29,13
75,29,13"#;

    // #[test]
    fn test1a() {
        let rules = TEST_DATA
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let values = line
                    .split("|")
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<_>>();
                [values[0], values[1]]
            })
            .collect::<Vec<_>>();

        // println!("rules: {:?}", rules);

        let pages = TEST_DATA
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(|line| {
                let values = line
                    .split(",")
                    .map(|num| num.parse().unwrap_or(0))
                    .collect::<Vec<i32>>();
                values
            })
            .collect::<Vec<_>>();

        // println!("pages: {:?}", pages);
    }

    #[test]
    fn test1b() {
        let paging = Paging::from_str(TEST_DATA_2).unwrap();

        println!("rules: {:?}", paging.rules);
        // assert_eq!(paging.rules, vec![[47, 53], [97, 13]]);
        // println!("pages: {:?}", paging.pages);
        assert_eq!(
            paging.pages,
            vec![vec![97, 61, 53, 29, 13], vec![75, 29, 13]]
        );

        // let iter = paging.pages.iter().collect();
        // for page in paging.pages {
        //     page.contains(paging.rules.)
        // }

        // .chunk_by(|a, b| paging.rules.contains())
    }

    #[test]
    fn test2() {
        let file_str = fs::read_to_string("example.txt").unwrap();
        // let file_str = fs::  ("../example.txt");
        // println!("file: {:?}", file_str.lines());

        let paging = Paging::from_str(&file_str).unwrap();
        println!("rules: {:?}", paging.rules);
        println!("pages: {:?}", paging.pages);
    }

    #[test]
    fn test2b() {
        let file_str = fs::read_to_string("example.txt").unwrap();

        let paging = Paging::from_str(&file_str).unwrap();
        // println!("rules: {:?}", paging.rules);
        // println!("pages: {:?}", paging.pages);
        let count = paging.validate_pages();
        println!("count: {:?}", count);
    }
}
