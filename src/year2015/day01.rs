pub fn part_1(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

pub fn part_2(input: &str) -> i32 {
    input
        .chars()
        .scan(0, |acc, c| {
            *acc += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            Some(*acc)
        })
        .position(|x| x < 0)
        .unwrap() as i32
        + 1
}

generate_tests!(
    2015,
    1,
    part_1,
    part_2,
    vec![
        ("(())", 0),
        ("()()", 0),
        ("(((", 3),
        ("(()(()(", 3),
        ("))(((((", 3),
        ("())", -1),
        ("))(", -1),
        (")))", -3),
        (")())())", -3),
    ],
    vec![(")", 1), ("()())", 5)],
    232,
    1783
);
