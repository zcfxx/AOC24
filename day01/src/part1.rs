pub fn parse_data(data: &str) -> Vec<isize> {
    let mut v1 = vec![];
    let mut v2 = vec![];

    for line in data.lines() {
        let mut iter = line.split_ascii_whitespace();

        v1.push(iter.next().unwrap().parse().unwrap_or(0));
        v2.push(iter.next().unwrap().parse().unwrap_or(0));
    }
    v1.sort();
    v2.sort();

    let mut v3 = vec![];
    for i in 0..v1.len() {
        let diff: isize = v2[i] - v1[i];
        v3.push(diff.abs());
    }

    v3
}

pub fn parse_data_v2(data: &str) -> isize {
    let mut v1: Vec<isize> = vec![];
    let mut v2: Vec<isize> = vec![];

    data.lines().for_each(|line| {
        let mut iter = line.split_ascii_whitespace();

        if let Some(v1a) = iter.next() {
            if let Some(v1b) = v1a.parse().ok() {
                v1.push(v1b);
            }
        }

        if let Some(v2a) = iter.next() {
            if let Some(v2b) = v2a.parse().ok() {
                v2.push(v2b);
            }
        }
    });

    v1.sort();
    v2.sort();

    let iter = v1.iter().zip(v2.iter());

    let total: isize = iter.into_iter().map(|v3i| (v3i.1 - v3i.0).abs()).sum();
    println!("total: {total}");

    total
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_parse() {
        let mut v1 = vec![];
        let mut v2 = vec![];

        for line in TEST_DATA.lines() {
            // println!("{line}");
            let mut iter = line.split_ascii_whitespace();
            // println!("{:?}, | {:?}", iter.next(), iter.next());
            v1.push(iter.next().unwrap().parse().unwrap_or(0));
            v2.push(iter.next().unwrap().parse().unwrap_or(0));
        }
        v1.sort();
        v2.sort();
        println!("{:?}", v1);
        println!("{:?}", v2);

        let mut v3 = vec![];

        for i in 0..v1.len() {
            v3.push(v2[i] - v1[i]);
        }

        println!("{:?}", v3);
        let total: isize = v3.iter().sum();

        println!("sum: {:?}", total);
        assert_eq!(total, 11);
    }

    #[test]
    fn test_parse_2() {
        let v3 = parse_data(TEST_DATA);
        let total: isize = v3.iter().sum();

        assert_eq!(total, 11);
    }

    #[test]
    fn test_parse_3() {
        let total = parse_data_v2(TEST_DATA);

        assert_eq!(total, 11);
    }
}
