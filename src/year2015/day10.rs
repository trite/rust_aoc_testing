pub fn cycle(input: &str) -> String {
    let current = input.to_string();
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
    next
}

pub fn cycle_n(
    input: &str,
    n: usize,
) -> String {
    let mut current = input.to_string();
    for _ in 0..n {
        current = cycle(&current);
    }
    current
}

pub fn part_1(input: &str) -> i32 {
    let current = cycle_n(input, 40);
    current.len() as i32
}

pub fn part_2(input: &str) -> i32 {
    let current = cycle_n(input, 50);
    current.len() as i32
}

generate_tests!(
    2015,
    10,
    part_1,
    part_2,
    vec![],
    vec![],
    Some(492982),
    Some(6989950)
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1", 2; "cycle test 1")]
    #[test_case("11", 2; "cycle test 2")]
    #[test_case("21", 4; "cycle test 3")]
    #[test_case("1211", 6; "cycle test 4")]
    #[test_case("111221", 6; "cycle test 5")]
    fn cycle_test(
        input: &str,
        expected: i32,
    ) {
        assert_eq!(cycle(input).len() as i32, expected);
    }
}
