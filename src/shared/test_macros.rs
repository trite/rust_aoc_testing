#[macro_export]
macro_rules! generate_tests {
    (
        $year:expr,
        $day:expr,
        $part_1:ident,
        $part_2:ident,
        $part_1_examples:expr,
        $part_2_examples:expr,
        $part_1_expected:expr,
        $part_2_expected:expr
    ) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::shared::test_input::{run_examples, run_part_against_input};
            use test_case::test_case;

            #[test_case($part_1_examples, $part_1; "p1")]
            #[test_case($part_2_examples, $part_2; "p2")]
            fn examples(examples: Vec<(&str, i32)>, part: fn(&str) -> i32) {
                run_examples(examples, part);
            }

            #[test_case($year, $day, $part_1, $part_1_expected; "p1")]
            #[test_case($year, $day, $part_2, $part_2_expected; "p2")]
            fn actual_input(year: i32, day: i32, part: fn(&str) -> i32, expected: i32) {
                run_part_against_input(year, day, part, expected);
            }
        }
    };
}
