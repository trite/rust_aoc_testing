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

#[allow(unused_imports)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::shared::test_input::{run_examples, run_part_against_input};

    const YEAR: i32 = 2015;
    const DAY: i32 = 1;

    #[test]
    fn part_1_examples() {
        run_examples(
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
            part_1,
        );
    }

    #[test]
    fn part_1_input() {
        run_part_against_input(YEAR, DAY, part_1, 232);
    }

    #[test]
    fn part_2_examples() {
        run_examples(vec![(")", 1), ("()())", 5)], part_2);
    }

    #[test]
    fn part_2_input() {
        run_part_against_input(YEAR, DAY, part_2, 1783);
    }
}
