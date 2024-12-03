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

// Part 2 took ~20 minutes
/* Prompt:
    Update the part_2 function to run parse_p2 on its input string. It should then run through the resulting instructions. This is done by creating a bool variable called enabled that will track whether or not to add the result of Mul instructions to the total sum. Whenever a Do instruction is encountered then enabled should be set to true. DoNot should set it to false. A Mul instruction should have its values multiplied and added to the running total sum only if enabled is true at that point in time.
*/
pub fn part_2(input: &str) -> i32 {
    let instructions = parse_p2(input);
    let mut enabled = true; // Copilot guessed wrong on this part, but I didn't provide enough context
    let mut sum = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::DoNot => enabled = false,
            Instruction::Mul(left, right) => {
                if enabled {
                    sum += left * right;
                }
            }
        }
    }

    sum
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

/* This took 2 prompts. The first one produced a function that would not give the
    desired outcome. Was pleasantly surprised that the generated code was able to
    combine the regex values into a single one. Didn't expect that without more guidance.

    Prompt 1:
        Create a parse_p2 function that runs the following regexes against its input string:

        mul\((?<left>\d+),(?<right>\d+)\)

        do\(\)

        don't\(\)

        the result should be a list of Instructions containing each matched value in the order it was found in the original string.

    Prompt 2:
        That implementation will return all Mul instructions first, then all Dos, then all DoNots. I need the original order maintained.
*/
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
    vec![("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", 161)],
    vec![("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", 48)],
    Some(171183089),
    Some(63866497)
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
