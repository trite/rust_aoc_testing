fn part_1(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

fn part_2(input: &str) -> i32 {
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

mod tests {
    use super::*;
    use crate::shared::test_input::run_part_against_input;

    const YEAR: i32 = 2015;
    const DAY: i32 = 1;

    // #[test]
    // fn part_1_examples() {
    //     vec![
    //         ("(())", 0),
    //         ("()()", 0),
    //         ("(((", 3),
    //         ("(()(()(", 3),
    //         ("))(((((", 3),
    //         ("())", -1),
    //         ("))(", -1),
    //         (")))", -3),
    //         (")())())", -3),
    //     ]
    //     .iter()
    //     .enumerate()
    //     .for_each(|(index, (input, expected))| {
    //         assert_eq!(part_1(input), *expected, "Test failed for index: {}", index);
    //     });
    // }

    #[test]
    fn part_1_input() {
        run_part_against_input(YEAR, DAY, part_1, 232);
    }

    #[test]
    fn part_2_examples() {
        vec![(")", 1), ("()())", 5)]
            .iter()
            .enumerate()
            .for_each(|(index, (input, expected))| {
                assert_eq!(part_2(input), *expected, "Test failed for index: {}", index);
            });
    }

    #[test]
    fn part_2_input() {
        run_part_against_input(YEAR, DAY, part_2, 1783);
    }
}
