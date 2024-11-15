use std::fs;

pub fn get_input(year: i32, day: i32) -> String {
    let path = format!("src/year{}/day{:02}.txt", year, day);
    fs::read_to_string(&path).expect(&format!("Failed to read from {}", path))
}

pub fn run_part_against_input(year: i32, day: i32, part: fn(&str) -> i32, expected: i32) {
    let input = get_input(year, day);
    let result = part(&input);
    assert_eq!(result, expected);
}

pub fn run_examples(examples: Vec<(&str, i32)>, part: fn(&str) -> i32) {
    examples
        .iter()
        .enumerate()
        .for_each(|(index, (input, expected))| {
            assert_eq!(part(input), *expected, "Test failed for index: {}", index);
        });
}

pub fn run_tests(
    year: i32,
    day: i32,
    part_1: fn(&str) -> i32,
    part_2: fn(&str) -> i32,
    part_1_examples: Vec<(&str, i32)>,
    part_2_examples: Vec<(&str, i32)>,
    part_1_expected: i32,
    part_2_expected: i32,
) {
    run_examples(part_1_examples, part_1);
    run_part_against_input(year, day, part_1, part_1_expected);
    run_examples(part_2_examples, part_2);
    run_part_against_input(year, day, part_2, part_2_expected);
}
