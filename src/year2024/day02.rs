pub fn part_1(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

/* Prompt:
   Create a function to parse a line of input. The input will consist of 5 numbers separated by spaces. The result should be an option of a vector, returning None if the parse fails.
*/
fn parse_line(input: &str) -> Option<Vec<i32>> {
    let numbers: Result<Vec<i32>, _> =
        input.split_whitespace().map(str::parse).collect();
    numbers.ok()
}

generate_tests!(
    2024,
    2,
    part_1,
    part_2,
    vec![
        ("7 6 4 2 1", 1),
        ("1 2 7 8 9", 0),
        ("9 7 6 2 1", 0),
        ("1 3 2 4 5", 0),
        ("8 6 4 4 1", 0),
        ("1 3 6 7 9", 1)
    ], // part 1 examples
    vec![],   // part 2 examples
    Some(-1), // run part 1, expect -1 till we have an answer
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

    // Copilot generated this test case as part of the prompt to create the original function (yay!)
    #[test_case("7 6 4 2 1", Some(vec![7, 6, 4, 2, 1]); "valid input")]
    #[test_case("1 2 7 8 9", Some(vec![1, 2, 7, 8, 9]); "another valid input")]
    #[test_case("invalid input", None; "invalid input")]
    fn parse_line_test(
        input: &str,
        expected: Option<Vec<i32>>,
    ) {
        assert_eq!(parse_line(input), expected);
    }
}
