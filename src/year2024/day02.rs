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

/* Prompt:
   Create a function that takes a vector of integers and validates it. The input is valid if all values are either increasing or decreasing in value and if any two adjacent integers differ by at least 1 and at most 3.
*/
fn validate_vector(numbers: &Vec<i32>) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in numbers.windows(2) {
        let diff = (window[1] - window[0]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if window[1] < window[0] {
            increasing = false;
        }
        if window[1] > window[0] {
            decreasing = false;
        }
    }

    increasing || decreasing
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

    #[test_case(vec![1, 2, 3, 4, 5], true; "valid increasing")]
    #[test_case(vec![5, 4, 3, 2, 1], true; "valid decreasing")]
    #[test_case(vec![1, 3, 5, 7, 9], false; "invalid increasing")]
    #[test_case(vec![9, 7, 5, 3, 1], false; "invalid decreasing")]
    #[test_case(vec![1, 2, 4, 7, 10], false; "invalid difference")]
    fn validate_vector_test(
        input: Vec<i32>,
        expected: bool,
    ) {
        assert_eq!(validate_vector(&input), expected);
    }
}
