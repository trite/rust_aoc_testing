pub fn part_1(input: &str) -> i32 {
    let mut current = input.to_string();
    for _ in 0..40 {
        let mut next = String::new();
        let mut chars = current.chars().peekable();
        while let Some(c) = chars.next() {
            let mut count = 1;
            while let Some(&next_char) = chars.peek() {
                if next_char == c {
                    chars.next();
                    count += 1;
                } else {
                    break;
                }
            }
            next.push_str(&format!("{}{}", count, c));
        }
        current = next;
    }
    current.len() as i32
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

generate_tests!(
    2015,
    10,
    part_1,
    part_2,
    vec![("1", 2), ("11", 2), ("21", 4), ("1211", 6), ("111221", 6)],
    vec![],
    Some(-1),
    None
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    #[test_case("some input", -1; "example test case")]
    fn parse_test(
        input: &str,
        expected: i32,
    ) {
        assert_eq!(part_1(input), expected);
    }
}
