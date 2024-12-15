#[derive(Debug, PartialEq, Eq)]
enum Report {
    Safe,
    Unsafe,
}

fn is_inc(data: &Vec<i32>) -> bool {
    data.first() < data.last()
}

pub fn if_gradual(data: &Vec<i32>) -> bool {
    let mut result: usize = 0;
    let output = data.chunk_by(|a, b| {
        let diff = b - a;
        match diff.abs() {
            1 | 2 | 3 => {
                result += 1;
                true
            }
            _ => false,
        }
    });

    let _ = output.into_iter().count();

    if result == data.len() - 1 {
        return true;
    }

    false
}

pub fn if_linear(data: &Vec<i32>) -> bool {
    let is_inc = is_inc(data);

    let output = data.chunk_by(|a, b| {
        if is_inc {
            a < b && a != b
        } else {
            b < a && b != a
        }
    });

    let count = output.into_iter().count();
    // println!("linear_count: {count}");

    count < 2
}

fn parse_data(data: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();

    for line in data.lines() {
        let parsed = line.split_whitespace();

        let result: Vec<i32> = parsed.into_iter().map(|v| v.parse().unwrap()).collect();

        if if_gradual(&result) && if_linear(&result) {
            reports.push(Report::Safe);
        } else {
            reports.push(Report::Unsafe);
        }
    }

    reports
}

fn parse_data_v2(data: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();

    data.lines().for_each(|line| {
        let parsed = line.split_whitespace();

        let result: Vec<i32> = parsed.into_iter().filter_map(|v| v.parse().ok()).collect();

        if if_gradual(&result) && if_linear(&result) {
            reports.push(Report::Safe);
        } else {
            reports.push(Report::Unsafe);
        }
    });

    reports
}

pub fn count_safe(data: &str) -> usize {
    let reports = parse_data(data);

    let output = reports
        .into_iter()
        .filter(|v| *v == Report::Safe)
        .collect::<Vec<_>>();

    output.len()
}

pub fn count_safe_v2(data: &str) -> usize {
    let reports = parse_data_v2(data);

    let output = reports
        .into_iter()
        .filter(|v| *v == Report::Safe)
        .collect::<Vec<_>>();

    output.len()
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    const TEST_DATA: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_1() {
        let mut reports: Vec<Report> = Vec::new();

        for line in TEST_DATA.lines() {
            let mut v1: Vec<Report> = vec![];

            let parsed = line.split_whitespace();

            let result: Vec<i32> = parsed.into_iter().map(|v| v.parse().unwrap()).collect();

            // println!("1: {:?}", result);

            let result2 = if_gradual(&result);

            // println!("2: {:?}", result2);

            let result3 = if_linear(&result);
            // println!("3: {:?}", result3);

            if result2 && result3 {
                reports.push(Report::Safe);
            } else {
                reports.push(Report::Unsafe);
            }
        }

        let output = reports
            .into_iter()
            .filter(|v| *v == Report::Safe)
            .collect::<Vec<_>>();

        println!("output_len: {}", output.len());
        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test2() {
        let result = count_safe(TEST_DATA);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = count_safe_v2(TEST_DATA);
        assert_eq!(result, 2);
    }
}
