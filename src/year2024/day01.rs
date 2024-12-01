// Pt 1 solved in 8 minutes
/* Prompt:
    Modify the part_1 function so that it creates 2 empty vectors (left and right). It should then split input into lines and map parse_line against each line. The left vector should be all the left values from the tuple, and the right vector should be all the right values from the tuple. The vectors should then be sorted, and the difference between each pair of values in the left and right vectors should be summed up to produce the result.
*/
pub fn part_1(input: &str) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if let Some((l, r)) = parse_line(line) {
            left.push(l);
            right.push(r);
        }
    }

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

// Pt 2 solved in 4 minutes
/* Prompt:
    Modify the part_2 function so that it iterates over every value in the left vector. For each value it should determine how many times that value exists in the right vector, and then multiply the value by that count. The result is the sum.
*/
pub fn part_2(input: &str) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if let Some((l, r)) = parse_line(line) {
            left.push(l);
            right.push(r);
        }
    }

    left.iter()
        .map(|&l| l * right.iter().filter(|&&r| r == l).count() as i32)
        .sum()
}

/* Prompt:
    Create a function to parse a line of input. The line will contain 2 nubers separated by multiple spaces. The ouput for the line should be an option of a tuple of the 2 values.
*/
fn parse_line(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
        if let (Ok(a), Ok(b)) =
            (parts[0].parse::<i32>(), parts[1].parse::<i32>())
        {
            return Some((a, b));
        }
    }
    None
}

generate_tests!(
    2024,
    1,
    part_1,
    part_2,
    vec![("3   4\n4   3\n2   5\n1   3\n3   9\n3   3", 11)], // part 1 examples
    vec![("3   4\n4   3\n2   5\n1   3\n3   9\n3   3", 31)], // part 2 examples
    Some(1579939),  // run part 1, expect -1 till we have an answer
    Some(20351745)  // don't run part 2 until we're ready
);
