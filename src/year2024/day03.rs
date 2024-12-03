use regex::Regex;

/* Prompt:
    Create a variant type with 3 possible states: Do, DoNot, and Mul(left, right). The Do and DoNot states require no additional information, the Mul state should hold 2 integer values.
*/
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Do,
    DoNot,
    Mul(i32, i32),
}

// Part 1 took ~15 minutes
/* Prompt:
    Modify the part_1 function so that it runs parse on its input string, then multiplies the values in each tuple and sums the results.
*/
pub fn part_1(input: &str) -> i32 {
    if let Some(pairs) = parse_p1(input) {
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

    mul\((?<left>\d+),(?<right>\d+)\)

    and return them all as tuples matching left to left and right to right. The result should be an option of a vector of tuples of ints.
*/
pub fn parse_p1(input: &str) -> Option<Vec<(i32, i32)>> {
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

pub fn parse_p2(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut instructions = Vec::new();

    for cap in re.captures_iter(input) {
        if let Some(left) = cap.get(1) {
            let left = left.as_str().parse::<i32>().unwrap();
            let right = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            instructions.push(Instruction::Mul(left, right));
        } else if cap.get(0).unwrap().as_str() == "do()" {
            instructions.push(Instruction::Do);
        } else if cap.get(0).unwrap().as_str() == "don't()" {
            instructions.push(Instruction::DoNot);
        }
    }

    instructions
}

generate_tests!(
    2024,
    3,
    part_1,
    part_2,
    vec![("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", 161)], // part 1 examples
    vec![("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", 48)],         // part 2 examples
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
    fn parse_p1_test(
        input: &str,
        expected: Option<Vec<(i32, i32)>>,
    ) {
        assert_eq!(parse_p1(input), expected);
    }

    #[test_case("mul(2,4)do()mul(3,7)don't()", vec![Instruction::Mul(2, 4), Instruction::Do, Instruction::Mul(3, 7), Instruction::DoNot]; "multiple matches")]
    #[test_case("do()don't()", vec![Instruction::Do, Instruction::DoNot]; "do and don't matches")]
    #[test_case("mul(10,20)", vec![Instruction::Mul(10, 20)]; "single mul match")]
    fn parse_p2_test(
        input: &str,
        expected: Vec<Instruction>,
    ) {
        assert_eq!(parse_p2(input), expected);
    }
}
