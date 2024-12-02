pub fn part_1(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

generate_tests!(
    2024,
    2,
    part_1,
    part_2,
    vec![("", -1)], // part 1 examples
    vec![],         // part 2 examples
    Some(-1),       // run part 1, expect -1 till we have an answer
    None            // don't run part 2 until we're ready
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
