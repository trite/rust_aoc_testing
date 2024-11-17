use std::fs;

fn get_input(
    year: i32,
    day: i32,
) -> String {
    let path = format!("src/year{}/day{:02}.txt", year, day);
    fs::read_to_string(&path).expect(&format!("Failed to read from {}", path))
}

pub fn run_part_against_input<T: PartialEq + std::fmt::Debug>(
    year: i32,
    day: i32,
    part: fn(&str) -> T,
    expected: T,
) {
    let input = get_input(year, day);
    let result = part(&input);
    assert_eq!(result, expected);
}

pub fn run_examples<T: PartialEq + std::fmt::Debug>(
    examples: Vec<(&str, T)>,
    part: fn(&str) -> T,
) {
    examples
        .iter()
        .enumerate()
        .for_each(|(index, (input, expected))| {
            assert_eq!(
                part(input),
                *expected,
                "Test failed for index: {}, example: ({:?}, {:?})",
                index,
                input,
                expected
            );
        });
}
