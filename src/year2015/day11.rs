pub fn part_1(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

generate_tests!(
    2015,           // year
    11,             // day
    part_1,         // part 1 function
    part_2,         // part 2 function
    vec![("", -1)], // part 1 examples
    vec![],         // part 2 examples
    Some(-1), // run part 1, expect -1 since we don't know the right answer yet
    None      // don't run part 2 until we're ready
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
