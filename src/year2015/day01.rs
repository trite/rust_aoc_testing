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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::test_input::{get_input, run_tests};
    use test_case::test_case;

    #[test]
    fn run_all_tests() {
        run_tests(
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
            1783,
        );
    }

    #[test_case("(())", 0; "example 1")]
    #[test_case("()()", 0; "example 2")]
    #[test_case("(((", 3; "example 3")]
    #[test_case("(()(()(", 3; "example 4")]
    #[test_case("))(((((", 3; "example 5")]
    #[test_case("())", -1; "example 6")]
    #[test_case("))(", -1; "example 7")]
    #[test_case(")))", -3; "example 8")]
    #[test_case(")())())", -3; "example 9")]
    fn part_1_examples(input: &str, expected: i32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case((2015, 1) => 232)]
    fn part_1_input((year, day): (i32, i32)) -> i32 {
        part_1(&get_input(year, day))
    }

    #[test_case(")", 1; "example 1")]
    #[test_case("()())", 5; "example 2")]
    fn part_2_examples(input: &str, expected: i32) {
        assert_eq!(part_2(input), expected);
    }

    #[test_case((2015, 1) => 1783)]
    fn part_2_input((year, day): (i32, i32)) -> i32 {
        part_2(&get_input(year, day))
    }
}
