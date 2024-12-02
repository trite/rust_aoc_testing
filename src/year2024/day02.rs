// Part 1 took ~25 minutes (with some distractions)
/* After generating parse_line and validate_vector funcs this is really easy,
   but asked Copilot anyway, here the prompt:
   Modify the part_1 function so that it splits its input into lines, and maps parse_line against each line, then mapping validate_vector on all Some values, and return the count of lines that returned Some(true) at the end.
*/
pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(parse_line)
        .filter(validate_vector)
        .count() as i32
}

// Part 2 took ~10 minutes
// Just copy/pasted and changed the filter function this time around
pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(parse_line)
        .filter(validate_dampened_vector)
        .count() as i32
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

/* Was pleasantly surprised to see Copilot reuse the validate_vector function here
   rather than re-implement the same logic! Prompt:
   Create a function to validate dampened vectors. Similar to the previous validation function the input is valid if all values are either increasing or decreasing in value and if any two adjacent integers differ by at least 1 and at most 3. For this function the result also valid if removing any 1 integer from the vector would cause it to be valid.
*/
fn validate_dampened_vector(numbers: &Vec<i32>) -> bool {
    if validate_vector(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut temp = numbers.clone();
        temp.remove(i);
        if validate_vector(&temp) {
            return true;
        }
    }

    false
}

generate_tests!(
    2024,
    2,
    part_1,
    part_2,
    vec![(
        "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9",
        2
    ),],
    vec![(
        "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9",
        4
    )],
    Some(639),
    Some(674)
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    // Copilot generated these test cases as part of the prompt to create the original function (yay!)
    #[test_case("7 6 4 2 1", Some(vec![7, 6, 4, 2, 1]); "valid input")]
    #[test_case("1 2 7 8 9", Some(vec![1, 2, 7, 8, 9]); "another valid input")]
    #[test_case("invalid input", None; "invalid input")]
    fn parse_line_test(
        input: &str,
        expected: Option<Vec<i32>>,
    ) {
        assert_eq!(parse_line(input), expected);
    }

    // Copilot generated these test cases as well, though 3 of them were incorrect (fixed em)
    // Copilot then deleted these tests when implementing `validate_dampened_vector_test` (fixed that too)
    #[test_case(vec![1, 2, 3, 4, 5], true; "valid increasing")]
    #[test_case(vec![5, 4, 3, 2, 1], true; "valid decreasing")]
    #[test_case(vec![1, 3, 5, 7, 17], false; "invalid increasing")]
    #[test_case(vec![19, 17, 15, 3, 1], false; "invalid decreasing")]
    #[test_case(vec![1, 2, 42, 44, 47], false; "invalid difference")]
    fn validate_vector_test(
        input: Vec<i32>,
        expected: bool,
    ) {
        assert_eq!(validate_vector(&input), expected);
    }

    // Copilot generated these test cases as well, though 1 was wrong (fixing it was still way faster than writing it all by hand)
    #[test_case(vec![1, 2, 3, 4, 5], true; "valid increasing")]
    #[test_case(vec![5, 4, 3, 2, 1], true; "valid decreasing")]
    #[test_case(vec![1, 3, 5, 12, 17], false; "invalid increasing")]
    #[test_case(vec![19, 17, 15, 3, 1], false; "invalid decreasing")]
    #[test_case(vec![1, 2, 42, 44, 47], false; "invalid difference")]
    #[test_case(vec![1, 2, 3, 5, 7], true; "valid with one removal")]
    #[test_case(vec![5, 3, 2, 1, 0], true; "valid with one removal 2")]
    fn validate_dampened_vector_test(
        input: Vec<i32>,
        expected: bool,
    ) {
        assert_eq!(validate_dampened_vector(&input), expected);
    }
}
