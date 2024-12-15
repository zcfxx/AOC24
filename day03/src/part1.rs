use regex::Regex;

pub fn parse_regex(data: &str) -> Vec<i32> {
    let muls = Regex::new(r"mul[\(]\d{1,3},\d{1,3}[\)]")
        .unwrap()
        .find_iter(data)
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let mut store = vec![];
    for value in muls.iter() {
        let rmul = value.replace("mul", "");
        let rparen = rmul.replace("(", "");
        let lparen = rparen.replace(")", "");

        let values = lparen.split(",").collect::<Vec<_>>();

        store.push(
            values.first().unwrap().parse().unwrap_or(0)
                * values.last().unwrap().parse().unwrap_or(0),
        );
    }

    store
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::parse_regex;

    const TEST_DATA: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    #[test]
    fn test1() {
        let muls = Regex::new(r"mul[\(]\d{1,2},\d{1,2}[\)]")
            .unwrap()
            .find_iter(&TEST_DATA)
            .map(|v| v.as_str())
            .collect::<Vec<_>>();

        println!("{:?}", muls);

        let mut store = vec![];
        for value in muls.iter() {
            let rmul = value.replace("mul", "");
            let rparen = rmul.replace("(", "");
            let lparen = rparen.replace(")", "");

            let values = lparen.split(",").collect::<Vec<_>>();

            println!("{:?} | {:?}", values.first(), values.last());

            store.push(
                values.first().unwrap().parse().unwrap_or(0)
                    * values.last().unwrap().parse().unwrap_or(0),
            );
        }

        let total: i32 = store.iter().sum();

        println!("total: {total}");
        assert_eq!(total, 161)
    }

    #[test]
    fn test2() {
        let result = parse_regex(TEST_DATA);

        let total: i32 = result.iter().sum();

        assert_eq!(total, 161)
    }
}
