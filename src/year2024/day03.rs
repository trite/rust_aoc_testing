use regex::Regex;

/* Prompt:
    Modify the part_1 function so that it runs parse on its input string, then multiplies the values in each tuple and sums the results.
*/
pub fn part_1(input: &str) -> i32 {
    if let Some(pairs) = parse(input) {
        pairs.iter().map(|(left, right)| left * right).sum()
    } else {
        0
    }
}

pub fn part_2(_input: &str) -> i32 {
    panic!("Not yet implemented");
}

/* Prompt:
    Create a parse function that takes a string as input. The function should find all instances that match this regex:

    mul((?<left>d+),(?<right>\d+))

    and return them all as tuples matching left to left and right to right. The result should be an option of a vector of tuples of ints.
*/
pub fn parse(input: &str) -> Option<Vec<(i32, i32)>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = Vec::new();

    for cap in re.captures_iter(input) {
        let left = cap[1].parse::<i32>().ok()?;
        let right = cap[2].parse::<i32>().ok()?;
        results.push((left, right));
    }

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

generate_tests!(
    2024,
    3,
    part_1,
    part_2,
    vec![("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", 161)], // part 1 examples
    vec![],         // part 2 examples
    Some(171183089),       // run part 1, expect -1 till we have an answer
    None            // don't run part 2 until we're ready
);

#[cfg(test)]
mod local_tests {
    use super::*;
    use test_case::test_case;

    #[test_case("mul(2,4)mul(3,7)", Some(vec![(2, 4), (3, 7)]); "multiple matches")]
    #[test_case("no matches here", None; "no matches")]
    #[test_case("mul(10,20)", Some(vec![(10, 20)]); "single match")]
    fn parse_test(
        input: &str,
        expected: Option<Vec<(i32, i32)>>,
    ) {
        assert_eq!(parse(input), expected);
    }
}
