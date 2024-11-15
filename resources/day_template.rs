pub fn part_1(input: &str) -> i32 {
    panic!("Not yet implemented");
}

pub fn part_2(input: &str) -> i32 {
    panic!("Not yet implemented");
}

#[allow(unused_imports)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::shared::test_input::run_tests;

    #[test]
    fn run_all_tests() {
        run_tests(
            2015,
            0,
            part_1,
            part_2,
            vec![("2x3x4", 58), ("1x1x10", 43)],
            vec![("2x3x4", 34), ("1x1x10", 14)],
            0,
            0,
        );
    }
}
